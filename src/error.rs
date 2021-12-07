use std::io;
use std::result;

pub type Result<T, E = Error> = result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] io::Error),

    #[error("HTTP error")]
    Reqwest(#[from] reqwest::Error),

    #[error("HTTP header error")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),

    #[error("URL parse error")]
    Url(#[from] url::ParseError),

    #[error("JSON parsing error")]
    JsonError(#[from] serde_json::Error),

    #[error("")]
    AdsError(String),

    #[error("")]
    TokenError,
}
