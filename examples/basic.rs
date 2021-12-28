use adsabs::prelude::*;

fn main() -> Result<(), AdsError> {
    let client = Ads::from_env()?;

    println!("\nquery: 'supernova'");
    for doc in client
        .search("supernova")
        .sort("citation_count", &SortOrder::Desc)
        .iter()
        .take(5)
    {
        let doc = doc?;
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
        .sort("citation_count", &SortOrder::Desc)
        .iter()
        .take(5)
    {
        let doc = doc?;
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
        .sort_desc("date")
        .iter()
        .take(5)
    {
        let doc = doc?;
        println!(
            "{} ({}): {}",
            doc.first_author.unwrap(),
            doc.year.unwrap(),
            doc.title.unwrap().join(" ")
        );
    }

    Ok(())
}
