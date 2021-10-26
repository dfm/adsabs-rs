use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Query {
    pub q: String,
    pub rows: i32,
    pub start: i32,
    pub fl: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub fq: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
}

impl Query {
    pub fn new_query(q: &str) -> Self {
        Query {
            q: q.to_string(),
            ..Query::default()
        }
    }
}

impl Default for Query {
    fn default() -> Self {
        Query {
            q: "".to_string(),
            rows: 10,
            start: 0,
            fl: "id".to_string(),
            // fl: vec![Field::Id],
            fq: "".to_string(),
            sort: None,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Field {
    #[serde(rename = "abstract")]
    Abs,
    Ack,
    Aff,
    AffId,
    AlternateBibcode,
    AlternateTitle,
    ArxivClass,
    Author,
    AuthorCount,
    AuthorNorm,
    Bibcode,
    Bibgroup,
    Bibstem,
    Citation,
    CitationCount,
    CiteReadBoost,
    ClassicFactor,
    Comment,
    Copyright,
    Data,
    Database,
    Date,
    Doctype,
    Doi,
    Eid,
    Entdate,
    EntryDate,
    Esources,
    Facility,
    FirstAuthor,
    FirstAuthorNorm,
    Grant,
    GrantAgencies,
    GrantId,
    Id,
    Identifier,
    Indexstamp,
    Inst,
    Isbn,
    Issn,
    Issue,
    Keyword,
    KeywordNorm,
    KeywordSchema,
    Lang,
    LinksData,
    Nedid,
    Nedtype,
    OrcidPub,
    OrcidOther,
    OrcidUser,
    Page,
    PageCount,
    PageRange,
    Property,
    #[serde(rename = "pub")]
    Publication,
    PubRaw,
    Pubdate,
    Pubnote,
    ReadCount,
    Reference,
    Simbid,
    Title,
    Vizier,
    Volume,
    Year,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_deserialize() {
        let query = Query::default();
        let data = serde_json::to_string(&query).unwrap();
        println!("{}", data);
        assert!(false);
    }
}
