use adsabs::{search, Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new()?;
    let response = search::SearchBuilder::new(&client, "au:foreman-mackey")
        .rows(10)
        .sort("citation_count", search::SortOrder::Desc)
        .send()
        .await?;
    println!("{:?}", response);
    Ok(())
}
