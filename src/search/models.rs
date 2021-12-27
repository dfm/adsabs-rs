use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response {
    #[serde(rename = "numFound")]
    pub num_found: u32,
    pub start: u32,
    pub docs: Vec<Document>,
}

#[adsabs_macro::make_optional]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(default)]
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
    pub author_count: u32,
    pub author_norm: Vec<String>,
    pub bibcode: String,
    pub bibgroup: Vec<String>,
    pub bibstem: Vec<String>,
    pub citation: Vec<String>,
    pub citation_count: u32,
    pub cite_read_boost: f32,
    pub classic_factor: u32,
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
    pub read_count: u32,
    pub reference: Vec<String>,
    pub simbid: Vec<String>,
    pub title: Vec<String>,
    pub vizier: Vec<String>,
    pub volume: String,
    pub year: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Database {
    Astronomy,
    Physics,
    General,
}

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
}
