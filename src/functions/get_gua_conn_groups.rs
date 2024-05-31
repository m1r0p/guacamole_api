///// local modules
pub use crate::conf::GUA_REST_CONN_GROUPS;
pub use crate::structures::guaconngrp::{GuaConnGrp, GuaConnGrpAttributes};

///// external crates
use serde_json::{Map, Value};
use std::error::Error;
use std::sync::Arc;

#[tokio::main]
pub async fn get_gua_conn_groups(
    gua_address: &String,
    gua_token: &String,
) -> Result<Vec<GuaConnGrp>, Box<dyn Error>> {
    let mut conn_group_list: Vec<GuaConnGrp> = Vec::new();
    let addr: String = gua_address.clone();
    let tkn: String = gua_token.clone();
    let gua_addr: Arc<String> = Arc::new(addr);
    let gua_tkn: Arc<String> = Arc::new(tkn);

    let client = reqwest::Client::new();

    let resp = client
        .get(format!(
            "{}{}?token={}",
            gua_addr.clone(),
            GUA_REST_CONN_GROUPS,
            gua_tkn.clone()
        ))
        .send()
        .await?
        .text()
        .await?;
    let raw_json: Value = serde_json::from_str(resp.as_str()).unwrap();
    let conn_obj_json: &Map<String, Value> = raw_json.as_object().unwrap();
    for raw_conn in conn_obj_json.values() {
        //let mut enable_session_affinity: String = String::new();
        //match raw_conn["attributes"]["enable-session-affinity"].as_str() {
        //    None => enable_session_affinity.push_str("None"),
        //    Some(x) => enable_session_affinity.push_str(x),
        //}

        let attributes: GuaConnGrpAttributes = GuaConnGrpAttributes {
            max_connections: raw_conn["attributes"]["max-connections"].to_string(),
            max_connections_per_user: raw_conn["attributes"]["max-connections-per-user"]
                .to_string(),
            //enable_session_affinity: enable_session_affinity,
            enable_session_affinity: raw_conn["attributes"]["enable-session-affinity"].to_string(),
        };

        let conn_grp: GuaConnGrp = GuaConnGrp {
            name: raw_conn["name"].as_str().unwrap().to_string(),
            identifier: raw_conn["identifier"].as_str().unwrap().to_string(),
            parent_identifier: raw_conn["parentIdentifier"].as_str().unwrap().to_string(),
            conn_grp_type: raw_conn["type"].as_str().unwrap().to_string(),
            active_connections: raw_conn["activeConnections"].as_u64().unwrap(),
            attributes: attributes,
        };

        conn_group_list.push(conn_grp);
    }

    return Ok(conn_group_list);
}
