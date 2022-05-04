use super::super::config::TunnelNetConfig;
use super::TunDev;
use default_net::interface;
use std::io::Error;
use std::process::Command;

pub fn setup_tun(dev: &dyn TunDev, cfg: &TunnelNetConfig) -> Result<(), Error> {
    if cfg!(target_os = "linux") {
        Command::new("/sbin/ip")
            .arg("link")
            .arg("set")
            .arg("dev")
            .arg(dev.name())
            .arg("mtu")
            .arg(cfg.get_mtu().to_string())
            .output()?;

        Command::new("/sbin/ip")
            .arg("addr")
            .arg("add")
            .arg(cfg.get_cidr())
            .arg("dev")
            .arg(dev.name())
            .output()?;

        Command::new("/sbin/ip")
            .arg("link")
            .arg("set")
            .arg("dev")
            .arg(dev.name())
            .arg("up")
            .output()?;
    } else if cfg!(target_os = "darwin") {
        Command::new("ifconfig")
            .arg(dev.name())
            .arg("inet")
            .arg(cfg.get_darwin_vpn_ip())
            .arg(cfg.get_darwin_vpn_gateway())
            .arg("up")
            .output()?;
    } else {
        return Err(Error::last_os_error());
    }

    if let Some((server_ip, _)) = cfg.get_server_addr().split_once(":") {
        let physical_ifc = interface::get_default_interface().or(Err(Error::last_os_error()))?;

        if cfg!(target_os = "linux") {
            if !cfg.get_global_mode() {
                return Ok(());
            }

            Command::new("/sbin/ip")
                .arg("route")
                .arg("add")
                .arg("0.0.0.0/1")
                .arg("dev")
                .arg(dev.name())
                .output()?;
            Command::new("/sbin/ip")
                .arg("route")
                .arg("add")
                .arg("128.0.0.0/1")
                .arg("dev")
                .arg(dev.name())
                .output()?;
            Command::new("/sbin/ip")
                .arg("route")
                .arg("add")
                .arg(vec![server_ip.to_string(), "32".to_string()].join("/"))
                .arg("via")
                .arg("gateway")
                .arg("dev")
                .arg(physical_ifc.ipv4.get(0).unwrap().addr.to_string())
                .output()?;

            if let Some((dns_ip, _)) = cfg.get_dns().split_once(":") {
                Command::new("/sbin/ip")
                    .arg("route")
                    .arg("add")
                    .arg(vec![dns_ip.to_string(), "32".to_string()].join("/"))
                    .arg("via")
                    .arg("gateway")
                    .arg("dev")
                    .arg(physical_ifc.ipv4.get(0).unwrap().addr.to_string())
                    .output()?;
            }
        } else if cfg!(target_os = "darwin") {
            let gateway = physical_ifc.gateway.unwrap();
            if cfg.get_global_mode() {
                Command::new("route")
                    .arg("add")
                    .arg(server_ip.to_string())
                    .arg(gateway.ip_addr.to_string())
                    .output()?;
                if let Some((dns_ip, _)) = cfg.get_dns().split_once(":") {
                    Command::new("route")
                        .arg("add")
                        .arg(dns_ip.to_string())
                        .arg(gateway.ip_addr.to_string())
                        .output()?;
                }
                Command::new("route")
                    .arg("add")
                    .arg("0.0.0.0/1")
                    .arg("-interface")
                    .arg(dev.name())
                    .output()?;
                Command::new("route")
                    .arg("add")
                    .arg("128.0.0.0/1")
                    .arg("-interface")
                    .arg(dev.name())
                    .output()?;
                Command::new("route")
                    .arg("add")
                    .arg("default")
                    .arg(cfg.get_darwin_vpn_gateway())
                    .output()?;
                Command::new("route")
                    .arg("add")
                    .arg("change")
                    .arg(cfg.get_darwin_vpn_gateway())
                    .output()?;
            } else {
                Command::new("route")
                    .arg("add")
                    .arg("default")
                    .arg(gateway.ip_addr.to_string())
                    .output()?;
                Command::new("route")
                    .arg("change")
                    .arg("default")
                    .arg(gateway.ip_addr.to_string())
                    .output()?;
            }
        }
    } else {
        return Err(Error::last_os_error());
    }

    Ok(())
}
