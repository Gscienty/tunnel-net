use strum_macros::{EnumString, EnumVariantNames};

#[derive(Debug, Clone, Copy, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
pub enum TunnelNetProtocol {
    TCP,
    UDP,
    WebSocket,
}
