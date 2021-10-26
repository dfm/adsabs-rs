use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchResponse {
    #[serde(rename = "abstract")]
    pub abs: Option<String>,
    pub ack: Option<String>,
    pub aff: Option<Vec<String>>,
    pub aff_id: Option<Vec<String>>,
    pub alternate_bibcode: Option<Vec<String>>,
    pub alternate_title: Option<Vec<String>>,
    pub arxiv_class: Option<Vec<String>>,
    pub author: Option<Vec<String>>,
    pub author_count: Option<u32>,
    pub author_norm: Option<Vec<String>>,
    pub bibcode: Option<String>,
    pub bibgroup: Option<Vec<String>>,
    pub bibstem: Option<Vec<String>>,
    pub citation: Option<Vec<String>>,
    pub citation_count: Option<u32>,
    pub cite_read_boost: Option<f32>,
    pub classic_factor: Option<u32>,
    pub comment: Option<String>,
    pub copyright: Option<String>,
    pub data: Option<Vec<String>>,
    pub database: Option<Vec<Database>>,
    pub date: Option<DateTime<Utc>>,
    pub doctype: Option<DocType>,
    pub doi: Option<Vec<String>>,
    pub eid: Option<String>,
    pub entdate: Option<String>, // YYYY-MM-DD
    pub entry_date: Option<DateTime<Utc>>,
    pub esources: Option<Vec<String>>,
    pub facility: Option<Vec<String>>,
    pub first_author: Option<String>,
    pub first_author_norm: Option<String>,
    pub grant: Option<Vec<String>>,
    pub grant_agencies: Option<Vec<String>>,
    pub grant_id: Option<Vec<String>>,
    pub id: Option<String>,
    pub identifier: Option<Vec<String>>,
    pub indexstamp: Option<DateTime<Utc>>,
    pub inst: Option<Vec<String>>,
    pub isbn: Option<Vec<String>>,
    pub issn: Option<Vec<String>>,
    pub issue: Option<String>,
    pub keyword: Option<Vec<String>>,
    pub keyword_norm: Option<Vec<String>>,
    pub keyword_schema: Option<Vec<String>>,
    pub lang: Option<String>,
    pub links_data: Option<Vec<String>>,
    pub nedid: Option<Vec<String>>,
    pub nedtype: Option<Vec<String>>,
    pub orcid_pub: Option<Vec<String>>,
    pub orcid_other: Option<Vec<String>>,
    pub orcid_user: Option<Vec<String>>,
    pub page: Option<Vec<String>>,
    pub page_count: Option<String>,
    pub page_range: Option<String>,
    pub property: Option<Vec<String>>,
    #[serde(rename = "pub")]
    pub publication: Option<String>,
    pub pub_raw: Option<String>,
    pub pubdate: Option<String>, // YYYY-MM-DD
    pub pubnote: Option<Vec<String>>,
    pub read_count: Option<u32>,
    pub reference: Option<Vec<String>>,
    pub simbid: Option<Vec<String>>,
    pub title: Option<Vec<String>>,
    pub vizier: Option<Vec<String>>,
    pub volume: Option<String>,
    pub year: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Database {
    Astronomy,
    Physics,
    General,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;
    use std::matches;

    #[test]
    fn basic_deserialize() {
        let data = "
        {
            \"abstract\": \"abstract\",
            \"aff\": [\"aff1\"],
            \"database\": [\"astronomy\"],
            \"entdate\": \"2021-09-25\",
            \"indexstamp\":\"2021-10-24T07:56:53.361Z\"
        }
        ";
        let response: SearchResponse = serde_json::from_str(data).unwrap();
        assert_eq!(response.abs.unwrap(), "abstract");
        assert_eq!(response.aff.unwrap()[0], "aff1");
        assert!(matches!(response.database.unwrap()[0], Database::Astronomy));
        assert_eq!(response.entdate.unwrap(), "2021-09-25");
        assert_eq!(response.indexstamp.unwrap().year(), 2021);
    }
}
