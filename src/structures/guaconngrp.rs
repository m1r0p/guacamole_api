#[derive(Debug)]
pub struct GuaConnGrpAttributes {
    pub max_connections: String,
    pub max_connections_per_user: String,
    pub enable_session_affinity: String,
}

#[derive(Debug)]
pub struct GuaConnGrp {
    pub name: String,
    pub identifier: String,
    pub parent_identifier: String,
    pub conn_grp_type: String,
    pub active_connections: u64,
    pub attributes: GuaConnGrpAttributes,
}
