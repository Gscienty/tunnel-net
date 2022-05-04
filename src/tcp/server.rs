use super::super::config::TunnelNetConfig;
use super::super::tun::{create_tun, setup_tun, TunDev};
use super::super::utils::{get_ipv4_source, get_ipv4_target};
use mio::event::Event;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};
use std::collections::HashMap;
use std::io::{Error, ErrorKind, Read, Result, Write};
use std::net::SocketAddr;

const TCP_SERVER_TUN_TOKEN: Token = Token(0);
const TCP_SERVER_SERVER_TOKEN: Token = Token(1);

pub struct TunnelNetTCPServer {
    dev: Box<dyn TunDev>,
    local_addr: String,
    unique_token: Token,
    poll: Poll,

    client_map: HashMap<u32, usize>,
}

impl TunnelNetTCPServer {
    pub fn new(cfg: &TunnelNetConfig) -> Result<Self> {
        let mut server = TunnelNetTCPServer {
            dev: Box::new(create_tun(cfg.get_dev_name())?),
            local_addr: String::from(cfg.get_local_addr()),
            unique_token: Token(TCP_SERVER_SERVER_TOKEN.0 + 1),
            poll: Poll::new()?,
            client_map: HashMap::new(),
        };

        setup_tun(server.dev.as_mut(), cfg)?;

        Ok(server)
    }

    pub fn next_unique_token(&mut self) -> Token {
        let next = self.unique_token.0;
        self.unique_token.0 += 1;

        Token(next)
    }

    fn handle_to_client(
        &mut self,
        connections: &HashMap<Token, TcpStream>,
        event: &Event,
    ) -> Result<()> {
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
                if let Some(key) = get_ipv4_target(&received_data) {
                    if let Some(token_value) = self.client_map.get(&key) {
                        if let Some(mut conn) = connections.get(&Token(*token_value)) {
                            conn.write(&received_data[..bytes_read])?;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn handle_to_server(
        &mut self,
        token: &Token,
        conn: &mut TcpStream,
        event: &Event,
    ) -> Result<bool> {
        if event.is_readable() {
            let mut received_data = vec![0; 4096];
            let mut bytes_read = 0;

            loop {
                match conn.read(&mut received_data[bytes_read..]) {
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
                if let Some(key) = get_ipv4_source(&received_data) {
                    self.client_map.insert(key, token.0);

                    self.dev.write(&received_data[..bytes_read])?;
                }
            }
        };

        Ok(false)
    }

    pub fn start(&mut self) -> Result<()> {
        let addr: SocketAddr = (&self.local_addr).parse().or(Err(Error::last_os_error()))?;

        let mut server = TcpListener::bind(addr)?;

        self.poll.registry().register(
            self.dev.as_mut(),
            TCP_SERVER_TUN_TOKEN,
            Interest::READABLE,
        )?;

        self.poll
            .registry()
            .register(&mut server, TCP_SERVER_SERVER_TOKEN, Interest::READABLE)?;

        let mut connections = HashMap::new();
        let mut events = Events::with_capacity(128);
        loop {
            self.poll.poll(&mut events, None)?;

            for event in events.iter() {
                match event.token() {
                    TCP_SERVER_SERVER_TOKEN => loop {
                        let (mut conn, _) = match server.accept() {
                            Ok((conn, addr)) => (conn, addr),
                            Err(e) if e.kind() == ErrorKind::WouldBlock => {
                                break;
                            }
                            Err(e) => return Err(e),
                        };

                        let token = self.next_unique_token();
                        self.poll
                            .registry()
                            .register(&mut conn, token, Interest::READABLE)?;

                        connections.insert(token, conn);
                    },
                    TCP_SERVER_TUN_TOKEN => {
                        self.handle_to_client(&connections, event)?;
                    }
                    token => {
                        let done = if let Some(conn) = connections.get_mut(&token) {
                            self.handle_to_server(&token, conn, event)?
                        } else {
                            false
                        };
                        if done {
                            if let Some(mut conn) = connections.remove(&token) {
                                self.poll.registry().deregister(&mut conn)?;
                            }
                        }
                    }
                }
            }
        }
    }
}
