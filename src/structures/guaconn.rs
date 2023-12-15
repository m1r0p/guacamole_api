pub use crate::enums::ProtoBasedAttributes;

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
    pub proto_based_attributes: ProtoBasedAttributes,
}

#[derive(Debug)]
pub struct GuaRDPattributes {
    pub hostname: String,
    pub port: String,
    pub username: String,
    pub domain: String,
    pub ignore_cert: String,
    pub wol_send_packet: String,
    pub wol_mac_addr: String,
}

#[derive(Debug)]
pub struct GuaVNCattributes {}
