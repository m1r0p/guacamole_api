///// local modules
pub use crate::conf::GUA_REST_CONNECTIONS;

///// external crates
use std::error::Error;

#[tokio::main]
pub async fn delete_gua_connection(
    gua_address: &String,
    gua_token: &String,
    conn_id: &String,
) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();

    let _resp = client
        .delete(format!(
            "{}{}/{}?token={}",
            gua_address, GUA_REST_CONNECTIONS, conn_id, gua_token
        ))
        .send()
        .await?
        .text()
        .await?;
    return Ok(());
}
