#[allow(dead_code)]
//pub mod conf;
//pub use conf::{
//    MIKROTIK_DHCP_LEASES, MIKROTIK_PROTO, PHPIPAM_REST_ADDRESSES, PHPIPAM_REST_SUBNETS,
//};
pub mod structures;
pub use structures::GuaConn;

//use config::{Config, File, FileFormat};
//use reqwest::header::{HeaderMap, CONTENT_TYPE};
//use serde_json::Value;
use std::error::Error;
use csv;

//fn print_type_of<T>(_: &T) {
//    println!("{}", std::any::type_name::<T>())
//}

//pub fn get_config_params(string_path: String) -> Result<Vec<String>, Box<dyn Error>> {
//    let mut config_params: Vec<String> = Vec::new();
//
//    let mut builder = Config::builder();
//    builder = builder.set_default("default", "1")?;
//    builder = builder.add_source(File::new(&string_path, FileFormat::Json));
//    builder = builder.set_override("override", "1")?;
//    let raw_conf = builder.build().unwrap();
//    config_params.push(raw_conf.get("mikrotik_ip").unwrap());
//    config_params.push(raw_conf.get("mikrotik_user").unwrap());
//    config_params.push(raw_conf.get("mikrotik_password").unwrap());
//    config_params.push(raw_conf.get("mikrotik_dhcp_server").unwrap());
//    config_params.push(raw_conf.get("phpipam_proto_address").unwrap());
//    config_params.push(raw_conf.get("phpipam_token").unwrap());
//    config_params.push(raw_conf.get("phpipam_subnet_id").unwrap());
//    return Ok(config_params);
//}

pub fn parse_csv(csv_path: String) -> Result<Vec<GuaConn>, Box<dyn Error>> {
    let mut gua_connections: Vec<GuaConn> = Vec::new();
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
//
//#[tokio::main]
//pub async fn create_phpipam_host(
//    phpipam_address: &String,
//    token: &String,
//    subnet_id: &String,
//    ip_address: &String,
//    hostname: &String,
//    mac_address: &String,
//    status: &String,
//    dynamic: &String,
//) -> Result<(), Box<dyn Error>> {
//    let mut headers = HeaderMap::new();
//    headers.insert("token", token.parse().unwrap());
//    headers.insert(CONTENT_TYPE, format!("application/json").parse().unwrap());
//    let mut dynamic_edited: String = String::new();
//    match dynamic {
//        _ if dynamic == "true" => dynamic_edited.push_str("Dynamic"),
//        _ if dynamic == "false" => dynamic_edited.push_str("Reserved"),
//        _ => dynamic_edited.push_str("None"),
//    }
//    let mut state: u8 = 2;
//    if status == "waiting" && dynamic == "false" {
//        state = 1;
//    }
//    if status == "bound" && dynamic == "false" {
//        state = 3;
//    }
//
//    let request_data = format!(
//        r#"{{"hostname":"{}","subnetId":"{}","ip":"{}","mac":"{}","custom_Status":"{}","state":"{}","custom_Dynamic":"{}"}}"#,
//        hostname, subnet_id, ip_address, mac_address, status, state, dynamic_edited
//    );
//
//    let client = reqwest::Client::new();
//
//    let _resp = client
//        .post(format!("{}{}", phpipam_address, PHPIPAM_REST_ADDRESSES))
//        .headers(headers.clone())
//        .body(request_data)
//        .send()
//        .await?
//        .text()
//        .await?;
//    return Ok(());
//}
