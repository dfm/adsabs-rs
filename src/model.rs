use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
