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

//#[tokio::main]
//pub async fn get_mikrotik_leases(
//    ip: &String,
//    user: &String,
//    password: &String,
//    dhcp_server: &String,
//) -> Result<Vec<MikrotikLease>, Box<dyn Error>> {
//    let mut dhcp_leases: Vec<MikrotikLease> = Vec::new();
//    let client = reqwest::Client::new();
//    //let user_name: String = config_params[1].to_string();
//    let user_name: String = user.to_string();
//    //let password: Option<String> = Some(config_params[2].to_string());
//    let password: Option<String> = Some(password.to_string());
//
//    let resp = client
//        .get(format!("{}{}{}", MIKROTIK_PROTO, ip, MIKROTIK_DHCP_LEASES))
//        .basic_auth(user_name, password)
//        .send()
//        .await?
//        .text()
//        .await?;
//    let hosts_json: Value = serde_json::from_str(resp.as_str()).unwrap();
//    let hosts_vec: &Vec<Value> = hosts_json.as_array().unwrap();
//
//    for i in hosts_vec.iter() {
//        match i["server"].as_str() {
//            None => continue,
//            Some(x) => match x {
//                _ if x == dhcp_server.as_str() => {
//                    let mut id: String = String::new();
//                    match i[".id"].as_str() {
//                        None => id.push_str("None"),
//                        Some(x) => id.push_str(x),
//                    }
//                    let mut active_address: String = String::new();
//                    match i["active-address"].as_str() {
//                        None => active_address.push_str("None"),
//                        Some(x) => active_address.push_str(x),
//                    }
//                    let mut active_client_id: String = String::new();
//                    match i["active-client-id"].as_str() {
//                        None => active_client_id.push_str("None"),
//                        Some(x) => active_client_id.push_str(x),
//                    }
//                    let mut active_mac_address: String = String::new();
//                    match i["active-mac-address"].as_str() {
//                        None => active_mac_address.push_str("None"),
//                        Some(x) => active_mac_address.push_str(x),
//                    }
//                    let mut active_server: String = String::new();
//                    match i["active-server"].as_str() {
//                        None => active_server.push_str("None"),
//                        Some(x) => active_server.push_str(x),
//                    }
//                    let mut address: String = String::new();
//                    match i["address"].as_str() {
//                        None => address.push_str("None"),
//                        Some(x) => address.push_str(x),
//                    }
//                    let mut address_lists: String = String::new();
//                    match i["address-lists"].as_str() {
//                        None => address_lists.push_str("None"),
//                        Some(x) => address_lists.push_str(x),
//                    }
//                    let mut age: String = String::new();
//                    match i["age"].as_str() {
//                        None => age.push_str("None"),
//                        Some(x) => age.push_str(x),
//                    }
//                    let mut blocked: String = String::new();
//                    match i["blocked"].as_str() {
//                        None => blocked.push_str("None"),
//                        Some(x) => blocked.push_str(x),
//                    }
//                    let mut client_id: String = String::new();
//                    match i["client-id"].as_str() {
//                        None => client_id.push_str("None"),
//                        Some(x) => client_id.push_str(x),
//                    }
//                    let mut dhcp_option: String = String::new();
//                    match i["dhcp-option"].as_str() {
//                        None => dhcp_option.push_str("None"),
//                        Some(x) => dhcp_option.push_str(x),
//                    }
//                    let mut disabled: String = String::new();
//                    match i["disabled"].as_str() {
//                        None => disabled.push_str("None"),
//                        Some(x) => disabled.push_str(x),
//                    }
//                    let mut dynamic: String = String::new();
//                    match i["dynamic"].as_str() {
//                        None => dynamic.push_str("None"),
//                        Some(x) => dynamic.push_str(x),
//                    }
//                    let mut expires_after: String = String::new();
//                    match i["expires-after"].as_str() {
//                        None => expires_after.push_str("None"),
//                        Some(x) => expires_after.push_str(x),
//                    }
//                    let mut host_name: String = String::new();
//                    match i["host-name"].as_str() {
//                        None => host_name.push_str("None"),
//                        Some(x) => host_name.push_str(x),
//                    }
//                    let mut last_seen: String = String::new();
//                    match i["last-seen"].as_str() {
//                        None => last_seen.push_str("None"),
//                        Some(x) => last_seen.push_str(x),
//                    }
//                    let mut mac_address: String = String::new();
//                    match i["mac-address"].as_str() {
//                        None => mac_address.push_str("None"),
//                        Some(x) => mac_address.push_str(x),
//                    }
//                    let mut radius: String = String::new();
//                    match i["radius"].as_str() {
//                        None => radius.push_str("None"),
//                        Some(x) => radius.push_str(x),
//                    }
//                    let server: String = String::from(i["server"].as_str().unwrap());
//
//                    let mut status: String = String::new();
//                    match i["status"].as_str() {
//                        None => status.push_str("None"),
//                        Some(x) => status.push_str(x),
//                    }
//
//                    let host: MikrotikLease = MikrotikLease {
//                        id: id,
//                        active_address: active_address,
//                        active_client_id: active_client_id,
//                        active_mac_address: active_mac_address,
//                        active_server: active_server,
//                        address: address,
//                        address_lists: address_lists,
//                        age: age,
//                        blocked: blocked,
//                        client_id: client_id,
//                        dhcp_option: dhcp_option,
//                        disabled: disabled,
//                        dynamic: dynamic,
//                        expires_after: expires_after,
//                        host_name: host_name,
//                        last_seen: last_seen,
//                        mac_address: mac_address,
//                        radius: radius,
//                        server: server,
//                        status: status,
//                    };
//
//                    dhcp_leases.push(host);
//                }
//                _ => continue,
//            },
//        }
//    }
//    return Ok(dhcp_leases);
//}
//
//#[tokio::main]
//pub async fn del_phpipam_existing_hosts(
//    phpipam_address: &String,
//    token: &String,
//    subnet_id: &String,
//) -> Result<(), Box<dyn Error>> {
//    let mut headers = HeaderMap::new();
//    headers.insert("token", token.parse().unwrap());
//    let client = reqwest::Client::new();
//
//    let _resp = client
//        .delete(format!(
//            "{}{}{}/truncate",
//            phpipam_address, PHPIPAM_REST_SUBNETS, subnet_id
//        ))
//        .headers(headers.clone())
//        .send()
//        .await?
//        .text()
//        .await?;
//    return Ok(());
//}

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
    if sccm_host.username != "NO USER" {
        conn_user.push_str(&sccm_host.username.as_str());
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
"ignore-cert": "",
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
"domain": "",
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
pub async fn delete_gua_token(
    gua_address: &String,
    gua_token: &String,
) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();

    let _resp = client
        .delete(format!(
            "{}{}/{}",
            gua_address, GUA_REST_TOKENS, gua_token
        ))
        .send()
        .await?
        .text()
        .await?;
    return Ok(());
}


