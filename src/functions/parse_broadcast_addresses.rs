use serde_json::Value;
use std::error::Error;
use std::collections::HashMap;

pub fn parse_broadcast_addresses(
    address_string: String,
) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut addresses: HashMap<String, String> = HashMap::new();

    let mut jsn: Value = serde_json::from_str(address_string.as_str()).unwrap();

    println!("{:?}", &jsn);

    return Ok(addresses);
}
