//! # adsabs
//!
//! A Rust client for the SAO/NASA Astrophysics Data System API.
//!
//! ## Usage
//!
//! To use `adsabs` as a library, add it as a dependency in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! adsabs = "0.1"
//! ```
//!
//! For now, only the `/search` endpoint is supported, as described below. Other
//! endpoints could be manually accessed using [`Ads::get`] directly, and pull
//! requests would be welcome!
//!
//! ## Examples
//!
//! To search for highly cited supernova papers, something like the following
//! should do the trick:
//!
//! ```no_run
//! # fn doc() -> adsabs::Result<()> {
//! use adsabs::prelude::*;
//!
//! let client = Ads::new("ADS_API_TOKEN")?;
//! for doc in client.search("supernova")
//!     .sort("citation_count")
//!     .iter()
//!     .limit(5)
//! {
//!     println!("{:?}", doc?);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! Don't forget to replace `ADS_API_TOKEN` with your [ADS settings page], or
//! using another method as described in the [API token](#api-token) section
//! below.
//!
//! The `query` parameter passed to [`Ads::search`] supports all the usual ADS
//! search syntax. So, for example, if you want to search for papers by a
//! particular first author, use something like the following:
//!
//! ```no_run
//! # fn doc() -> adsabs::Result<()> {
//! use adsabs::prelude::*;
//!
//! let client = Ads::new("ADS_API_TOKEN")?;
//! for doc in client.search("author:\"^Dalcanton, J\"").iter().limit(5) {
//!     println!("{:?}", doc?);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! You can find executable examples of these and other sample usage in [the
//! `examples` directory of the repository on
//! GitHub](https://github.com/dfm/adsabs-rs/tree/main/examples).
//!
//! ## API token
//!
//! All queries to the ADS API must be authenticated with your API token from
//! the [ADS settings page]. You can pass your token as a string directly to the
//! client:
//!
//! ```rust
//! # fn doc() -> adsabs::Result<()> {
//! # use adsabs::prelude::*;
//! let client = Ads::new("ADS_API_TOKEN")?;
//! # Ok(())
//! # }
//! ```
//!
//! Or you can load the token automatically from your environment using
//! [`AdsBuilder::from_env`]:
//!
//! ```no_run
//! # fn doc() -> adsabs::Result<()> {
//! # use adsabs::prelude::*;
//! let client = Ads::from_env()?;
//! # Ok(())
//! # }
//! ```
//!
//! In this case, the following locations are checked, in the listed order:
//!
//! 1. The `ADS_API_TOKEN` environment variable,
//! 2. The `ADS_DEV_KEY` environment variable,
//! 3. The contents of the `~/.ads/token` file, and
//! 4. The contents of the `~/.ads/dev_key` file.
//!
//! Where these were chosen to be compatible with the locations supported by the
//! Python client `ads`.
//!
//! [ADS settings page]: https://ui.adsabs.harvard.edu/user/settings/token

mod auth;
mod endpoints;
mod error;
mod model;

pub use endpoints::{export, search, Sort};
pub use error::{AdsError, Result};
pub use model::Document;

use reqwest::header;

pub mod prelude {
    pub use crate::{export::FormatType, Ads, AdsError, Document, Sort};
}

const API_BASE_URL: &str = "https://api.adsabs.harvard.edu/v1/";

/// An interface to the NASA ADS API.
///
/// This has various configuration values to tweak, but the most important one
/// is `token`, which you'll want to set to your ADS API token, which is
/// available on your [ADS settings page]. To configure your `Ads` interface,
/// use [`Ads::builder`].
///
/// [ADS settings page]: https://ui.adsabs.harvard.edu/user/settings/token
///
/// # Examples
///
/// ```rust
/// # fn doc() -> adsabs::Result<()> {
/// use adsabs::Ads;
/// let api_token = "ADS_API_TOKEN";
/// let client = Ads::new(api_token)?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct Ads {
    base_url: reqwest::Url,
    #[cfg(feature = "blocking")]
    blocking_client: std::rc::Rc<reqwest::blocking::Client>,
    #[cfg(feature = "async")]
    async_client: std::rc::Rc<reqwest::Client>,
}

