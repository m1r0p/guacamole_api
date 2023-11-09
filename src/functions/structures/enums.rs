use crate::structures::{GuaRDPattributes, GuaVNCattributes};

#[derive(Debug)]
pub enum ProtoBasedAttributes {
    RDP(GuaRDPattributes),
    VNC(GuaVNCattributes),
}
