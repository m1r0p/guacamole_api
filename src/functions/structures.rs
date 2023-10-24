//////// structures
#[allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GuaConn {
    pub hostname: String,
    pub username: String,
    pub ipv4: String,
    pub mac: String,
}
