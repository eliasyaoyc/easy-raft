use std::fmt::Display;
use serde::export::Formatter;
use std::fmt;

pub type IResult<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    err: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    NotFound,
    NotLeader,
    InvalidRequest,
}

impl ErrorKind {
    pub fn as_str(&self) -> &'static str {
        match *self {
            ErrorKind::NotFound => "key not found",
            ErrorKind::NotLeader => "leader not found",
            ErrorKind::InvalidRequest => "request is invalid",
        }
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<ErrorKind> for Error {
    fn from(e: ErrorKind) -> Self {
        Error {
            err: e,
        }
    }
}