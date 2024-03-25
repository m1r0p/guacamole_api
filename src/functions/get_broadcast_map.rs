use ipnet::Ipv4Net;
use serde_json::{Map, Value};
use std::collections::HashMap;

pub fn get_broadcast_map(address_string: String) -> HashMap<Ipv4Net, String> {
    let mut addresses: HashMap<Ipv4Net, String> = HashMap::new();

    let jsn: Value = serde_json::from_str(address_string.as_str()).unwrap();
    let jsn_obj: &Map<String, Value> = jsn.as_object().unwrap();
    for (k, v) in jsn_obj.iter() {
        addresses.insert(k.parse().unwrap(), v.to_string());
    }

    return addresses;
}
