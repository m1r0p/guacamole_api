///// local modules
pub use crate::conf::GUA_REST_CONNECTIONS;
pub use crate::structures::host::Host;

///// external crates
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use std::error::Error;

#[tokio::main]
pub async fn update_gua_rdp_connection(
    gua_address: &String,
    gua_token: &String,
    input_host: &Host,
    conn_id: &String,
    conn_grp_id: &String,
) -> Result<(), Box<dyn Error>> {
    let mut conn_user: String = String::new();
    if input_host.username != "no user" {
        conn_user = input_host.username.clone();
    }

    //let conn_grp_id: String = String::from("ROOT");

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
        conn_grp_id, input_host.hostname, input_host.ipv4, conn_user, input_host.mac
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
