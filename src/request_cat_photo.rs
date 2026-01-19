use anyhow::Result;
use reqwest;
use bytes::Bytes;
use dotenvy::dotenv;


pub async fn request_cat_photo() -> Result<Bytes, reqwest::Error> {
    dotenv().ok();
    let bytes = reqwest::get(std::env::var("CAT_API").unwrap())
        .await?
        .bytes()
        .await?;

    Ok(bytes)
}
