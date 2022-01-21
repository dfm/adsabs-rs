use adsabs::prelude::*;
use anyhow::{Context, Result};
use clap::Parser;
use serde::Deserialize;

#[derive(Parser)]
#[clap(author, version, about, name="adsabs", long_about = None)]
struct Cli {
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

    /// Use a standard output format or select one of the standard values; see
    /// https://adsabs.github.io/help/actions/export
    #[clap(short, long)]
    output: Option<FormatType>,

    /// A custom format string; see https://adsabs.github.io/help/actions/export
    #[clap(short, long)]
    format: Option<String>,

    /// Output JSON instead of a standard or custom format
    #[clap(long)]
    json: bool,

    /// Fields to return in JSON; ignored if `--json` not also set
    #[clap(long)]
    fl: Vec<String>,
}

const DEFAULT_LIMIT: u64 = 100;

#[derive(Debug, Deserialize)]
struct Config {
    token: Option<String>,
    sort: Option<String>,
    limit: Option<u64>,
    output: Option<FormatType>,
    format: Option<String>,
}

fn get_config_from_file() -> Result<Config> {
    let mut config_file = dirs::home_dir().context("could not find home directory")?;
    config_file.push(".config");
    config_file.push("adsabs.toml");
    let data = std::fs::read_to_string(config_file)?;
    Ok(toml::from_str(&data).with_context(|| "could not parse config file")?)
}

fn main() -> Result<()> {
    // Parse the command line arguments
    let mut cli = Cli::parse();

    // Load config file and apply defaults as necessary
    if let Ok(config) = get_config_from_file() {
        cli.token = cli.token.or_else(|| config.token.clone());
        cli.sort = cli.sort.or_else(|| config.sort.clone());
        cli.limit = cli
            .limit
            .or_else(|| config.limit.clone())
            .or_else(|| Some(DEFAULT_LIMIT));
        cli.output = cli.output.or_else(|| config.output.clone());
        cli.format = cli.format.or_else(|| config.format.clone());
    }

    // Initialize the API client
    let client = if let Some(token) = cli.token {
        Ads::new(&token)
    } else {
        Ads::from_env()
    }
    .with_context(|| "could not initialize API client")?;

    // Set up the query
    let mut search = client.search(&cli.query.join(" "));
    if let Some(field) = cli.sort.clone() {
        search = search.sort(field);
    }

    // When JSON output is requested, we only need to do one request
    let response = if cli.json {
        // Select a subset of the fields
        for fl in cli.fl {
            search = search.fl(&fl);
        }

        // We'll request raw JSON and limit the iteration over documents
        let mut iter_docs = search.iter::<serde_json::Value>();
        if let Some(limit) = cli.limit {
            iter_docs = iter_docs.limit(limit);
        }

        // Collect the documents into a string
        let docs = iter_docs
            .collect::<adsabs::Result<Vec<_>>>()
            .with_context(|| "unexpected error when fetching documents from API")?;
        serde_json::to_string(&docs)
            .with_context(|| "unexpected error when serializing documents to JSON")?
    } else {
        // Only retrieve bibcodes for this query since that's all we need for
        // the export endpoint
        search = search.fl("bibcode");
        let mut iter_docs = search.iter::<Document>();
        if let Some(limit) = cli.limit {
            iter_docs = iter_docs.limit(limit);
        }
        let bibcode = iter_docs
            .map(|doc| Ok(doc?.bibcode.with_context(|| "could not load bibcodes")?))
            .collect::<Result<Vec<_>>>()?;

        let (format_type, format) = if cli.output.is_none() && cli.format.is_none() {
            (FormatType::Custom, Some("%3.2m (%Y) <%u>".to_owned()))
        } else {
            (
                cli.output.clone().unwrap_or(FormatType::Custom),
                cli.format.clone(),
            )
        };

        // Query the export endpoint to get the formatted output
        let mut export = client.export(format_type, &bibcode);
        if let Some(format) = format {
            export = export.format(&format);
        }
        if let Some(field) = cli.sort.clone() {
            export = export.sort(field);
        }
        export.send()?
    };

    print!("{}", response);

    Ok(())
}
