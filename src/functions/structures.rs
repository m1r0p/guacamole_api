//////// structures
#[allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GuaConn {
    pub PCSCCM: String,
    pub PrimaryUserSCCM: String,
    pub IPv4: String,
    pub MAC: String,
}
