#![allow(dead_code)]

pub mod conf;
pub use conf::{GUA_REST_CONNECTIONS, GUA_REST_CONN_GROUPS, GUA_REST_TOKENS};
pub mod structures;
pub use structures::enums::ProtoBasedAttributes;
pub use structures::{
    GuaConn, GuaConnAttributes, GuaConnGrp, GuaConnGrpAttributes, GuaRDPattributes,
    GuaVNCattributes, SccmHost,
};

use config::{Config, File, FileFormat};
use csv;
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use serde_json::{Map, Value};
use std::error::Error;
//use std::thread;
use std::sync::Arc;

//fn print_type_of<T>(_: &T) {
//    println!("{}", std::any::type_name::<T>())
//}

pub fn get_config_params(string_path: String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut config_params: Vec<String> = Vec::new();

    let mut builder = Config::builder();
    builder = builder.set_default("default", "1")?;
    builder = builder.add_source(File::new(&string_path, FileFormat::Json));
    builder = builder.set_override("override", "1")?;
    let raw_conf = builder.build().unwrap();
    config_params.push(raw_conf.get("csv_input_file").unwrap());
    config_params.push(raw_conf.get("gua_proto_address").unwrap());
    config_params.push(raw_conf.get("gua_user").unwrap());
    config_params.push(raw_conf.get("gua_pass").unwrap());

    return Ok(config_params);
}

