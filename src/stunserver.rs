extern crate tokio;
use crate::handlers::handle_message;
use async_trait::async_trait;
use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream, UdpSocket};

#[async_trait]
pub trait StunServer {
    async fn run(&self) -> Result<(), Box<dyn Error>>;
}

struct TcpStunServer {
    _server_address: SocketAddr,
    tcp_socket: TcpListener,
}

#[async_trait]
impl StunServer for TcpStunServer {
    async fn run(&self) -> Result<(), Box<dyn Error>> {
        loop {
            tokio::select! {
                tcp_stream = self.tcp_socket.accept() => {
                    match tcp_stream {
                        Ok(stream) => {
                            tokio::spawn(async move {
                                println!("Accepted connection from {}", &stream.1);
                                if let Err(e) = handle_tcp_connection(stream.0).await {
                                    println!("an error occurred; error = {:?}", e);
                                }
                            });
                        },
                        Err(e) => println!("{:?}", e),
                    };
                }
            }
        }
    }
}

struct UdpStunServer {
    _server_address: SocketAddr,
    udp_socket: UdpSocket,
}

#[async_trait]
impl StunServer for UdpStunServer {
    async fn run(&self) -> Result<(), Box<dyn Error>> {
        loop {
            let mut buffer = [0 as u8; 1024];
            tokio::select! {
                udp_message = self.udp_socket.recv_from(&mut buffer) => {
                    match udp_message {
                        Ok(message) => {
                            tokio::spawn(async move {
                                println!("Accepted connection from {}, received {} bytes", &message.1, &message.0);
                                if let Err(e) = handle_udp_connection(&buffer, message.0, message.1).await {
                                    println!("an error occurred; error = {:?}", e);
                                }
                            });
                        },
                        Err(e) => println!("{:?}", e),
                    }
                },
            }
        }
    }
}

struct MultiplexedStunServer {
    _server_address: SocketAddr,
    udp_socket: UdpSocket,
    tcp_socket: TcpListener,
}

#[async_trait]
impl StunServer for MultiplexedStunServer {
    async fn run(&self) -> Result<(), Box<dyn Error>> {
        loop {
            let mut buffer = [0 as u8; 1024];
            tokio::select! {
                udp_message = self.udp_socket.recv_from(&mut buffer) => {
                    match udp_message {
                        Ok(message) => {
                            tokio::spawn(async move {
                                println!("Accepted connection from {}, received {} bytes", &message.1, &message.0);
                                if let Err(e) = handle_udp_connection(&buffer, message.0, message.1).await {
                                    println!("an error occurred; error = {:?}", e);
                                }
                            });
                        },
                        Err(e) => println!("{:?}", e),
                    }
                }
                tcp_stream = self.tcp_socket.accept() => {
                    match tcp_stream {
                        Ok(stream) => {
                            tokio::spawn(async move {
                                println!("Accepted connection from {}", &stream.1);
                                if let Err(e) = handle_tcp_connection(stream.0).await {
                                    println!("an error occurred; error = {:?}", e);
                                }
                            });
                        },
                        Err(e) => println!("{:?}", e),
                    };
                },
            }
        }
    }
}

pub enum StunServerEnum {
    TcpStunServer,
    UdpStunServer,
    MultiplexedStunServer,
}

pub struct StunServerBuilder {}

impl StunServerBuilder {
    pub async fn build(
        server_address: SocketAddr,
        servertype: StunServerEnum,
    ) -> Result<Box<dyn StunServer>, Box<dyn Error>> {
        match servertype {
            StunServerEnum::TcpStunServer => {
                return Ok(StunServerBuilder::build_tcp_server(server_address).await?);
            }
            StunServerEnum::UdpStunServer => {
                return Ok(StunServerBuilder::build_udp_server(server_address).await?);
            }
            StunServerEnum::MultiplexedStunServer => {
                return Ok(StunServerBuilder::build_multiplexed_server(server_address).await?);
            }
        }
    }

