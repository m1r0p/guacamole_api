///// local modules
pub use crate::conf::GUA_REST_TOKENS;

///// external crates
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use serde_json::Value;
use std::error::Error;

#[tokio::main]
pub async fn create_gua_token(
    gua_address: &String,
    gua_user: &String,
    gua_pass: &String,
) -> Result<String, Box<dyn Error>> {
    let mut token: String = String::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        format!("application/x-www-form-urlencoded")
            .parse()
            .unwrap(),
    );
    let request_data = format!("username={}&password={}", gua_user, gua_pass);
    let client = reqwest::Client::new();

    let resp = client
        .post(format!("{}{}", gua_address, GUA_REST_TOKENS))
        .headers(headers.clone())
        .body(request_data)
        .send()
        .await?
        .text()
        .await?;
    let token_json: Value = serde_json::from_str(resp.as_str()).unwrap();
    let token_clr: &str = &token_json["authToken"].as_str().unwrap();
    //println!("{:?}", token_clr);
    token.push_str(token_clr);
    return Ok(token);
}
