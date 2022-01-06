use std::{borrow::Cow, io};
pub mod oci;

#[derive(Debug)]
pub enum OciError {
    Error(String),
}

pub fn oci_error<'a, M>(message: M) -> OciError
where
    M: Into<Cow<'a, str>>,
{
    let message = message.into();
    match message {
        Cow::Borrowed(s) => OciError::Error(s.to_owned()),
        Cow::Owned(s) => OciError::Error(s),
    }
}
