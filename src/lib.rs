mod auth;
mod error;
pub mod models;
use reqwest;

pub use error::{Error, Result};

const API_BASE_URL: &str = "https://api.adsabs.harvard.edu/v1";

pub struct Client {
    base_url: reqwest::Url,
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> Result<Client> {
        if let Some(token) = auth::get_token() {
            Client::new_with_token(&token)
        } else {
            Err(Error::token())
        }
    }

    pub fn new_with_token(token: &str) -> Result<Client> {
        let mut hmap = reqwest::header::HeaderMap::new();
        hmap.append(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", token).parse().unwrap(),
        );
        let client = reqwest::Client::builder()
            .user_agent("adsabs-rust")
            .default_headers(hmap)
            .build()?;
        Ok(Client {
            base_url: reqwest::Url::parse(API_BASE_URL).unwrap(),
            client: client,
        })
    }

    pub async fn get<P>(&self, path: P)
    where
        P: AsRef<str>,
    {
        let url = self.base_url.join(path.as_ref()).unwrap();
        let mut request = self.client.get(url);
    }
}
