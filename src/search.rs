use crate::{error::Error, models::SearchResponse};
use async_stream::try_stream;
use futures_core::stream::Stream;
use futures_util::stream::StreamExt;

#[derive(serde::Serialize, Clone)]
pub struct SearchBuilder<'ads> {
    #[serde(skip)]
    client: &'ads crate::Client,
    q: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    rows: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<i32>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(serialize_with = "comma_separated")]
    fl: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fq: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<String>,
}

impl<'ads> SearchBuilder<'ads> {
    pub fn new(client: &'ads crate::Client, query: impl Into<String>) -> Self {
        Self {
            client,
            q: query.into(),
            rows: None,
            start: None,
            fl: Vec::new(),
            fq: None,
            sort: None,
        }
    }

    /// The number of results to return. The default is `10` and the maximum is
    /// `2000`.
    pub fn rows(mut self, rows: impl Into<i32>) -> Self {
        self.rows = Some(rows.into());
        self
    }

    /// The starting point for returned results, used for pagination. The
    /// default is `0`. To return the next page of results, set start equal to
    /// the value of start from the previous request, plus the number of results
    /// returned in the previous request. For the default values, set `start=10`
    /// to return the second page of results.
    pub fn start(mut self, start: impl Into<i32>) -> Self {
        self.start = Some(start.into());
        self
    }

    /// The list of fields to return. The value should be a comma separated list
    /// of field names, e.g. `fl=bibcode,author,title`. The default is the
    /// document id (`fl=id`). A non-exhaustive list of available fields is
    /// available at:
    /// https://adsabs.github.io/help/search/comprehensive-solr-term-list
    pub fn fl(mut self, fl: impl Into<String>) -> Self {
        self.fl.push(fl.into());
        self
    }

    /// Filters the list of search results. The syntax is the same as that for
    /// the `q` parameter. Adding search parameters via the `fq` parameter can
    /// speed up search results, as it searches only the results returned by the
    /// search entered via the `q` parameter, not the entire index.
    ///
    /// Note: multiple values for this are not yet supported by this client.
    pub fn fq(mut self, fq: impl Into<String>) -> Self {
        self.fq = Some(fq.into());
        self
    }

    /// The sorting field and direction to be used when returning results. The
    /// `field` argument should be a valid field name and the `asc` parameter
    /// should be `true` for an ascending sort and `false` for a descending
    /// sort. The default sort method is the relevancy score as calculated by
    /// the search engine. Other useful fields to sort on may be `date`,
    /// `read_count`, `first_author`, or `bibcode`.
    pub fn sort(mut self, field: &str, order: SortOrder) -> Self {
        self.sort = Some(format!(
            "{}+{}",
            field,
            match order {
                SortOrder::Asc => "asc",
                SortOrder::Desc => "desc",
            }
        ));
        self
    }

    /// Send the actual request.
    pub async fn send(self) -> crate::Result<SearchResponse> {
        let text = self
            .client
            .get("search/query", Some(&self))
            .await?
            .text()
            .await?;
        let data: serde_json::Value = serde_json::from_str(&text)?;
        if let serde_json::Value::String(msg) = &data["error"]["msg"] {
            return Err(Error::AdsError(msg.to_owned()));
        }
        Ok(serde_json::from_value(data["response"].to_owned())?)
    }

    /// Get a stream of pages.
    pub fn pages(self) -> impl Stream<Item = crate::Result<crate::models::SearchResponse>> + 'ads {
        let mut page = self.start.unwrap_or(0);
        let per_page = self.rows.unwrap_or(10);
        Box::pin(try_stream! {
            loop {
                let builder = self.clone();
                let current = builder.start(page).rows(per_page).send().await?;
                if current.docs.len() == 0 {
                    break;
                }
                yield current;
                page += per_page;
            }
        })
    }

    /// Get a stream of search results.
    pub fn into_stream(self) -> impl Stream<Item = crate::Result<crate::models::Document>> + 'ads {
        Box::pin(try_stream! {
            let mut pages = self.pages();
            while let Some(page) = pages.next().await {
                for doc in page?.docs {
                    yield doc;
                }
            }
        })
    }
}

pub enum SortOrder {
    Asc,
    Desc,
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
        let client = crate::ClientBuilder::new()
            .set_token("token")
            .build()
            .unwrap();
        let query = SearchBuilder::new(&client, "au:foreman-mackey")
            .rows(10)
            .start(5)
            .fl("id")
            .fl("author")
            .fq("au:hogg")
            .sort("citation_count", SortOrder::Desc);

        assert_eq!(
            serde_json::to_value(query).unwrap(),
            serde_json::json!({
                "q": "au:foreman-mackey",
                "rows": 10,
                "start": 5,
                "fl": "id,author",
                "fq": "au:hogg",
                "sort": "citation_count+desc",
            })
        )
    }

    #[test]
    fn vec_fls() {
        let client = crate::ClientBuilder::new()
            .set_token("token")
            .build()
            .unwrap();
        let query = SearchBuilder::new(&client, "au:foreman-mackey").fl("id,author");

        assert_eq!(
            serde_json::to_value(query).unwrap(),
            serde_json::json!({
                "q": "au:foreman-mackey",
                "fl": "id,author",
            })
        )
    }
}
