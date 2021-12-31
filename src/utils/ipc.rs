use crate::common::error::Error;
use anyhow::bail;
use anyhow::Result;
use nix::{
    sys::{socket, uio},
    unistd::{close, read, write},
};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use std::os::unix::prelude::RawFd;
use std::path::Path;

fn put_uint32(dst: &mut [u8], n: u32) {
    let _ = dst[3];
    dst[0] = (n >> 24) as u8;
    dst[1] = (n >> 16) as u8;
    dst[2] = (n >> 8) as u8;
    dst[3] = n as u8;
}

fn read_u32(src: &[u8]) -> u32 {
    let _ = src[3];
    src[3] as u32 | (src[2] as u32) << 8 | (src[1] as u32) << 16 | (src[0] as u32) << 24
}

pub struct Writer<T>
where
    T: Serialize,
{
    fd: RawFd,
    phantom: PhantomData<T>,
}

impl<T> Writer<T>
where
    T: Serialize,
{
    pub fn write(&self, object: T) -> Result<()> {
        let payload = serde_json::to_vec(&object)?;
        let mut buf = [0u8; 4];
        put_uint32(&mut buf, payload.len() as u32);
        write(self.fd, &buf)?;
        write(self.fd, &payload)?;
        Ok(())
    }

    pub fn close(&self) -> Result<()> {
        Ok(close(self.fd)?)
    }
}

pub struct Reader<T>
where
    T: serde::de::DeserializeOwned,
{
    fd: RawFd,
    phantom: PhantomData<T>,
}

impl<T> Reader<T>
where
    T: serde::de::DeserializeOwned,
{
    pub fn peek(&self) -> Result<usize> {
        let mut buf = [0u8; 4];
        let _ = read(self.fd, &mut buf)?;
        Ok(read_u32(&buf) as usize)
    }

    pub fn read(&self) -> Result<T> {
        let size = self.peek()?;
        let mut buf = vec![0u8; size as usize];
        let num = read(self.fd, &mut buf)?;
        Ok(serde_json::from_slice(&buf[..num])?)
    }

    pub fn close(&self) -> Result<()> {
        Ok(close(self.fd)?)
    }
}

pub fn channel<T>() -> Result<(Writer<T>, Reader<T>)>
where
    T: serde::de::DeserializeOwned + Serialize,
{
    let (w_fd, r_fd) = socket::socketpair(
        socket::AddressFamily::Unix,
        socket::SockType::SeqPacket,
        None,
        socket::SockFlag::SOCK_CLOEXEC,
    )?;
    Ok((
        Writer {
            fd: w_fd,
            phantom: PhantomData,
        },
        Reader {
            fd: r_fd,
            phantom: PhantomData,
        },
    ))
}

struct NotityLister {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ipc() {
        let (w, r) = channel::<String>().unwrap();
        let _ = std::thread::spawn(move || {
            std::thread::sleep_ms(1000);
            w.write("aa".to_owned()).unwrap();
            w.close().unwrap();
        });

        let s = r.read().unwrap();
        r.close().unwrap();
        println!("{}", s);
    }
}
