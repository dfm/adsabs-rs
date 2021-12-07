use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() -> adsabs::Result<()> {
    let client = adsabs::Client::default();
    let mut docs = client
        .search("author:foreman-mackey")
        .rows(10)
        .sort("citation_count", adsabs::SortOrder::Desc)
        .fl("id")
        .fl("title")
        .into_stream();
    while let Some(Ok(doc)) = docs.next().await {
        println!(
            "{:?}",
            doc.title.unwrap_or_else(|| vec!["No title".to_owned()])
        );
    }
    Ok(())
}
