#[tokio::main]
async fn main() -> adsabs::Result<()> {
    let response = adsabs::Client::default()
        .search("author:foreman-mackey")
        .rows(10)
        .sort("citation_count", adsabs::SortOrder::Desc)
        .fl("id")
        .fl("title")
        .send()
        .await?;
    let data = response.text().await?;
    let data: serde_json::Value = serde_json::from_str(&data).unwrap();
    println!("{:?}", data);
    Ok(())
}
