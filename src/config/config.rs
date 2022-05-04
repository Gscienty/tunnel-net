use super::TunnelNetProtocol;
pub use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "tunnel-net")]
pub struct TunnelNetConfig {
    #[structopt(short, long, help = "server mode")]
    server_mode: bool,

    #[structopt(short, long, help = "client global mode")]
    global_mode: bool,

    #[structopt(short = "n", long, default_value = "utun10", help = "dev name")]
    dev_name: String,

    #[structopt(
        short,
        long,
        default_value = "172.16.0.10/24",
        help = "tun / tap interface CIDR"
    )]
    cidr: String,

    #[structopt(long, default_value = "8.8.8.8:53", help = "dns address")]
    dns: String,

    #[structopt(long, default_value = "172.16.0.10", help = "darwin os tun / tap ip")]
    darwin_vpn_ip: String,

    #[structopt(
        long,
        default_value = "172.16.0.1",
        help = "darwin os tun / tap gateway"
    )]
    darwin_vpn_gateway: String,

    #[structopt(short, long, default_value = "1500", help = "tun / tap MTU")]
    mtu: u32,

    #[structopt(short, long, default_value = "127.0.0.1:3000", help = "local address")]
    local_addr: String,

    #[structopt(
        short = "S",
        long,
        default_value = "127.0.0.1:3001",
        help = "server address"
    )]
    server_addr: String,

    #[structopt(
        short,
        long,
        case_insensitive = true,
        default_value = "tcp",
        help = "protocol tcp / udp / web-socket"
    )]
    protocol: TunnelNetProtocol,
}

impl TunnelNetConfig {
    pub fn get_protocol(&self) -> TunnelNetProtocol {
        self.protocol
    }

    pub fn get_mtu(&self) -> u32 {
        self.mtu
    }

    pub fn get_cidr(&self) -> &str {
        &self.cidr
    }

    pub fn get_dev_name(&self) -> &str {
        &self.dev_name
    }

    pub fn get_server_addr(&self) -> &str {
        &self.server_addr
    }

    pub fn get_local_addr(&self) -> &str {
        &self.local_addr
    }

    pub fn get_server_mode(&self) -> bool {
        self.server_mode
    }

    pub fn get_global_mode(&self) -> bool {
        self.global_mode
    }

    pub fn get_darwin_vpn_ip(&self) -> &str {
        &self.darwin_vpn_ip
    }

    pub fn get_darwin_vpn_gateway(&self) -> &str {
        &self.darwin_vpn_gateway
    }

    pub fn get_dns(&self) -> &str {
        &self.dns
    }
}
