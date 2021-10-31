use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::result;

pub type Result<T, E = Error> = result::Result<T, E>;

#[derive(Debug)]
pub struct Error(Box<ErrorKind>);

impl Error {
    pub fn new(kind: ErrorKind) -> Error {
        Error(Box::new(kind))
    }

    pub fn token() -> Error {
        Error(Box::new(ErrorKind::TokenError))
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorKind {
    Io(io::Error),
    Reqwest(reqwest::Error),
    InvalidHeaderValue(reqwest::header::InvalidHeaderValue),
    Url(url::ParseError),
    JsonError(serde_json::Error),
    AdsError(String),
    TokenError,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::new(ErrorKind::Io(err))
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::new(ErrorKind::Reqwest(err))
    }
}

impl From<reqwest::header::InvalidHeaderValue> for Error {
    fn from(err: reqwest::header::InvalidHeaderValue) -> Error {
        Error::new(ErrorKind::InvalidHeaderValue(err))
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Error {
        Error::new(ErrorKind::Url(err))
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::new(ErrorKind::JsonError(err))
    }
}

impl From<String> for Error {
    fn from(msg: String) -> Error {
        Error::new(ErrorKind::AdsError(msg))
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self.0 {
            ErrorKind::Io(ref err) => Some(err),
            ErrorKind::Reqwest(ref err) => Some(err),
            ErrorKind::InvalidHeaderValue(ref err) => Some(err),
            ErrorKind::Url(ref err) => Some(err),
            ErrorKind::JsonError(ref err) => Some(err),
            ErrorKind::AdsError(_) => None,
            ErrorKind::TokenError => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            ErrorKind::Io(ref err) => err.fmt(f),
            ErrorKind::Reqwest(ref err) => err.fmt(f),
            ErrorKind::InvalidHeaderValue(ref err) => err.fmt(f),
            ErrorKind::Url(ref err) => err.fmt(f),
            ErrorKind::JsonError(ref err) => err.fmt(f),
            ErrorKind::AdsError(ref msg) => f.write_str(msg),
            ErrorKind::TokenError => f.write_str("could not find API token"),
        }
    }
}
