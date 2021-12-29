//! An interface to the Search endpoint of the ADS API.
//!
//! # Examples
//!
//! The primary interface is [`Query`], and this will generally be accessed via
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
//! for doc in client.search("supernova").iter_docs().limit(5) {
//!     println!("{:?}", doc?.title);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! But, **take note of the 'limit' call in the above example**, because
//! iterating over all documents with the search term "supernova" will quickly
//! cause you to hit your API limits. It would also be possible to use
//! [`std::iter::Iterator::take`], instead of [`IterDocs::limit`], but the
//! former gives us more information, and allows us to minimize the load on the
//! API servers.

use crate::error::{AdsError, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
#[derive(serde::Serialize, Clone)]
#[must_use]
pub struct Query<'ads> {
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
pub struct Response {
    #[serde(rename = "numFound")]
    pub num_found: u64,
    pub start: u64,
    pub docs: Vec<Document>,
}

/// A `Document` returned from a search query. All of the fields are `Option`s
/// and will only be `Some` if that field was requested in the query using
/// [`Query::fl`].
#[adsabs_macro::make_optional]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Document {
    #[serde(rename = "abstract")]
    pub abs: String,
    pub ack: String,
    pub aff: Vec<String>,
    pub aff_id: Vec<String>,
    pub alternate_bibcode: Vec<String>,
    pub alternate_title: Vec<String>,
    pub arxiv_class: Vec<String>,
    pub author: Vec<String>,
    pub author_count: u64,
    pub author_norm: Vec<String>,
    pub bibcode: String,
    pub bibgroup: Vec<String>,
    pub bibstem: Vec<String>,
    pub citation: Vec<String>,
    pub citation_count: u64,
    pub cite_read_boost: f32,
    pub classic_factor: u64,
    pub comment: String,
    pub copyright: String,
    pub data: Vec<String>,
    pub database: Vec<Database>,
    pub date: DateTime<Utc>,
    pub doctype: DocType,
    pub doi: Vec<String>,
    pub eid: String,
    pub entdate: String, // YYYY-MM-DD
    pub entry_date: DateTime<Utc>,
    pub esources: Vec<String>,
    pub facility: Vec<String>,
    pub first_author: String,
    pub first_author_norm: String,
    pub grant: Vec<String>,
    pub grant_agencies: Vec<String>,
    pub grant_id: Vec<String>,
    pub id: String,
    pub identifier: Vec<String>,
    pub indexstamp: DateTime<Utc>,
    pub inst: Vec<String>,
    pub isbn: Vec<String>,
    pub issn: Vec<String>,
    pub issue: String,
    pub keyword: Vec<String>,
    pub keyword_norm: Vec<String>,
    pub keyword_schema: Vec<String>,
    pub lang: String,
    pub links_data: Vec<String>,
    pub nedid: Vec<String>,
    pub nedtype: Vec<String>,
    pub orcid_pub: Vec<String>,
    pub orcid_other: Vec<String>,
    pub orcid_user: Vec<String>,
    pub page: Vec<String>,
    pub page_count: String,
    pub page_range: String,
    pub property: Vec<String>,
    #[serde(rename = "pub")]
    pub publication: String,
    pub pub_raw: String,
    pub pubdate: String, // YYYY-MM-DD
    pub pubnote: Vec<String>,
    pub read_count: u64,
    pub reference: Vec<String>,
    pub simbid: Vec<String>,
    pub title: Vec<String>,
    pub vizier: Vec<String>,
    pub volume: String,
    pub year: String,
}

/// The databases supported by the search API.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Database {
    Astronomy,
    Physics,
    General,
}

/// The document types supported by the search API.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum DocType {
    Article,
    Eprint,
    Inproceedings,
    Inbook,
    Abstract,
    Book,
    Bookreview,
    Catalog,
    Circular,
    Erratum,
    Mastersthesis,
    Newsletter,
    Obituary,
    Phdthesis,
    Pressrelease,
    Proceedings,
    Proposal,
    Software,
    Talk,
    Techreport,
    Misc,
}

impl<'ads> Query<'ads> {
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

