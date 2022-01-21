use std::str::FromStr;

use super::{comma_separated, Sort};
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)]
#[must_use]
pub struct Export<'ads> {
    #[serde(skip)]
    client: &'ads crate::Ads,
    #[serde(skip)]
    format_type: FormatType,
    bibcode: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(serialize_with = "comma_separated")]
    sort: Vec<Sort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[must_use]
#[non_exhaustive]
#[serde(rename_all = "lowercase")]
pub enum FormatType {
    BibTeX,
    BibTeXABS,
    ADS,
    EndNote,
    ProCite,
    RIS,
    RefWorks,
    RSS,
    MEDLARS,
    DCXML,
    REFXML,
    REFABSXML,
    AASTeX,
    Icarus,
    MNRAS,
    SoPh,
    VOTable,
    Custom,
}

impl FromStr for FormatType {
    type Err = crate::AdsError;
    fn from_str(s: &str) -> Result<Self> {
        Ok(serde_json::from_str(&format!("\"{}\"", s))?)
    }
}

/// A response from the export API.
#[derive(Serialize, Deserialize, Clone)]
pub struct Response {
    pub msg: String,
    pub export: String,
}

impl<'ads> Export<'ads> {
    pub fn new(client: &'ads crate::Ads, format_type: FormatType, bibcode: &[String]) -> Self {
        Self {
            client,
            format_type,
            bibcode: bibcode.into(),
            sort: Vec::new(),
            format: None,
        }
    }

    /// The sorting field and direction to be used when returning results.
    ///
    /// The `field` argument should be a valid field name. The default sort
    /// method is the relevancy score as calculated by the search engine. Other
    /// useful fields to sort on may be `date`, `read_count`, `first_author`, or
    /// `bibcode`.
    pub fn sort<T: Into<Sort>>(mut self, field: T) -> Self {
        self.sort.push(field.into());
        self
    }

    /// Custom format string for the output.
    ///
    /// Required when using custom format. Set the value to a string with the
    /// desired formatting codes: <https://adsabs.github.io/help/actions/export>
    pub fn format(mut self, format: &str) -> Self {
        self.format = Some(format.to_owned());
        self
    }
}

#[cfg(feature = "blocking")]
impl<'ads> Export<'ads> {
    /// Synchronously submit the request.
    ///
    /// # Errors
    ///
    /// This method fails on HTTP errors, with messages from the server.
    pub fn send(&self) -> Result<String> {
        let response: Response = self
            .client
            .blocking_post(
                &format!("export/{:?}", self.format_type).to_lowercase(),
                Some(self),
            )?
            .json()?;
        Ok(response.export)
    }
}

#[cfg(feature = "async")]
impl<'ads> Export<'ads> {
    /// Asynchronously submit the request.
    ///
    /// # Errors
    ///
    /// This method fails on HTTP errors, with messages from the server.
    pub async fn send_async(&self) -> Result<String> {
        let response: Response = self
            .client
            .async_post(
                &format!("export/{:?}", self.format_type).to_lowercase(),
                Some(self),
            )
            .await?
            .json()
            .await?;
        Ok(response.export)
    }
}
