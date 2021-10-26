#[tokio::main]
async fn main() -> adsabs::Result<()> {
    let response = adsabs::Client::default()
        .search("au:foreman-mackey")
        .rows(10)
        .sort("citation_count", adsabs::SortOrder::Desc)
        .send()
        .await?;
    println!("{:?}", response);
    Ok(())
}