pub fn parse_csv(csv_path: &String) -> Result<Vec<SccmHost>, Box<dyn Error>> {
    let mut gua_connections: Vec<SccmHost> = Vec::new();
    let reader = csv::Reader::from_path(csv_path);
    for record in reader?.deserialize() {
        gua_connections.push(record?);
    }
    return Ok(gua_connections);
}

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

        let rdp_attributes_array: [String; 7] = tokio::task::spawn_blocking(move || {
            //let rdp_attributes: [String; 7]  = thread::spawn(move || {
            let rdp_attrs: [String; 7] =
                get_gua_connection_details(gua_addr, gua_tkn, &conn_id).unwrap();
            rdp_attrs
        })
        .await
        //.join()
        .unwrap();
        //println!("{:?}", rdp_attributes);
        //println!("{} {} {} {} {}", rdp_attributes[0], rdp_attributes[1], rdp_attributes[2], rdp_attributes[3], rdp_attributes[4]);
        let protocol: String = raw_conn["protocol"].as_str().unwrap().to_string();

        match protocol.as_str() {
            _ if protocol.as_str() == "rdp" => {
                let proto_attributes: ProtoBasedAttributes =
                    ProtoBasedAttributes::RDP(GuaRDPattributes {
                        hostname: rdp_attributes_array[0].clone(),
                        port: rdp_attributes_array[1].clone(),
                        username: rdp_attributes_array[2].clone(),
                        domain: rdp_attributes_array[3].clone(),
                        ignore_cert: rdp_attributes_array[4].clone(),
                        wol_send_packet: rdp_attributes_array[5].clone(),
                        wol_mac_addr: rdp_attributes_array[6].clone(),
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
                let proto_attributes: ProtoBasedAttributes =
                    ProtoBasedAttributes::VNC(GuaVNCattributes {});
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

#[tokio::main]
pub async fn get_gua_connection_details(
    gua_address: Arc<String>,
    gua_token: Arc<String>,
    conn_id: &String,
) -> Result<[String; 7], Box<dyn Error>> {
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

    let mut domain: String = String::new();
    match resp_json["domain"].as_str() {
        None => domain.push_str("None"),
        Some(x) => domain.push_str(x),
    }

    let mut hostname: String = String::new();
    match resp_json["hostname"].as_str() {
        None => hostname.push_str("None"),
        Some(x) => hostname.push_str(x),
    }

    let mut ignore_cert: String = String::new();
    match resp_json["ignore-cert"].as_str() {
        None => ignore_cert.push_str("None"),
        Some(x) => ignore_cert.push_str(x),
    }

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

    let conn_parameters: [String; 7] = [
        hostname,
        port,
        username,
        domain,
        ignore_cert,
        wol_send_packet,
        wol_mac_addr,
    ];
    return Ok(conn_parameters);
}

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

#[tokio::main]
pub async fn create_gua_connection(
    gua_address: &String,
    gua_token: &String,
    sccm_host: &SccmHost,
    conn_grp_id: &String,
) -> Result<(), Box<dyn Error>> {
    let mut conn_user: String = String::new();
    //if sccm_host.username != "NO USER" {
    //    let vec_user: &Vec<&str> = &sccm_host.username.split("\\").collect();
    //    conn_user = format!("{}\\\\{}", vec_user[0], vec_user[1]);
    //}
    if sccm_host.username != "no user" {
        //let vec_user: &Vec<&str> = &sccm_host.username.split("\\").collect();
        conn_user = sccm_host.username.clone();
    }

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, format!("application/json").parse().unwrap());

    let request_data = format!(
        r#"{{
    "parentIdentifier": "{}",
    "name": "{}",
    "protocol": "rdp",
    "parameters": {{
    "port": "3389",
    "read-only": "",
    "swap-red-blue": "",
    "cursor": "",
    "color-depth": "",
    "clipboard-encoding": "",
    "disable-copy": "",
    "disable-paste": "",
    "dest-port": "",
    "recording-exclude-output": "",
    "recording-exclude-mouse": "",
    "recording-include-keys": "",
    "create-recording-path": "",
    "enable-sftp": "",
    "sftp-port": "",
    "sftp-server-alive-interval": "",
    "enable-audio": "",
    "security": "",
    "disable-auth": "",
    "ignore-cert": "true",
    "gateway-port": "",
    "server-layout": "",
    "timezone": "",
    "console": "",
    "width": "",
    "height": "",
    "dpi": "",
    "resize-method": "",
    "console-audio": "",
    "disable-audio": "",
    "enable-audio-input": "",
    "enable-printing": "",
    "enable-drive": "",
    "create-drive-path": "",
    "enable-wallpaper": "",
    "enable-theming": "",
    "enable-font-smoothing": "",
    "enable-full-window-drag": "",
    "enable-desktop-composition": "",
    "enable-menu-animations": "",
    "disable-bitmap-caching": "",
    "disable-offscreen-caching": "",
    "disable-glyph-caching": "",
    "preconnection-id": "",
    "hostname": "{}",
    "username": "{}",
    "password": "",
    "domain": "developex",
    "gateway-hostname": "",
    "gateway-username": "",
    "gateway-password": "",
    "gateway-domain": "",
    "initial-program": "",
    "client-name": "",
    "printer-name": "",
    "drive-name": "",
    "drive-path": "",
    "static-channels": "",
    "remote-app": "",
    "remote-app-dir": "",
    "remote-app-args": "",
    "preconnection-blob": "",
    "load-balance-info": "",
    "recording-path": "",
    "recording-name": "",
    "sftp-hostname": "",
    "sftp-host-key": "",
    "sftp-username": "",
    "sftp-password": "",
    "sftp-private-key": "",
    "sftp-passphrase": "",
    "sftp-root-directory": "",
    "sftp-directory": "",
    "wol-send-packet": "true",
    "wol-mac-addr": "{}"
    }},"attributes": {{
    "max-connections": "",
    "max-connections-per-user": "",
    "weight": "",
    "failover-only": "",
    "guacd-port": "",
    "guacd-encryption": "",
    "guacd-hostname": ""
    }}
        }}"#,
        conn_grp_id, sccm_host.hostname, sccm_host.ipv4, conn_user, sccm_host.mac
    );

    let client = reqwest::Client::new();

    let _resp = client
        .post(format!(
            "{}{}?token={}",
            gua_address, GUA_REST_CONNECTIONS, gua_token
        ))
        .headers(headers.clone())
        .body(request_data)
        .send()
        .await?
        .text()
        .await?;

    return Ok(());
}

#[tokio::main]
pub async fn update_gua_connection(
    gua_address: &String,
    gua_token: &String,
    sccm_host: &SccmHost,
    conn_id: &String,
    conn_grp_id: &String,
) -> Result<(), Box<dyn Error>> {
    let mut conn_user: String = String::new();
    if sccm_host.username != "no user" {
        conn_user = sccm_host.username.clone();
    }

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, format!("application/json").parse().unwrap());

    let request_data = format!(
        r#"{{
    "parentIdentifier": "{}",
    "name": "{}",
    "protocol": "rdp",
    "parameters": {{
    "port": "3389",
    "read-only": "",
    "swap-red-blue": "",
    "cursor": "",
    "color-depth": "",
    "clipboard-encoding": "",
    "disable-copy": "",
    "disable-paste": "",
    "dest-port": "",
    "recording-exclude-output": "",
    "recording-exclude-mouse": "",
    "recording-include-keys": "",
    "create-recording-path": "",
    "enable-sftp": "",
    "sftp-port": "",
    "sftp-server-alive-interval": "",
    "enable-audio": "",
    "security": "",
    "disable-auth": "",
    "ignore-cert": "true",
    "gateway-port": "",
    "server-layout": "",
    "timezone": "",
    "console": "",
    "width": "",
    "height": "",
    "dpi": "",
    "resize-method": "",
    "console-audio": "",
    "disable-audio": "",
    "enable-audio-input": "",
    "enable-printing": "",
    "enable-drive": "",
    "create-drive-path": "",
    "enable-wallpaper": "",
    "enable-theming": "",
    "enable-font-smoothing": "",
    "enable-full-window-drag": "",
    "enable-desktop-composition": "",
    "enable-menu-animations": "",
    "disable-bitmap-caching": "",
    "disable-offscreen-caching": "",
    "disable-glyph-caching": "",
    "preconnection-id": "",
    "hostname": "{}",
    "username": "{}",
    "password": "",
    "domain": "developex",
    "gateway-hostname": "",
    "gateway-username": "",
    "gateway-password": "",
    "gateway-domain": "",
    "initial-program": "",
    "client-name": "",
    "printer-name": "",
    "drive-name": "",
    "drive-path": "",
    "static-channels": "",
    "remote-app": "",
    "remote-app-dir": "",
    "remote-app-args": "",
    "preconnection-blob": "",
    "load-balance-info": "",
    "recording-path": "",
    "recording-name": "",
    "sftp-hostname": "",
    "sftp-host-key": "",
    "sftp-username": "",
    "sftp-password": "",
    "sftp-private-key": "",
    "sftp-passphrase": "",
    "sftp-root-directory": "",
    "sftp-directory": "",
    "wol-send-packet": "true",
    "wol-mac-addr": "{}"
    }},"attributes": {{
    "max-connections": "",
    "max-connections-per-user": "",
    "weight": "",
    "failover-only": "",
    "guacd-port": "",
    "guacd-encryption": "",
    "guacd-hostname": ""
    }}
        }}"#,
        conn_grp_id, sccm_host.hostname, sccm_host.ipv4, conn_user, sccm_host.mac
    );

    //let request_data = format!(
    //    r#"{{"parentIdentifier": "ROOT",
    //"name": "{}",
    //"protocol": "rdp",
    //"parameters": {{
    //"port": "3389",
    //"ignore-cert": "true",
    //"hostname": "{}",
    //"username": "{}",
    //"domain": "developex",
    //"wol-send-packet": "true",
    //"wol-mac-addr": "{}"
    //}}
    //}}"#,
    //    sccm_host.hostname, sccm_host.ipv4, conn_user, sccm_host.mac
    //);

    let client = reqwest::Client::new();

    let _resp = client
        .put(format!(
            "{}{}/{}?token={}",
            gua_address, GUA_REST_CONNECTIONS, conn_id, gua_token
        ))
        .headers(headers.clone())
        .body(request_data)
        .send()
        .await?
        .text()
        .await?;

    return Ok(());
}

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
