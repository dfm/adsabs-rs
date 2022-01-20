pub mod export;
pub mod search;

// Helpers for serializing queries
pub(crate) fn comma_separated<T: ToString, S: serde::Serializer>(
    items: &[T],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let items = items
        .iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<_>>();
    serializer.serialize_str(&items.join(","))
}

/// Used to set the order for sorting query results.
///
/// # Examples
///
/// By default, fields are sorted in descending order, so the following queries
/// are equivalent:
///
/// ```no_run
/// # fn run() -> adsabs::Result<()> {
/// # use adsabs::{Ads, search::Sort};
/// # let api_token = "ADS_API_TOKEN";
/// # let client = Ads::new(api_token)?;
/// client.search("supernova").sort("date");
/// # Ok(())
/// # }
/// ```
///
/// and
///
/// ```no_run
/// # fn run() -> adsabs::Result<()> {
/// # use adsabs::{Ads, search::Sort};
/// # let api_token = "ADS_API_TOKEN";
/// # let client = Ads::new(api_token)?;
/// client.search("supernova").sort(Sort::desc("date"));
/// # Ok(())
/// # }
/// ```
///
/// Ascending order can be requested using:
///
/// ```no_run
/// # fn run() -> adsabs::Result<()> {
/// # use adsabs::{Ads, search::Sort};
/// # let api_token = "ADS_API_TOKEN";
/// # let client = Ads::new(api_token)?;
/// client.search("supernova").sort(Sort::asc("date"));
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
pub enum Sort {
    Asc(String),
    Desc(String),
}

// Helpers for serialization and deserialization of sort orders
impl Sort {
    /// Build an ascending sort on a field.
    pub fn asc(field: &str) -> Self {
        Sort::Asc(field.to_owned())
    }

    /// Build a descending sort on a field.
    pub fn desc(field: &str) -> Self {
        Sort::Desc(field.to_owned())
    }
}

impl From<&str> for Sort {
    fn from(s: &str) -> Self {
        Sort::Desc(s.to_owned())
    }
}

impl From<String> for Sort {
    fn from(s: String) -> Self {
        Sort::Desc(s)
    }
}

impl ToString for Sort {
    fn to_string(&self) -> String {
        match self {
            Sort::Asc(fl) => format!("{} asc", fl),
            Sort::Desc(fl) => format!("{} desc", fl),
        }
    }
}
