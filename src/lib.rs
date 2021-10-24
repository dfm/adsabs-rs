mod auth;
mod error;
pub mod models;
pub use error::{Error, Result};

use reqwest;
use serde::Serialize;

const API_BASE_URL: &str = "https://api.adsabs.harvard.edu/v1/";

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

    pub async fn get<A, P>(&self, path: A, parameters: Option<&P>) -> Result<reqwest::Response>
    where
        A: AsRef<str>,
        P: Serialize + ?Sized,
    {
        self._get(self.absolute_url(path).unwrap(), parameters)
            .await
    }

    pub async fn _get<P>(
        &self,
        url: impl reqwest::IntoUrl,
        parameters: Option<&P>,
    ) -> Result<reqwest::Response>
    where
        P: Serialize + ?Sized,
    {
        let mut request = self.client.get(url);
        if let Some(parameters) = parameters {
            request = request.query(parameters);
        }
        Ok(request.send().await?)
    }

    pub fn absolute_url(&self, url: impl AsRef<str>) -> Result<reqwest::Url> {
        Ok(self.base_url.join(url.as_ref())?)
    }
}
