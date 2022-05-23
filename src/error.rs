//! Crate-level error types and handling.

use std::io::Error as IoError;
use std::num::ParseFloatError;
use std::num::ParseIntError;

/// An error that can occur when working with the VLC interface.
#[derive(Debug)]
pub enum Error {
    /// A standard **I/O** error.
    Io(IoError),
    /// The client failed to parse output received from VLC.
    ParseErr,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::Io(ref e) => e.fmt(f),
            Error::ParseErr => write!(
                f,
                "the client failed to parse the output received from VLC"
            ),
        }
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Error::Io(e)
    }
}

impl From<ParseFloatError> for Error {
    fn from(_: ParseFloatError) -> Self {
        Error::ParseErr
    }
}

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Self {
        Error::ParseErr
    }
}
