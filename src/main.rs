use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = rentry::api::client::HttpAdapter::new()?;

    let token = client.csrf_token().await?;
    println!("token: {}", token);

    Ok(())
}