/// A builder that can be used to create an [`Ads`] interface with custom
/// settings.
///
/// # Example
///
/// ```rust
/// # fn run() -> adsabs::Result<()> {
/// use adsabs::Ads;
/// let api_token = "ADS_API_TOKEN";
/// let client = Ads::builder(api_token)
///     .user_agent("my-user-agent")
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[must_use]
pub struct AdsBuilder {
    base_url: String,
    token: String,
    user_agent: String,
}

impl AdsBuilder {
    /// Constructs a new `AdsBuilder`.
    ///
    /// This is the same as [`Ads::builder`].
    pub fn new(token: &str) -> Self {
        Self {
            base_url: API_BASE_URL.to_owned(),
            token: token.to_owned(),
            user_agent: format!("adsabs-rs/{}", env!("CARGO_PKG_VERSION")),
        }
    }

    /// Constructs a new `AdsBuilder`, loading the API token from either
    /// environment valiables or the user's home directory.
    ///
    /// The following locations are checked, in the listed order:
    ///
    /// 1. The `ADS_API_TOKEN` environment variable,
    /// 2. The `ADS_DEV_KEY` environment variable,
    /// 3. The contents of the `~/.ads/token` file, and
    /// 4. The contents of the `~/.ads/dev_key` file.
    ///
    /// These were chosen to be compatible with the locations supported by the
    /// Python client `ads`.
    ///
    /// # Errors
    ///
    /// This method fails when the token cannot be loaded from any of the
    /// expected locations.
    pub fn from_env() -> Result<Self> {
        Ok(Self::new(&auth::get_token()?))
    }

    /// Sets the base API URL to be used by this client.
    pub fn base_url(mut self, url: &str) -> Self {
        self.base_url = url.to_owned();
        self
    }

    /// Sets the API token to be used by this client.
    pub fn token(mut self, token: &str) -> Self {
        self.token = token.to_owned();
        self
    }

    /// Sets the `User-Agent` header to be used by this client.
    pub fn user_agent(mut self, user_agent: &str) -> Self {
        self.user_agent = user_agent.to_owned();
        self
    }

    /// Build the `Ads` API client
    ///
    /// # Errors
    ///
    /// This method fails when there are problems parsing any of the parameters
    /// into the right formats for `reqwest`.
    pub fn build(self) -> Result<Ads> {
        let mut auth_value: header::HeaderValue = format!("Bearer {}", self.token).parse()?;
        auth_value.set_sensitive(true);
        let mut headers = header::HeaderMap::new();
        headers.append(header::AUTHORIZATION, auth_value);

        #[cfg(feature = "blocking")]
        let blocking_client = reqwest::blocking::Client::builder()
            .user_agent(self.user_agent.clone())
            .default_headers(headers.clone())
            .build()?;

        #[cfg(feature = "async")]
        let async_client = reqwest::Client::builder()
            .user_agent(self.user_agent.clone())
            .default_headers(headers.clone())
            .build()?;

        Ok(Ads {
            base_url: reqwest::Url::parse(&self.base_url)?,
            #[cfg(feature = "blocking")]
            blocking_client: std::rc::Rc::new(blocking_client),
            #[cfg(feature = "async")]
            async_client: std::rc::Rc::new(async_client),
        })
    }
}

impl Ads {
    /// Get an API client with a given token.
    ///
    /// # Errors
    ///
    /// This method fails when [`AdsBuilder::build`] fails.
    pub fn new(token: &str) -> Result<Self> {
        Self::builder(token).build()
    }

    /// Constructs a new `Ads` interface, loading the API token from either
    /// environment valiables or the user's home directory.
    ///
    /// # Errors
    ///
    /// This method fails when either [`AdsBuilder::build`] or
    /// [`AdsBuilder::from_env`] fails.
    pub fn from_env() -> Result<Self> {
        AdsBuilder::from_env()?.build()
    }

    /// Constructs a new [`AdsBuilder`] so that the parameters of the `Ads`
    /// interface can be customized.
    pub fn builder(token: &str) -> AdsBuilder {
        AdsBuilder::new(token)
    }

    /// Constructs a query for Search API endpoint that can be customized using
    /// a [`search::Search`].
    pub fn search(&self, query: &str) -> search::Search {
        search::Search::new(self, query)
    }

    /// Constructs a query for Export API endpoint that can be customized using
    /// a [`export::Export`].
    pub fn export(&self, format_type: export::FormatType, bibcode: &[String]) -> export::Export {
        export::Export::new(self, format_type, bibcode)
    }

