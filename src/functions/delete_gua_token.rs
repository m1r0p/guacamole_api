///// local modules
pub use crate::conf::GUA_REST_TOKENS;

///// external crates
use std::error::Error;

#[tokio::main]
pub async fn delete_gua_token(
    gua_address: &String,
    gua_token: &String,
) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();

    let _resp = client
        .delete(format!("{}{}/{}", gua_address, GUA_REST_TOKENS, gua_token))
        .send()
        .await?
        .text()
        .await?;
    return Ok(());
}
