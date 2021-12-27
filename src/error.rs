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

    #[error("JSON parse error")]
    Json(#[from] serde_json::Error),

    #[error("")]
    Ads(String),

    #[error("unable to load API token from environment variables or home directory")]
    Token,
}
