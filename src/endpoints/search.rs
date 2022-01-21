//! An interface to the Search endpoint of the ADS API.
//!
//! # Examples
//!
//! The primary interface is [`Search`], and this will generally be accessed via
//! the [`crate::Ads::search`] method as follows:
//!
//! ```no_run
//! # fn run() -> adsabs::Result<()> {
//! use adsabs::Ads;
//! let api_token = "ADS_API_TOKEN";
//! let client = Ads::new(api_token)?;
//! let query = client.search("supernova");
//! # Ok(())
//! # }
//! ```
//!
//! The results from the API will typically be pagniated, but this interface
//! includes support for iterating over all results without worrying about this:
//!
//! ```no_run
//! # fn run() -> adsabs::Result<()> {
//! # use adsabs::Ads;
//! # let api_token = "ADS_API_TOKEN";
//! # let client = Ads::new(api_token)?;
//! for doc in client.search("supernova").iter().limit(5) {
//!     println!("{:?}", doc?);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! But, **take note of the 'limit' call in the above example**, because
//! iterating over all documents with the search term "supernova" will quickly
//! cause you to hit your API limits. It would also be possible to use
//! [`std::iter::Iterator::take`], instead of [`SearchIter::limit`], but the
//! former gives us more information, and allows us to minimize the load on the
//! API servers.

use super::{comma_separated, Sort};
use crate::error::Result;
#[cfg(feature = "async")]
use futures_util::Stream;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

// The maximum number of rows that the API allows
const MAX_ROWS: u64 = 2000;

/// A builder for a search API query that can be used to customize and filter
/// the query.
///
/// # Example
///
/// This should generally be accessed via [`crate::Ads::search`] as follows:
///
/// ```no_run
/// # fn run() -> adsabs::Result<()> {
/// # use adsabs::Ads;
/// # let api_token = "ADS_API_TOKEN";
/// # let client = Ads::new(api_token)?;
/// client.search("supernova");
/// # Ok(())
/// # }
/// ```
#[derive(Serialize, Clone)]
#[must_use]
pub struct Search<'ads> {
    #[serde(skip)]
    client: &'ads crate::Ads,
    q: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    rows: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<u64>,
    #[serde(serialize_with = "fl_defaults")]
    fl: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fq: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(serialize_with = "comma_separated")]
    sort: Vec<Sort>,
}

/// A single page of responses from the search API.
#[derive(Serialize, Deserialize, Clone)]
pub struct Response<T> {
    #[serde(rename = "numFound")]
    pub num_found: u64,
    pub start: u64,
    pub docs: Vec<T>,
}

impl<'ads> Search<'ads> {
    /// Build a new query.
    ///
    /// This should generally be accessed using [`crate::Ads::search`] instead
    /// of this method directly.
    pub fn new(client: &'ads crate::Ads, query: &str) -> Self {
        Self {
            client,
            q: query.to_owned(),
            rows: None,
            start: None,
            fl: Vec::new(),
            fq: None,
            sort: Vec::new(),
        }
    }

    /// The starting point for returned results, used for pagination.
    ///
    /// The default is `0`. To return the next page of results, set start equal
    /// to the value of start from the previous request, plus the number of
    /// results returned in the previous request. For the default values, set
    /// `start=10` to return the second page of results.
    pub fn start(mut self, start: u64) -> Self {
        self.start = Some(start);
        self
    }

    /// The list of fields to return.
    ///
    /// The value should be a comma separated list of field names, e.g.
    /// `fl=bibcode,author,title`. The default is the document id (`fl=id`). A
    /// non-exhaustive list of available fields is available at:
    /// <https://adsabs.github.io/help/search/comprehensive-solr-term-list>
    pub fn fl(mut self, fl: &str) -> Self {
        self.fl.push(fl.to_owned());
        self
    }

    /// Filters the list of search results.
    ///
    /// The syntax is the same as that for the `q` parameter. Adding search
    /// parameters via the `fq` parameter can speed up search results, as it
    /// searches only the results returned by the search entered via the `q`
    /// parameter, not the entire index.
    ///
    /// Note: multiple values for this are not yet supported by this client.
    pub fn fq(mut self, fq: &str) -> Self {
        self.fq = Some(fq.to_owned());
        self
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

    /// The number of results to return per page.
    ///
    /// The default is `10` and the maximum is `2000`. [`IterDocs::limit`]
    /// should generally be used instead, for limiting the total number of
    /// records.
    pub fn rows(mut self, rows: u64) -> Self {
        self.rows = Some(rows);
        self
    }
}

#[cfg(feature = "blocking")]
impl<'ads> Search<'ads> {
    /// Submit the search query.
    ///
    /// # Errors
    ///
    /// This method fails on HTTP errors, with messages from the server.
    pub fn send<T: DeserializeOwned>(&self) -> Result<Response<T>> {
        let data: serde_json::Value = self
            .client
            .blocking_get("search/query", Some(self))?
            .json()?;
        Ok(serde_json::from_value(data["response"].clone())?)
    }

    /// Get an iterator over all search results with transparent support for
    /// pagination.
    pub fn iter<T: DeserializeOwned>(self) -> iter::SearchIter<'ads, T> {
        let start = self.start.unwrap_or(0);
        iter::SearchIter {
            query: self,
            num_found: 0,
            start,
            limit: None,
            docs: Vec::new().into_iter(),
        }
    }
}

#[cfg(feature = "blocking")]
mod iter {
    use super::{Result, Search, MAX_ROWS};
    use serde::de::DeserializeOwned;

