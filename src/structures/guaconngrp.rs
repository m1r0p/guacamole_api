#[derive(Debug)]
pub struct GuaConnGrp {
    pub name: String,
    pub identifier: String,
    pub parent_identifier: String,
    pub conn_grp_type: String,
    pub active_connections: u64,
    pub attributes: GuaConnGrpAttributes,
}
