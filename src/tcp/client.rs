use super::super::config::TunnelNetConfig;
use super::super::tun::{create_tun, setup_tun, TunDev};
use mio::event::Event;
use mio::net::TcpStream;
use mio::{Events, Interest, Poll, Token};
use std::io::{Error, ErrorKind, Read, Result, Write};

const TCP_CLIENT_TUN_TOKEN: Token = Token(0);
const TCP_CLIENT_SERVER_TOKEN: Token = Token(1);

pub struct TunnelNetTCPClient {
    dev: Box<dyn TunDev>,
    server_conn: TcpStream,
    poll: Poll,
}

impl TunnelNetTCPClient {
    pub fn new(cfg: &TunnelNetConfig) -> Result<Self> {
        let mut client = TunnelNetTCPClient {
            dev: Box::new(create_tun(cfg.get_dev_name())?),
            server_conn: TcpStream::connect(
                cfg.get_server_addr()
                    .parse()
                    .or(Err(Error::last_os_error()))?,
            )?,
            poll: Poll::new()?,
        };
        setup_tun(client.dev.as_mut(), cfg)?;

        Ok(client)
    }

    fn handle_to_tcp(&mut self, event: &Event) -> Result<()> {
        if event.is_readable() {
            let mut received_data = vec![0; 4096];
            let mut bytes_read = 0;

            loop {
                match self.dev.read(&mut received_data[bytes_read..]) {
                    Ok(0) => {
                        break;
                    }
                    Ok(n) => {
                        bytes_read += n;
                        if bytes_read == received_data.len() {
                            received_data.resize(received_data.len() + 1024, 0);
                        }
                    }
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => {
                        break;
                    }
                    Err(ref err) if err.kind() == ErrorKind::Interrupted => {
                        continue;
                    }
                    Err(err) => return Err(err),
                }
            }

            if bytes_read != 0 {
                self.server_conn.write(&received_data[..bytes_read])?;
            }
        }

        Ok(())
    }

    fn handle_to_tun(&mut self, event: &Event) -> Result<()> {
        if event.is_readable() {
            let mut received_data = vec![0; 4096];
            let mut bytes_read = 0;

            loop {
                match self.server_conn.read(&mut received_data[bytes_read..]) {
                    Ok(0) => {
                        break;
                    }
                    Ok(n) => {
                        bytes_read += n;
                        if bytes_read == received_data.len() {
                            received_data.resize(received_data.len() + 1024, 0);
                        }
                    }
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => {
                        break;
                    }
                    Err(ref err) if err.kind() == ErrorKind::Interrupted => {
                        continue;
                    }
                    Err(err) => return Err(err),
                }
            }

            if bytes_read != 0 {
                self.dev.write(&received_data[..bytes_read])?;
            }
        }
        Ok(())
    }

    pub fn start(&mut self) -> Result<()> {
        self.poll.registry().register(
            self.dev.as_mut(),
            TCP_CLIENT_TUN_TOKEN,
            Interest::READABLE,
        )?;

        self.poll.registry().register(
            &mut self.server_conn,
            TCP_CLIENT_SERVER_TOKEN,
            Interest::READABLE,
        )?;

        let mut events = Events::with_capacity(128);
        loop {
            self.poll.poll(&mut events, None)?;

            for event in events.iter() {
                match event.token() {
                    TCP_CLIENT_TUN_TOKEN => {
                        self.handle_to_tcp(event)?;
                    }
                    TCP_CLIENT_SERVER_TOKEN => {
                        self.handle_to_tun(event)?;
                    }
                    _ => {}
                }
            }
        }
    }
}
