#[allow(dead_code)]
use serde::Deserialize;

//#[derive(Serialize, Deserialize, Debug)]
#[derive(Deserialize, Debug)]
pub struct Host {
    pub hostname: String,
    pub username: String,
    pub ipv4: String,
    pub mac: String,
}

