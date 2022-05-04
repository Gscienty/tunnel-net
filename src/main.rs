mod config;
mod tcp;
mod tun;
mod utils;

use std::io::Result;
use structopt::StructOpt;
use tcp::{TunnelNetTCPClient, TunnelNetTCPServer};

fn tcp_protocol(cfg: &config::TunnelNetConfig) -> Result<()> {
    if cfg.get_server_mode() {
        let mut server = TunnelNetTCPServer::new(cfg)?;

        server.start()?;
    } else {
        let mut client = TunnelNetTCPClient::new(cfg)?;

        client.start()?;
    }

    Ok(())
}

fn udp_protocol(_: &config::TunnelNetConfig) {}

fn web_socket_protocol(_: &config::TunnelNetConfig) {}

fn main() {
    let args = config::TunnelNetConfig::from_args();

    match args.get_protocol() {
        config::TunnelNetProtocol::TCP => {
            tcp_protocol(&args).expect("tcp protocol error");
        }

        config::TunnelNetProtocol::UDP => udp_protocol(&args),

        config::TunnelNetProtocol::WebSocket => web_socket_protocol(&args),
    }
}
