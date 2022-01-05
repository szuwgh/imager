use crate::common::error::Error;
use anyhow::bail;
use anyhow::Result;
use nix::{
    sys::{socket, uio},
    unistd::{self, close, read, write},
};

use serde::Serialize;
use std::marker::PhantomData;
use std::os::unix::prelude::RawFd;
use std::path::Path;
use std::path::PathBuf;

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

pub fn new<T>() -> Result<(Writer<T>, Reader<T>)>
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

pub struct NotifyListener {
    fd: RawFd,
    socket_path: PathBuf,
}

impl NotifyListener {
    pub fn new(socket_path: &Path) -> Result<NotifyListener> {
        let raw_fd = socket::socket(
            socket::AddressFamily::Unix,
            socket::SockType::SeqPacket,
            socket::SockFlag::SOCK_CLOEXEC,
            None,
        )?;
        let sockaddr = socket::SockAddr::new_unix(socket_path)?;
        socket::bind(raw_fd, &sockaddr)?;
        socket::listen(raw_fd, 10)?;
        Ok(NotifyListener {
            fd: raw_fd,
            socket_path: socket_path.to_path_buf(),
        })
    }

    //
    pub fn wait_container_start(&self) -> Result<()> {
        let socket_fd = socket::accept(self.fd)?;
        let mut buf = [0u8; 4];
        let _ = read(socket_fd, &mut buf)?;
        let mut buf1 = vec![0u8; read_u32(&buf) as usize];
        let _ = read(socket_fd, &mut buf1)?;
        let cmd = String::from_utf8(buf1)?;
        println!("cmd:{}", cmd);
        if cmd != "start" {
            bail!("not start")
        }
        Ok(())
    }

    pub fn close(&self) -> Result<()> {
        close(self.fd)?;
        std::fs::remove_file(&self.socket_path)?;
        Ok(())
    }
}

pub struct NotifySocket {
    fd: RawFd,
}

impl NotifySocket {
    pub fn new(socket_path: &Path) -> Result<NotifySocket> {
        let raw_fd = socket::socket(
            socket::AddressFamily::Unix,
            socket::SockType::SeqPacket,
            socket::SockFlag::SOCK_CLOEXEC,
            None,
        )?;
        let sockaddr = socket::SockAddr::new_unix(Path::new(socket_path))?;
        socket::connect(raw_fd, &sockaddr)?;
        Ok(NotifySocket { fd: raw_fd })
    }

    pub fn notify(&self, msg: &str) -> Result<()> {
        let mut buf = [0u8; 4];
        put_uint32(&mut buf, msg.len() as u32);
        write(self.fd, &buf)?;
        write(self.fd, msg.as_bytes())?;
        Ok(())
    }

    pub fn close(&self) -> Result<()> {
        Ok(close(self.fd)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ipc() {
        let (w, r) = new::<String>().unwrap();
        let _ = std::thread::spawn(move || {
            std::thread::sleep_ms(1000);
            w.write("aa".to_owned()).unwrap();
            w.close().unwrap();
        });

        let s = r.read().unwrap();
        r.close().unwrap();
        println!("{}", s);
    }

    #[test]
    fn test_notifysocket() {
        let socket_path = Path::new("/opt/test.sock");
        let mut notify_listener = NotifyListener::new(socket_path).unwrap();
        let _ = std::thread::spawn(move || {
            std::thread::sleep_ms(1000);
            let notify_socket = NotifySocket::new(socket_path).unwrap();
            notify_socket.notify("start").unwrap();
            notify_socket.close().unwrap();
        });
        notify_listener.wait_container_start().unwrap();
        notify_listener.close().unwrap();
    }
}