    async fn build_tcp_server(
        server_address: SocketAddr,
    ) -> Result<Box<dyn StunServer>, Box<dyn Error>> {
        let tcp_listener = TcpListener::bind(server_address).await?;

        let tcp_server = TcpStunServer {
            _server_address: server_address,
            tcp_socket: tcp_listener,
        };

        Ok(Box::new(tcp_server))
    }
    async fn build_udp_server(
        server_address: SocketAddr,
    ) -> Result<Box<dyn StunServer>, Box<dyn Error>> {
        let udp_socket = UdpSocket::bind(server_address).await?;

        let udp_server = UdpStunServer {
            _server_address: server_address,
            udp_socket: udp_socket,
        };

        Ok(Box::new(udp_server))
    }
    async fn build_multiplexed_server(
        server_address: SocketAddr,
    ) -> Result<Box<dyn StunServer>, Box<dyn Error>> {
        let udp_socket = UdpSocket::bind(server_address).await?;
        let tcp_listener = TcpListener::bind(server_address).await?;

        let multiplexed_stun_server = MultiplexedStunServer {
            _server_address: server_address,
            tcp_socket: tcp_listener,
            udp_socket: udp_socket,
        };
        Ok(Box::new(multiplexed_stun_server))
    }
}

async fn handle_tcp_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0 as u8; 1024];
    stream.readable().await?;
    let length = stream.read(&mut buffer).await?;
    println!("{}", String::from_utf8_lossy(&buffer[..length]));
    match stream.peer_addr() {
        Ok(a) => {
            handle_message(&buffer, a.port(), a.ip());
        }
        Err(e) => panic!(e),
    };
    Ok(())
}

async fn handle_udp_connection(
    buffer: &[u8; 1024],
    message_len: usize,
    address: SocketAddr,
) -> Result<(), Box<dyn Error>> {
    println!("{:?}", &buffer[..message_len]);
    let _message = handle_message(&buffer[..message_len], address.port(), address.ip()); //pase address, ta imot address
    Ok(())
}

pub fn parse_program_arguments(input: Vec<String>) -> (SocketAddr, StunServerEnum) {
    match input.len() {
        1 => {
            println!("No arguments passed, running on localhost");
            return (
                SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3478),
                StunServerEnum::MultiplexedStunServer,
            );
        }
        2 => {
            let parsed_address = &input[1];
            let address = IpAddr::V4(Ipv4Addr::from_str(parsed_address).unwrap());

            println!("Trying to bind address {}:3478", &parsed_address);
            return (
                SocketAddr::new(address, 3478),
                StunServerEnum::MultiplexedStunServer,
            );
        }
        3 => {
            let parsed_address = &input[1];
            let address = IpAddr::V4(Ipv4Addr::from_str(parsed_address).unwrap());

            let parsed_port = &input[2];
            let port = parsed_port.parse::<u16>().unwrap();

            println!("Trying to bind address {}:{}", &parsed_address, &port);
            return (
                SocketAddr::new(address, port),
                StunServerEnum::MultiplexedStunServer,
            );
        }
        4 => {
            let parsed_address = &input[1];
            let address = IpAddr::V4(Ipv4Addr::from_str(parsed_address).unwrap());

            let parsed_port = &input[2];
            let port = parsed_port.parse::<u16>().unwrap();

            let parsed_protocol = &input[3];

            println!(
                "Trying to bind address {}:{} with protocol: {} ",
                &parsed_address, &port, &parsed_protocol
            );

            match parsed_protocol.as_str() {
                "multiplex" => {
                    return (
                        SocketAddr::new(address, port),
                        StunServerEnum::MultiplexedStunServer,
                    );
                }
                "tcp" => {
                    return (
                        SocketAddr::new(address, port),
                        StunServerEnum::TcpStunServer,
                    );
                }
                "udp" => {
                    return (
                        SocketAddr::new(address, port),
                        StunServerEnum::UdpStunServer,
                    );
                }
                _ => {
                    println!("{}, what da fuck even is this??", &parsed_protocol);
                    return (
                        SocketAddr::new(address, port),
                        StunServerEnum::MultiplexedStunServer,
                    );
                }
            }
        }

        _ => {
            return (
                SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3478),
                StunServerEnum::MultiplexedStunServer,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(1, 1);
    }
}
