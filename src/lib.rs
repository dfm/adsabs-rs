mod auth;
mod error;
pub mod models;
pub mod search;
pub use error::{Error, Result};
pub use search::SortOrder;

const API_BASE_URL: &str = "https://api.adsabs.harvard.edu/v1/";

pub struct ClientBuilder {
    base_url: reqwest::Url,
    token: Option<String>,
    user_agent: String,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            base_url: reqwest::Url::parse(API_BASE_URL).unwrap(),
            token: None,
            user_agent: String::from("adsabs-rust-client"),
        }
    }
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn base_url(mut self, url: impl Into<reqwest::Url>) -> Self {
        self.base_url = url.into();
        self
    }

    pub fn set_token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    pub fn load_token(mut self) -> Self {
        self.token = auth::get_token();
        self
    }

    pub fn build(self) -> Result<Client> {
        let mut hmap = reqwest::header::HeaderMap::new();
        hmap.append(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", self.token.ok_or(Error::TokenError)?).parse()?,
        );
        let client = reqwest::Client::builder()
            .user_agent(self.user_agent)
            .default_headers(hmap)
            .build()?;
        Ok(Client {
            base_url: self.base_url,
            client,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    base_url: reqwest::Url,
    client: reqwest::Client,
}

impl Default for Client {
    fn default() -> Self {
        ClientBuilder::default().load_token().build().unwrap()
    }
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    pub async fn get<A, P>(&self, path: A, parameters: Option<&P>) -> Result<reqwest::Response>
    where
        A: AsRef<str>,
        P: serde::Serialize + ?Sized,
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
        P: serde::Serialize + ?Sized,
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

impl Client {
    pub fn search(&self, query: impl Into<String>) -> search::SearchBuilder {
        search::SearchBuilder::new(self, query)
    }
}
