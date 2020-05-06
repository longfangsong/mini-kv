use failure::Fail;
use std::io;
use std::string::FromUtf8Error;

/// Error type for kvs.
#[derive(Fail, Debug)]
pub enum Error {
    /// error by bincode
    #[fail(display = "bincode error: {}", _0)]
    BincodeError(bincode::Error),
    /// error by io
    #[fail(display = "IO error: {}", _0)]
    IoError(#[cause] io::Error),
    /// utf-8 converting error
    #[fail(display = "UTF-8 error: {}", _0)]
    Utf8(FromUtf8Error),
    /// error from sled
    #[fail(display = "sled error: {}", _0)]
    Sled(sled::Error),
    /// key not found
    #[fail(display = "Key not found")]
    KeyNotFound,
    /// string error
    #[fail(display = "{}", _0)]
    StringError(String),
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

impl From<FromUtf8Error> for Error {
    fn from(e: FromUtf8Error) -> Self {
        Error::Utf8(e)
    }
}

impl From<sled::Error> for Error {
    fn from(e: sled::Error) -> Self {
        Error::Sled(e)
    }
}