    /// Submit the seach query.
    ///
    /// # Errors
    ///
    /// This method fails on HTTP errors, with messages from the server.
    pub fn send(&self) -> Result<Response> {
        let data: serde_json::Value = self.client.get("search/query", Some(self))?.json()?;
        if let Some(serde_json::Value::String(msg)) = data.get("error").and_then(|x| x.get("msg")) {
            return Err(AdsError::Ads(msg.clone()));
        }
        Ok(serde_json::from_value(data["response"].clone())?)
    }

    /// Get an iterator over all search results with transparent support for
    /// pagination.
    pub fn iter_docs(self) -> IterDocs<'ads> {
        let start = self.start.unwrap_or(0);
        IterDocs {
            query: self,
            num_found: 0,
            start,
            limit: None,
            docs: Vec::new().into_iter(),
        }
    }
}

/// Used to set the order for sorting query results.
///
/// # Examples
///
/// By default, fields are sorted in descending order, so the following queries
/// are equivalent:
///
/// ```no_run
/// # fn run() -> adsabs::Result<()> {
/// # use adsabs::{Ads, search::Sort};
/// # let api_token = "ADS_API_TOKEN";
/// # let client = Ads::new(api_token)?;
/// client.search("supernova").sort("date");
/// # Ok(())
/// # }
/// ```
///
/// and
///
/// ```no_run
/// # fn run() -> adsabs::Result<()> {
/// # use adsabs::{Ads, search::Sort};
/// # let api_token = "ADS_API_TOKEN";
/// # let client = Ads::new(api_token)?;
/// client.search("supernova").sort(Sort::desc("date"));
/// # Ok(())
/// # }
/// ```
///
/// Ascending order can be requested using:
///
/// ```no_run
/// # fn run() -> adsabs::Result<()> {
/// # use adsabs::{Ads, search::Sort};
/// # let api_token = "ADS_API_TOKEN";
/// # let client = Ads::new(api_token)?;
/// client.search("supernova").sort(Sort::asc("date"));
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
pub enum Sort {
    Asc(String),
    Desc(String),
}

impl Sort {
    /// Build an ascending sort on a field.
    pub fn asc(field: &str) -> Self {
        Sort::Asc(field.to_owned())
    }

    /// Build a descending sort on a field.
    pub fn desc(field: &str) -> Self {
        Sort::Desc(field.to_owned())
    }
}

impl From<&str> for Sort {
    fn from(s: &str) -> Self {
        Sort::Desc(s.to_owned())
    }
}

impl ToString for Sort {
    fn to_string(&self) -> String {
        match self {
            Sort::Asc(fl) => format!("{} asc", fl),
            Sort::Desc(fl) => format!("{} desc", fl),
        }
    }
}

/// An iterator over the results of a query with transparent support for
/// pagination.
#[must_use]
pub struct IterDocs<'ads> {
    query: Query<'ads>,
    num_found: u64,
    start: u64,
    limit: Option<u64>,
    docs: <Vec<Document> as IntoIterator>::IntoIter,
}

impl<'ads> IterDocs<'ads> {
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

    fn try_next(&mut self) -> Result<Option<Document>> {
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

impl<'ads> Iterator for IterDocs<'ads> {
    type Item = Result<Document>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(Some(doc)) => Some(Ok(doc)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
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

fn comma_separated<T: ToString, S: serde::Serializer>(
    items: &[T],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let items = items.iter().map(|x| x.to_string()).collect::<Vec<_>>();
    serializer.serialize_str(&items.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;
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
        let response: Response = serde_json::from_str(data).unwrap();
        assert_eq!(response.num_found, 194);
        assert_eq!(response.start, 12);
        assert_eq!(response.docs.len(), 2);
        assert_eq!(response.docs[0].id.as_ref().unwrap(), "312911");
    }

    #[test]
    fn basic_query() {
        let client = crate::Ads::new("token").unwrap();
        let query = Query::new(&client, "au:foreman-mackey")
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
        let query = Query::new(&client, "au:foreman-mackey").fl("id,author");

        assert_eq!(
            serde_json::to_value(query).unwrap(),
            serde_json::json!({
                "q": "au:foreman-mackey",
                "fl": "id,author",
            })
        )
    }
}
