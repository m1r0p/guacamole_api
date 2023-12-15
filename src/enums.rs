use crate::structures::guaconn::{GuaRDPattributes, GuaVNCattributes};

#[derive(Debug)]
pub enum ProtoBasedAttributes {
    RDP(GuaRDPattributes),
    VNC(GuaVNCattributes),
}
