///// local modules
pub use crate::structures::host::Host;

///// external crates
use std::error::Error;
use csv;

pub fn parse_csv(csv_path: &String) -> Result<Vec<Host>, Box<dyn Error>> {
    let mut gua_connections: Vec<Host> = Vec::new();
    let reader = csv::Reader::from_path(csv_path);
    for record in reader?.deserialize() {
        gua_connections.push(record?);
    }
    return Ok(gua_connections);
}
