///// local modules
pub use crate::conf::GUA_REST_CONNECTIONS;
pub use crate::structures::host::Host;

///// external crates
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use std::error::Error;

#[tokio::main]
pub async fn create_gua_vnc_connection(
    gua_address: &String,
    gua_token: &String,
    input_host: &Host,
    conn_grp_id: &String,
) -> Result<(), Box<dyn Error>> {
    let mut conn_user: String = String::new();
    //if sccm_host.username != "NO USER" {
    //    let vec_user: &Vec<&str> = &sccm_host.username.split("\\").collect();
    //    conn_user = format!("{}\\\\{}", vec_user[0], vec_user[1]);
    //}
    if input_host.username != "no user" {
        //let vec_user: &Vec<&str> = &sccm_host.username.split("\\").collect();
        conn_user = input_host.username.clone();
    }

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, format!("application/json").parse().unwrap());

    let request_data = format!(
        r#"{{
    "parentIdentifier": "{}",
    "name": "{}",
    "protocol": "vnc",
    "parameters": {{
    "port": "5900",
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
    "audio-servername": "",
    "sftp-hostname": "",
    "sftp-host-key": "",
    "sftp-username": "",
    "sftp-password": "",
    "sftp-private-key": "",
    "sftp-passphrase": "",
    "sftp-root-directory": "",
    "sftp-directory": "",
    "recording-path": "",
    "recording-name": "",
    "dest-host": "",
    "hostname": "{}",
    "username": "{}",
    "password": "",
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
        conn_grp_id, input_host.hostname, input_host.ipv4, conn_user, input_host.mac
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
