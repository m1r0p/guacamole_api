///// local modules
pub use crate::conf::GUA_REST_CONN_GROUPS;

///// external crates
use std::error::Error;

#[tokio::main]
pub async fn delete_gua_conn_group(
    gua_address: &String,
    gua_token: &String,
    conn_grp_id: &String,
) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();

    let _resp = client
        .delete(format!(
            "{}{}/{}?token={}",
            gua_address, GUA_REST_CONN_GROUPS, conn_grp_id, gua_token
        ))
        .send()
        .await?
        .text()
        .await?;
    return Ok(());
}