    fn absolute_url(&self, url: &str) -> Result<reqwest::Url> {
        Ok(self.base_url.join(url)?)
    }
}

#[cfg(feature = "blocking")]
impl Ads {
    /// Execute a blocking `GET` request to the API.
    ///
    /// # Errors
    ///
    /// This method fails when the URL cannot be parsed or on HTTP errors.
    pub fn blocking_get<P>(
        &self,
        path: &str,
        parameters: Option<&P>,
    ) -> Result<reqwest::blocking::Response>
    where
        P: serde::Serialize + ?Sized,
    {
        let mut request = self.blocking_client.get(self.absolute_url(path)?);
        if let Some(parameters) = parameters {
            request = request.query(parameters);
        }
        Ok(request.send()?)
    }

    /// Execute a blocking `POST` request to the API.
    ///
    /// # Errors
    ///
    /// This method fails when the URL cannot be parsed or on HTTP errors.
    pub fn blocking_post<P>(
        &self,
        path: &str,
        parameters: Option<&P>,
    ) -> Result<reqwest::blocking::Response>
    where
        P: serde::Serialize + ?Sized,
    {
        let mut request = self.blocking_client.post(self.absolute_url(path)?);
        if let Some(parameters) = parameters {
            request = request.json(parameters);
        }
        Ok(request.send()?)
    }

    /// Execute a blocking `PUT` request to the API.
    ///
    /// # Errors
    ///
    /// This method fails when the URL cannot be parsed or on HTTP errors.
    pub fn blocking_put<P>(
        &self,
        path: &str,
        parameters: Option<&P>,
    ) -> Result<reqwest::blocking::Response>
    where
        P: serde::Serialize + ?Sized,
    {
        let mut request = self.blocking_client.put(self.absolute_url(path)?);
        if let Some(parameters) = parameters {
            request = request.json(parameters);
        }
        Ok(request.send()?)
    }

    /// Execute a blocking `DELETE` request to the API.
    ///
    /// # Errors
    ///
    /// This method fails when the URL cannot be parsed or on HTTP errors.
    pub fn blocking_delete(&self, path: &str) -> Result<reqwest::blocking::Response> {
        let request = self.blocking_client.delete(self.absolute_url(path)?);
        Ok(request.send()?)
    }
}

#[cfg(feature = "async")]
impl Ads {
    /// Execute an async `GET` request to the API.
    ///
    /// # Errors
    ///
    /// This method fails when the URL cannot be parsed or on HTTP errors.
    pub async fn async_get<P>(
        &self,
        path: &str,
        parameters: Option<&P>,
    ) -> Result<reqwest::Response>
    where
        P: serde::Serialize + ?Sized,
    {
        let mut request = self.async_client.get(self.absolute_url(path)?);
        if let Some(parameters) = parameters {
            request = request.query(parameters);
        }
        Ok(request.send().await?)
    }

    /// Execute an async `POST` request to the API.
    ///
    /// # Errors
    ///
    /// This method fails when the URL cannot be parsed or on HTTP errors.
    pub async fn async_post<P>(
        &self,
        path: &str,
        parameters: Option<&P>,
    ) -> Result<reqwest::Response>
    where
        P: serde::Serialize + ?Sized,
    {
        let mut request = self.async_client.post(self.absolute_url(path)?);
        if let Some(parameters) = parameters {
            request = request.json(parameters);
        }
        Ok(request.send().await?)
    }

    /// Execute an async `PUT` request to the API.
    ///
    /// # Errors
    ///
    /// This method fails when the URL cannot be parsed or on HTTP errors.
    pub async fn async_put<P>(
        &self,
        path: &str,
        parameters: Option<&P>,
    ) -> Result<reqwest::Response>
    where
        P: serde::Serialize + ?Sized,
    {
        let mut request = self.async_client.put(self.absolute_url(path)?);
        if let Some(parameters) = parameters {
            request = request.json(parameters);
        }
        Ok(request.send().await?)
    }

    /// Execute an async `DELETE` request to the API.
    ///
    /// # Errors
    ///
    /// This method fails when the URL cannot be parsed or on HTTP errors.
    pub async fn async_delete(&self, path: &str) -> Result<reqwest::Response> {
        let request = self.async_client.delete(self.absolute_url(path)?);
        Ok(request.send().await?)
    }
}
