use reqwest::Result;
use std::time::Duration;
use reqwest::ClientBuilder;

#[tokio::main]
async fn main() -> Result<()> {
    let user = "kublaikhan1";
    let request_url = format!("https://api.github.com/users/{}", user);
    println!("{}", request_url);

    let timeout = Duration::new(5, 0);
    let client = ClientBuilder::new().timeout(timeout).build()?;
    let response = client.head(&request_url).send().await?;

    print!("{:#?}", response);

    Ok(())
}
