//! An example showing how to access the Export API.
use adsabs::prelude::*;

fn main() -> Result<(), AdsError> {
    let client = Ads::from_env()?;

    println!("=> BibTeX:");
    let export = client
        .export(FormatType::BibTeX, &["2015RaSc...50..916A".to_owned()])
        .send()?;
    println!("{}", export);

    println!("=> Custom format (%m %Y):");
    let export = client
        .export(
            FormatType::Custom,
            &[
                "2000A&AS..143...41K".to_owned(),
                "2000A&AS..143...85A".to_owned(),
                "2000A&AS..143..111G".to_owned(),
            ],
        )
        .format("%m %Y")
        .sort(Sort::Asc("first_author".to_owned()))
        .send()?;
    println!("{}", export);

    Ok(())
}
