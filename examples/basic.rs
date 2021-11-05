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
    while let Some(doc) = docs.next().await {
        println!("got {:?}", doc);
    }
    Ok(())
}
