#[derive(serde::Serialize)]
pub struct SearchBuilder<'ads, 'q, 'fl> {
    #[serde(skip)]
    client: &'ads crate::Client,
    q: &'q str,
    #[serde(skip_serializing_if = "Option::is_none")]
    rows: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "comma_separated")]
    fl: Option<&'fl [String]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fq: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<String>,
}

impl<'ads, 'q, 'fl> SearchBuilder<'ads, 'q, 'fl> {
    pub fn new(client: &'ads crate::Client, query: &'q str) -> Self {
        Self {
            client,
            q: query,
            rows: None,
            start: None,
            fl: None,
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
    pub fn fl(mut self, fl: &'fl (impl AsRef<[String]> + ?Sized)) -> Self {
        self.fl = Some(fl.as_ref());
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
    pub async fn send(self) -> crate::Result<reqwest::Response> {
        self.client.get("search/query", Some(&self)).await
    }
}

pub enum SortOrder {
    Asc,
    Desc,
}

fn comma_separated<S: serde::Serializer>(
    items: &Option<&[String]>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&items.unwrap().join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_query() {
        let client = crate::Client::new_with_token("token").unwrap();
        let fl = vec!["id".to_string(), "author".to_string()];
        let query = SearchBuilder::new(&client, "au:foreman-mackey")
            .rows(10)
            .start(5)
            .fl(&fl)
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
}
