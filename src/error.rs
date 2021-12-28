use std::io;
use std::result;

pub type Result<T, E = AdsError> = result::Result<T, E>;

#[allow(clippy::module_name_repetitions)]
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum AdsError {
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
