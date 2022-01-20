//! An example showing basic usage examples from the docs.
use adsabs::{prelude::*, Document};

fn main() -> Result<(), AdsError> {
    let client = Ads::from_env()?;

    println!("\nquery: 'supernova'");
    for doc in client
        .search("supernova")
        .sort("citation_count")
        .iter()
        .limit(5)
    {
        let doc: Document = doc?;
        println!(
            "{} ({}): {}",
            doc.first_author.unwrap(),
            doc.year.unwrap(),
            doc.title.unwrap().join(" ")
        );
    }

    println!("\nquery: 'author:\"^Dalcanton, J\"'");
    for doc in client
        .search("author:\"^Dalcanton, J\"")
        .sort("citation_count")
        .iter()
        .limit(5)
    {
        let doc: Document = doc?;
        println!(
            "{} ({}): {}",
            doc.first_author.unwrap(),
            doc.year.unwrap(),
            doc.title.unwrap().join(" ")
        );
    }

    println!("\nquery: 'aff:\"Flatiron Institute\"'");
    for doc in client
        .search("aff:\"Flatiron Institute\"")
        .sort(Sort::Asc("date".to_owned()))
        .iter()
        .limit(5)
    {
        let doc: Document = doc?;
        println!(
            "{} ({}): {}",
            doc.first_author.unwrap(),
            doc.year.unwrap(),
            doc.title.unwrap().join(" ")
        );
    }

    Ok(())
}
