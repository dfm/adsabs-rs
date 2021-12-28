mod models;
pub use crate::search::models::{Document, Response};
use crate::{
    error::{AdsError, Result},
    SortOrder,
};

#[derive(serde::Serialize, Clone)]
#[must_use]
pub struct Builder<'ads> {
    #[serde(skip)]
    client: &'ads crate::Ads,
    q: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    rows: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<u32>,
    #[serde(serialize_with = "fl_defaults")]
    fl: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fq: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(serialize_with = "comma_separated")]
    sort: Vec<String>,
}

impl<'ads> Builder<'ads> {
    pub fn new(client: &'ads crate::Ads, query: impl Into<String>) -> Self {
        Self {
            client,
            q: query.into(),
            rows: None,
            start: None,
            fl: Vec::new(),
            fq: None,
            sort: Vec::new(),
        }
    }

    /// The number of results to return. The default is `10` and the maximum is
    /// `2000`.
    pub fn rows(mut self, rows: u32) -> Self {
        self.rows = Some(rows);
        self
    }

    /// The starting point for returned results, used for pagination. The
    /// default is `0`. To return the next page of results, set start equal to
    /// the value of start from the previous request, plus the number of results
    /// returned in the previous request. For the default values, set `start=10`
    /// to return the second page of results.
    pub fn start(mut self, start: u32) -> Self {
        self.start = Some(start);
        self
    }

    /// The list of fields to return. The value should be a comma separated list
    /// of field names, e.g. `fl=bibcode,author,title`. The default is the
    /// document id (`fl=id`). A non-exhaustive list of available fields is
    /// available at:
    /// <https://adsabs.github.io/help/search/comprehensive-solr-term-list>
    pub fn fl(mut self, fl: &str) -> Self {
        self.fl.push(fl.to_string());
        self
    }

    /// Filters the list of search results. The syntax is the same as that for
    /// the `q` parameter. Adding search parameters via the `fq` parameter can
    /// speed up search results, as it searches only the results returned by the
    /// search entered via the `q` parameter, not the entire index.
    ///
    /// Note: multiple values for this are not yet supported by this client.
    pub fn fq(mut self, fq: &str) -> Self {
        self.fq = Some(fq.to_string());
        self
    }

    /// The sorting field and direction to be used when returning results. The
    /// `field` argument should be a valid field name and the `asc` parameter
    /// should be `true` for an ascending sort and `false` for a descending
    /// sort. The default sort method is the relevancy score as calculated by
    /// the search engine. Other useful fields to sort on may be `date`,
    /// `read_count`, `first_author`, or `bibcode`.
    pub fn sort(mut self, field: &str, order: &SortOrder) -> Self {
        self.sort.push(format!(
            "{} {}",
            field,
            match order {
                SortOrder::Asc => "asc",
                SortOrder::Desc => "desc",
            }
        ));
        self
    }

    /// A shortcut for sorting results in ascending order.
    ///
    /// See [`Builder::sort`] for more information.
    pub fn sort_asc(self, field: &str) -> Self {
        self.sort(field, &SortOrder::Asc)
    }

    /// A shortcut for sorting results in descending order.
    ///
    /// See [`Builder::sort`] for more information.
    pub fn sort_desc(self, field: &str) -> Self {
        self.sort(field, &SortOrder::Desc)
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
    #[must_use]
    pub fn iter(&self) -> PaginatedResults {
        PaginatedResults {
            builder: self,
            num_found: 0,
            start: self.start.unwrap_or(0),
            docs: Vec::new().into_iter(),
        }
    }
}

pub struct PaginatedResults<'r> {
    builder: &'r Builder<'r>,
    num_found: u32,
    start: u32,
    docs: <Vec<Document> as IntoIterator>::IntoIter,
}

impl<'r> PaginatedResults<'r> {
    fn try_next(&mut self) -> Result<Option<Document>> {
        if let Some(doc) = self.docs.next() {
            self.start += 1;
            return Ok(Some(doc));
        }

        if self.start > 0 && self.start >= self.num_found {
            return Ok(None);
        }

        let response = self.builder.clone().start(self.start).send()?;
        self.num_found = response.num_found;
        self.start = response.start + 1;
        self.docs = response.docs.into_iter();
        Ok(self.docs.next())
    }
}

impl<'r> Iterator for PaginatedResults<'r> {
    type Item = Result<Document>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(Some(doc)) => Some(Ok(doc)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}

fn fl_defaults<S: serde::Serializer>(items: &[String], serializer: S) -> Result<S::Ok, S::Error> {
    if items.is_empty() {
        serializer.serialize_str("author,first_author,bibcode,id,year,title")
    } else {
        comma_separated(items, serializer)
    }
}

fn comma_separated<S: serde::Serializer>(
    items: &[String],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&items.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_query() {
        let client = crate::Ads::new("token").unwrap();
        let query = Builder::new(&client, "au:foreman-mackey")
            .rows(10)
            .start(5)
            .fl("id")
            .fl("author")
            .fq("au:hogg")
            .sort("citation_count", &SortOrder::Desc);

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
        let query = Builder::new(&client, "au:foreman-mackey").fl("id,author");

        assert_eq!(
            serde_json::to_value(query).unwrap(),
            serde_json::json!({
                "q": "au:foreman-mackey",
                "fl": "id,author",
            })
        )
    }
}
