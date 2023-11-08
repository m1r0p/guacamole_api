#[allow(dead_code)]
pub mod conf;
pub use conf::{GUA_REST_CONNECTIONS, GUA_REST_TOKENS};
pub mod structures;
pub use structures::{GuaConn, GuaConnAttributes, SccmHost};

use config::{Config, File, FileFormat};
use csv;
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use serde_json::{Map, Value};
use std::error::Error;

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
    let client = reqwest::Client::new();

    let resp = client
        .get(format!(
            "{}{}?token={}",
            gua_address, GUA_REST_CONNECTIONS, gua_token
        ))
        .send()
        .await?
        .text()
        .await?;
    let resp_json: Value = serde_json::from_str(resp.as_str()).unwrap();
    let conn_obj_json: &Map<String, Value> = resp_json.as_object().unwrap();
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

        let conn: GuaConn = GuaConn {
            active_connections: raw_conn["activeConnections"].as_u64().unwrap(),
            attributes: attributes,
            identifier: raw_conn["identifier"].as_str().unwrap().to_string(),
            name: raw_conn["name"].as_str().unwrap().to_string(),
            parent_identifier: raw_conn["parentIdentifier"].as_str().unwrap().to_string(),
            protocol: raw_conn["protocol"].as_str().unwrap().to_string(),
        };

        conn_list.push(conn);
    }

    return Ok(conn_list);
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
        r#"{{"parentIdentifier": "ROOT",
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
"sftp-directory": ""
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
        sccm_host.hostname, sccm_host.ipv4, conn_user
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
) -> Result<(), Box<dyn Error>> {
    let mut conn_user: String = String::new();
    if sccm_host.username != "no user" {
        conn_user = sccm_host.username.clone();
    }

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, format!("application/json").parse().unwrap());

    let request_data = format!(
        r#"{{"parentIdentifier": "ROOT",
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
"sftp-directory": ""
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
        sccm_host.hostname, sccm_host.ipv4, conn_user
    );

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
