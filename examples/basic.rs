#[tokio::main]
async fn main() -> adsabs::Result<()> {
    let client = adsabs::Client::new()?;
    let query = adsabs::search::Query::new_query("author:\"foreman-mackey\"");

    let response = client.get("search/query", Some(&query)).await?;
    println!("{:?}", response);

    Ok(())
}
