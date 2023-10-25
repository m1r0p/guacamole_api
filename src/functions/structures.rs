//////// structures
#[allow(dead_code)]
use serde::{Deserialize, Serialize};

//#[derive(Serialize, Deserialize, Debug)]
#[derive(Deserialize)]
pub struct SccmHost {
    pub hostname: String,
    pub username: String,
    pub ipv4: String,
    pub mac: String,
}

pub struct GuaConnAttributes {
    pub guacd_encryption: String,
    pub failover_only: String,
    pub weight: String,
    pub max_connections: String,
    pub guacd_hostname: String,
    pub guacd_port: String,
    pub max_connections_per_user: String,
}

pub struct GuaConn {
    pub name: String,
    pub identifier: String,
    pub parentIdentifier: String,
    pub protocol: String,
    pub attributes: GuaConnAttributes,
    pub activeConnections: String,
}


