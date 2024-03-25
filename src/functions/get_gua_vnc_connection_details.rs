///// local modules
pub use crate::conf::GUA_REST_CONNECTIONS;

///// external crates
use serde_json::Value;
use std::error::Error;
use std::sync::Arc;

#[tokio::main]
pub async fn get_gua_vnc_connection_details(
    gua_address: Arc<String>,
    gua_token: Arc<String>,
    conn_id: &String,
) -> Result<[String; 6], Box<dyn Error>> {
    let client = reqwest::Client::new();

    let resp = client
        .get(format!(
            "{}{}/{}/parameters?token={}",
            gua_address, GUA_REST_CONNECTIONS, conn_id, gua_token
        ))
        .send()
        .await?
        .text()
        .await?;
    let resp_json: Value = serde_json::from_str(resp.as_str()).unwrap();

    //let mut domain: String = String::new();
    //match resp_json["domain"].as_str() {
    //    None => domain.push_str("None"),
    //    Some(x) => domain.push_str(x),
    //}

    let mut hostname: String = String::new();
    match resp_json["hostname"].as_str() {
        None => hostname.push_str("None"),
        Some(x) => hostname.push_str(x),
    }

    //let mut ignore_cert: String = String::new();
    //match resp_json["ignore-cert"].as_str() {
    //    None => ignore_cert.push_str("None"),
    //    Some(x) => ignore_cert.push_str(x),
    //}

    let mut port: String = String::new();
    match resp_json["port"].as_str() {
        None => port.push_str("None"),
        Some(x) => port.push_str(x),
    }

    let mut username: String = String::new();
    match resp_json["username"].as_str() {
        None => username.push_str("None"),
        Some(x) => username.push_str(x),
    }

    let mut wol_send_packet: String = String::new();
    match resp_json["wol-send-packet"].as_str() {
        None => wol_send_packet.push_str("None"),
        Some(x) => wol_send_packet.push_str(x),
    }

    let mut wol_mac_addr: String = String::new();
    match resp_json["wol-mac-addr"].as_str() {
        None => wol_mac_addr.push_str("None"),
        Some(x) => wol_mac_addr.push_str(x),
    }

    let mut wol_broadcast_addr: String = String::new();
    match resp_json["wol-broadcast-addr"].as_str() {
        None => wol_broadcast_addr.push_str("None"),
        Some(x) => wol_broadcast_addr.push_str(x),
    }

    let conn_parameters: [String; 6] = [
        hostname,
        port,
        username,
        wol_send_packet,
        wol_mac_addr,
        wol_broadcast_addr,
    ];
    return Ok(conn_parameters);
}