    /// An iterator over the results of a query with transparent support for
    /// pagination.
    #[allow(clippy::module_name_repetitions)]
    #[must_use]
    pub struct SearchIter<'ads, T: DeserializeOwned> {
        pub(crate) query: Search<'ads>,
        pub(crate) num_found: u64,
        pub(crate) start: u64,
        pub(crate) limit: Option<u64>,
        pub(crate) docs: <Vec<T> as IntoIterator>::IntoIter,
    }

    impl<'ads, T: DeserializeOwned> SearchIter<'ads, T> {
        /// Limit the total number of results returned.
        ///
        /// Every attempt will be made to minimize the number of API calls, so this
        /// should be preferred to using the [`std::iter::Iterator::take`] method.
        pub fn limit(mut self, limit: u64) -> Self {
            self.limit = Some(limit);
            self
        }

        #[inline]
        fn page_size(&self) -> u64 {
            MAX_ROWS.min(
                self.limit
                    .unwrap_or_else(|| self.query.rows.unwrap_or(MAX_ROWS)),
            )
        }

        fn try_next(&mut self) -> Result<Option<T>> {
            if let Some(doc) = self.docs.next() {
                self.start += 1;
                return Ok(Some(doc));
            }

            if self.start > 0
                && (self.start >= self.num_found || self.start >= self.limit.unwrap_or(u64::MAX))
            {
                return Ok(None);
            }

            let response = self
                .query
                .clone()
                .start(self.start)
                .rows(self.page_size())
                .send()?;
            self.num_found = response.num_found;
            self.start = response.start + 1;
            self.docs = response.docs.into_iter();
            Ok(self.docs.next())
        }
    }

    impl<'ads, T: DeserializeOwned> Iterator for SearchIter<'ads, T> {
        type Item = Result<T>;

        fn next(&mut self) -> Option<Self::Item> {
            match self.try_next() {
                Ok(Some(doc)) => Some(Ok(doc)),
                Ok(None) => None,
                Err(err) => Some(Err(err)),
            }
        }
    }
}

#[cfg(feature = "async")]
impl<'ads> Search<'ads> {
    /// Asynchronously submit the seach query.
    ///
    /// # Errors
    ///
    /// This method fails on HTTP errors, with messages from the server.
    pub async fn send_async<T: DeserializeOwned>(&self) -> Result<Response<T>> {
        let data: serde_json::Value = self
            .client
            .async_get("search/query", Some(self))
            .await?
            .json()
            .await?;
        Ok(serde_json::from_value(data["response"].clone())?)
    }

    /// Get an asynchronous stream over all search results with transparent
    /// support for pagination.
    #[must_use]
    pub fn stream<T: 'ads + DeserializeOwned>(
        self,
    ) -> std::pin::Pin<Box<impl Stream<Item = Result<T>> + 'ads>> {
        use async_stream::try_stream;
        let mut offset = self.start.unwrap_or(0);
        let per_page = self.rows.unwrap_or(10);
        Box::pin(try_stream! {
            loop {
                let builder = self.clone();
                let current = builder.start(offset).rows(per_page).send_async().await?;
                let num = current.docs.len();
                if num == 0 {
                    break;
                }
                for doc in current.docs  {
                    yield doc;
                }
                offset += num as u64;
            }
        })
    }
}

// Helpers for serialization of search queries:
fn fl_defaults<S: serde::Serializer>(items: &[String], serializer: S) -> Result<S::Ok, S::Error> {
    if items.is_empty() {
        serializer.serialize_str("author,first_author,bibcode,id,year,title")
    } else {
        comma_separated(items, serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Database, Document};
    use chrono::Datelike;

    #[test]
    fn deserialize_document() {
        let data = "
        {
            \"abstract\": \"abstract\",
            \"aff\": [\"aff1\"],
            \"database\": [\"astronomy\"],
            \"entdate\": \"2021-09-25\",
            \"indexstamp\":\"2021-10-24T07:56:53.361Z\"
        }
        ";
        let response: Document = serde_json::from_str(data).unwrap();
        assert_eq!(response.abs.unwrap(), "abstract");
        assert_eq!(response.aff.unwrap()[0], "aff1");
        assert!(matches!(response.database.unwrap()[0], Database::Astronomy));
        assert_eq!(response.entdate.unwrap(), "2021-09-25");
        assert_eq!(response.indexstamp.unwrap().year(), 2021);
    }

    #[test]
    fn deserialize_search_response() {
        let data = "
        {
            \"numFound\": 194,
            \"start\": 12,
            \"docs\": [
                {
                    \"id\": \"312911\"
                },
                {
                    \"id\": \"1877482\"
                }
            ]
        }";
        let response: Response<Document> = serde_json::from_str(data).unwrap();
        assert_eq!(response.num_found, 194);
        assert_eq!(response.start, 12);
        assert_eq!(response.docs.len(), 2);
    }

    #[test]
    fn basic_query() {
        let client = crate::Ads::new("token").unwrap();
        let query = Search::new(&client, "au:foreman-mackey")
            .rows(10)
            .start(5)
            .fl("id")
            .fl("author")
            .fq("au:hogg")
            .sort("citation_count");

        assert_eq!(
            serde_json::to_value(query).unwrap(),
            serde_json::json!({
                "q": "au:foreman-mackey",
                "rows": 10,
                "start": 5,
                "fl": "id,author",
                "fq": "au:hogg",
                "sort": "citation_count desc",
            })
        )
    }

    #[test]
    fn vec_fls() {
        let client = crate::Ads::new("token").unwrap();
        let query = Search::new(&client, "au:foreman-mackey").fl("id,author");

        assert_eq!(
            serde_json::to_value(query).unwrap(),
            serde_json::json!({
                "q": "au:foreman-mackey",
                "fl": "id,author",
            })
        )
    }
}
