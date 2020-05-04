use std::fmt;
use std::fmt::{Display, Formatter};
use std::io;

/// Error type for kvs.
#[derive(Debug)]
pub enum Error {
    /// error by bincode
    BincodeError(bincode::Error),
    /// error by io
    IoError(io::Error),
    /// key not found
    KeyNotFound,
}

impl From<bincode::Error> for Error {
    fn from(e: bincode::Error) -> Self {
        Error::BincodeError(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IoError(e)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "KVError")
    }
}

impl std::error::Error for Error {}
