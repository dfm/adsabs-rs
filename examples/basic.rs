#[tokio::main]
async fn main() -> adsabs::Result<()> {
    let client = adsabs::Client::new()?;

    let response = client.get("search/query", None::<&()>).await?;
    println!("{:?}", response);

    Ok(())
}
