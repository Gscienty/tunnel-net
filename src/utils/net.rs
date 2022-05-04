fn parse_ipv4(bin: &[u8]) -> u32 {
    let mut result: u32 = 0;
    for v in bin.iter() {
        result = (result << 8) | u32::from(*v);
    }

    result
}

pub fn get_ipv4_source(packet: &[u8]) -> Option<u32> {
    if let Some(bin) = packet.get(12..16) {
        Some(parse_ipv4(bin))
    } else {
        None
    }
}

pub fn get_ipv4_target(packet: &[u8]) -> Option<u32> {
    if let Some(bin) = packet.get(16..20) {
        Some(parse_ipv4(bin))
    } else {
        None
    }
}
