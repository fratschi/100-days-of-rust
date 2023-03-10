use reqwest;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let x = reqwest::get("https://www.rust-lang.org/") .await?.text().await?;
    println!("{}", x);
    Ok(())
}