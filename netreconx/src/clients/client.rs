use reqwest::{blocking::Client, redirect};
use std::time::Duration;

pub fn build_client() -> Result<Client, anyhow::Error>{
    let client = Client::builder()
        .redirect(redirect::Policy::limited(4))
        .timeout(Duration::from_secs(5))
        .build()?;
    Ok(client)
}
