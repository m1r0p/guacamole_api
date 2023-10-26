//////// structures
#[allow(dead_code)]
use serde::{Deserialize, Serialize};

//#[derive(Serialize, Deserialize, Debug)]
#[derive(Deserialize, Debug)]
pub struct SccmHost {
    pub hostname: String,
    pub username: String,
    pub ipv4: String,
    pub mac: String,
}

#[derive(Debug)]
pub struct GuaConnAttributes {
    pub guacd_encryption: String,
    pub failover_only: String,
    pub weight: String,
    pub max_connections: String,
    pub guacd_hostname: String,
    pub guacd_port: String,
    pub max_connections_per_user: String,
}

#[derive(Debug)]
pub struct GuaConn {
    pub name: String,
    pub identifier: String,
    pub parent_identifier: String,
    pub protocol: String,
    pub attributes: GuaConnAttributes,
    pub active_connections: u64,
}
