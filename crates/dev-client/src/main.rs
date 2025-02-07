use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let client = httpc_test::new_client("http://localhost:8080")?;

    while !client
        .do_get("/ready")
        .await
        .map(|res| res.status().is_success())
        .unwrap_or(false)
    {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    client.do_get("/hello").await?.print().await?;

    client.do_get("/hello?format=json").await?.print().await?;

    Ok(())
}
