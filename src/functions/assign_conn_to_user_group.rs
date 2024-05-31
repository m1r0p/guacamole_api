///// local modules
pub use crate::conf::GUA_REST_USER_GROUPS;
pub use crate::enums::ProtoBasedAttributes;
pub use crate::structures::guaconn::GuaConn;

///// external crates
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use std::error::Error;

#[tokio::main]
pub async fn assign_conn_to_user_group(
    gua_address: &String,
    gua_token: &String,
    gua_conn: &GuaConn,
    user_group: &String,
) -> Result<(), Box<dyn Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, format!("application/json").parse().unwrap());

    //let test_name: String = String::from("test_grp1");

    let request_data = format!(
        r#"[
        {{
        "op": "add",
        "path": "/connectionPermissions/{}",
        "value": "READ"
        }}
        ]
        "#,
        gua_conn.identifier
    );

    let client = reqwest::Client::new();

    //let mut username: String = String::new();
    //match &gua_conn.proto_based_attributes {
    //    ProtoBasedAttributes::RDP(x) => username = x.username.to_owned(),
    //    _ => println!(""),
    //}

    let _resp = client
        .patch(format!(
            "{}{}/{}/permissions?token={}",
            gua_address, GUA_REST_USER_GROUPS, user_group, gua_token
        ))
        .headers(headers.clone())
        .body(request_data)
        .send()
        .await?
        .text()
        .await?;

    return Ok(());
}
