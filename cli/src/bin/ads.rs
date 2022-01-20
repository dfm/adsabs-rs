use adsabs::prelude::*;
use anyhow::{Context, Result};
use clap::{AppSettings, Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, name="ads", long_about = None)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Query the NASA ADS literature search API
    Search {
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
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Search {
            query,
            token,
            sort,
            limit,
        } => search(&query, token, sort, limit),
    }
}

fn search(
    query: &[String],
    token: Option<String>,
    sort: Option<String>,
    limit: Option<u64>,
) -> Result<()> {
    let client = if let Some(token) = token {
        Ads::new(&token)
    } else {
        Ads::from_env()
    }
    .context("initializing client")?;

    let mut query = client.search(&query.join(" "));
    if let Some(field) = sort {
        query = query.sort(field);
    }

    let mut iter_docs = query.iter::<Document>();
    if let Some(limit) = limit {
        iter_docs = iter_docs.limit(limit);
    }
    let docs = iter_docs
        .collect::<adsabs::Result<Vec<_>>>()
        .context("fetching documents")?;
    println!(
        "{}",
        serde_json::to_string(&docs).context("serializing documents")?
    );
    Ok(())
}
