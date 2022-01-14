use adsabs::prelude::*;
use anyhow::{Context, Result};
use clap::Parser;

/// Query the NASA ADS literature search API
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The search query
    query: Vec<String>,

    /// Optional API token; by default loaded from environment
    #[clap(short, long)]
    token: Option<String>,

    /// Field to sort in descending order
    #[clap(short, long)]
    sort: Option<String>,

    /// Limit the number of results
    #[clap(short, long)]
    limit: Option<u64>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let client = if let Some(token) = args.token {
        Ads::new(&token)
    } else {
        Ads::from_env()
    }
    .context("initializing client")?;

    let mut query = client.search(&args.query.join(" "));
    if let Some(field) = args.sort {
        query = query.sort(field);
    }

    let mut iter_docs = query.iter_docs();
    if let Some(limit) = args.limit {
        iter_docs = iter_docs.limit(limit);
    }
    for doc in iter_docs {
        println!(
            "{}",
            serde_json::to_string(&doc.context("parsing document")?)
                .context("serializing document")?
        );
    }

    Ok(())
}
