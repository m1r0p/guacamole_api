///// local modules
pub use crate::conf::GUA_REST_CONN_GROUPS;

///// external crates
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use std::error::Error;

#[tokio::main]
pub async fn create_gua_conn_group(
    gua_address: &String,
    gua_token: &String,
    gua_conn_grp_name: &String,
) -> Result<(), Box<dyn Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, format!("application/json").parse().unwrap());

    //let test_name: String = String::from("test_grp1");

    let request_data = format!(
        r#"{{"parentIdentifier": "ROOT",
    "name": "{}",
    "type": "ORGANIZATIONAL",
    "attributes": {{
    "max-connections": "",
    "max-connections-per-user": "",
    "enable-session-affinity": ""
    }}
        }}"#,
        gua_conn_grp_name
    );

    let client = reqwest::Client::new();

    let _resp = client
        .post(format!(
            "{}{}?token={}",
            gua_address, GUA_REST_CONN_GROUPS, gua_token
        ))
        .headers(headers.clone())
        .body(request_data)
        .send()
        .await?
        .text()
        .await?;

    return Ok(());
}
