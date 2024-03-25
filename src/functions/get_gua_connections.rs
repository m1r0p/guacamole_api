///// local modules
pub use crate::conf::GUA_REST_CONNECTIONS;
pub use crate::enums::ProtoBasedAttributes;
pub use crate::functions::get_gua_rdp_connection_details::*;
pub use crate::functions::get_gua_vnc_connection_details::*;
pub use crate::structures::guaconn::{
    GuaConn, GuaConnAttributes, GuaRDPattributes, GuaVNCattributes,
};

///// external crates
//use reqwest::header::{HeaderMap, CONTENT_TYPE};
use serde_json::{Map, Value};
use std::error::Error;
use std::sync::Arc;

#[tokio::main]
pub async fn get_gua_connections(
    gua_address: &String,
    gua_token: &String,
) -> Result<Vec<GuaConn>, Box<dyn Error>> {
    let mut conn_list: Vec<GuaConn> = Vec::new();
    let addr: String = gua_address.clone();
    let tkn: String = gua_token.clone();
    let gua_addr: Arc<String> = Arc::new(addr);
    let gua_tkn: Arc<String> = Arc::new(tkn);

    let client = reqwest::Client::new();

    let resp = client
        .get(format!(
            "{}{}?token={}",
            gua_addr.clone(),
            GUA_REST_CONNECTIONS,
            gua_tkn.clone()
        ))
        .send()
        .await?
        .text()
        .await?;
    let raw_json: Value = serde_json::from_str(resp.as_str()).unwrap();
    let conn_obj_json: &Map<String, Value> = raw_json.as_object().unwrap();
    for raw_conn in conn_obj_json.values() {
        let attributes: GuaConnAttributes = GuaConnAttributes {
            failover_only: raw_conn["attributes"]["failover-only"].to_string(),
            guacd_encryption: raw_conn["attributes"]["guacd-encryption"].to_string(),
            guacd_hostname: raw_conn["attributes"]["guacd-hostname"].to_string(),
            guacd_port: raw_conn["attributes"]["guacd-port"].to_string(),
            max_connections: raw_conn["attributes"]["max-connections"].to_string(),
            max_connections_per_user: raw_conn["attributes"]["max-connections-per-user"]
                .to_string(),
            weight: raw_conn["attributes"]["weight"].to_string(),
        };

        let conn_id: String = raw_conn["identifier"].as_str().unwrap().to_string().clone();
        let gua_addr: Arc<String> = Arc::clone(&gua_addr);
        let gua_tkn: Arc<String> = Arc::clone(&gua_tkn);

        //let rdp_attributes_array: [String; 7] = tokio::task::spawn_blocking(move || {
        //    let rdp_attrs: [String; 7] =
        //        get_gua_rdp_connection_details(gua_addr, gua_tkn, &conn_id).unwrap();
        //    rdp_attrs
        //})
        //.await
        //.unwrap();

        let protocol: String = raw_conn["protocol"].as_str().unwrap().to_string();
        match protocol.as_str() {
            _ if protocol.as_str() == "rdp" => {
                let rdp_attributes_array: [String; 8] = tokio::task::spawn_blocking(move || {
                    let rdp_attrs: [String; 8] =
                        get_gua_rdp_connection_details(gua_addr, gua_tkn, &conn_id).unwrap();
                    rdp_attrs
                })
                .await
                .unwrap();
                let proto_attributes: ProtoBasedAttributes =
                    ProtoBasedAttributes::RDP(GuaRDPattributes {
                        hostname: rdp_attributes_array[0].clone(),
                        port: rdp_attributes_array[1].clone(),
                        username: rdp_attributes_array[2].clone(),
                        domain: rdp_attributes_array[3].clone(),
                        ignore_cert: rdp_attributes_array[4].clone(),
                        wol_send_packet: rdp_attributes_array[5].clone(),
                        wol_mac_addr: rdp_attributes_array[6].clone(),
                        wol_broadcast_addr: rdp_attributes_array[7].clone(),
                    });

                let conn: GuaConn = GuaConn {
                    active_connections: raw_conn["activeConnections"].as_u64().unwrap(),
                    attributes: attributes,
                    identifier: raw_conn["identifier"].as_str().unwrap().to_string(),
                    name: raw_conn["name"].as_str().unwrap().to_string(),
                    parent_identifier: raw_conn["parentIdentifier"].as_str().unwrap().to_string(),
                    protocol: raw_conn["protocol"].as_str().unwrap().to_string(),
                    proto_based_attributes: proto_attributes,
                };

                conn_list.push(conn);
            }

            _ if protocol.as_str() == "vnc" => {
                let vnc_attributes_array: [String; 6] = tokio::task::spawn_blocking(move || {
                    let vnc_attrs: [String; 6] =
                        get_gua_vnc_connection_details(gua_addr, gua_tkn, &conn_id).unwrap();
                    vnc_attrs
                })
                .await
                .unwrap();

                let proto_attributes: ProtoBasedAttributes =
                    ProtoBasedAttributes::VNC(GuaVNCattributes {
                        hostname: vnc_attributes_array[0].clone(),
                        port: vnc_attributes_array[1].clone(),
                        username: vnc_attributes_array[2].clone(),
                        wol_send_packet: vnc_attributes_array[3].clone(),
                        wol_mac_addr: vnc_attributes_array[4].clone(),
                        wol_broadcast_addr: vnc_attributes_array[5].clone(),
                    });

                let conn: GuaConn = GuaConn {
                    active_connections: raw_conn["activeConnections"].as_u64().unwrap(),
                    attributes: attributes,
                    identifier: raw_conn["identifier"].as_str().unwrap().to_string(),
                    name: raw_conn["name"].as_str().unwrap().to_string(),
                    parent_identifier: raw_conn["parentIdentifier"].as_str().unwrap().to_string(),
                    protocol: raw_conn["protocol"].as_str().unwrap().to_string(),
                    proto_based_attributes: proto_attributes,
                };

                conn_list.push(conn);
            }

            &_ => continue,
        }
    }

    return Ok(conn_list);
}
