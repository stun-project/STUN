extern crate tokio;

use async_trait::async_trait;
use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::str::FromStr;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream, UdpSocket};

#[async_trait]
pub trait StunServer {
    async fn run(&self) -> Result<(), Box<dyn Error>>;
}

struct TcpStunServer {
    server_address: SocketAddr,
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
    server_address: SocketAddr,
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
                                if let Err(e) = handle_udp_connection(&buffer, message.0).await {
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
    server_address: SocketAddr,
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
                                if let Err(e) = handle_udp_connection(&buffer, message.0).await {
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
        let tcp_listener = TcpListener::bind("192.168.1.112:3478").await?;

        let tcp_server = TcpStunServer {
            server_address: server_address,
            tcp_socket: tcp_listener,
        };

        Ok(Box::new(tcp_server))
    }
    async fn build_udp_server(
        server_address: SocketAddr,
    ) -> Result<Box<dyn StunServer>, Box<dyn Error>> {
        let udp_socket = UdpSocket::bind("192.168.1.112:3478").await?;

        let udp_server = UdpStunServer {
            server_address: server_address,
            udp_socket: udp_socket,
        };

        Ok(Box::new(udp_server))
    }
    async fn build_multiplexed_server(
        server_address: SocketAddr,
    ) -> Result<Box<dyn StunServer>, Box<dyn Error>> {
        let udp_socket = UdpSocket::bind("192.168.1.112:3478").await?;
        let tcp_listener = TcpListener::bind("192.168.1.112:3478").await?;

        let multiplexed_stun_server = MultiplexedStunServer {
            server_address: server_address,
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
    Ok(())
}

async fn handle_udp_connection(
    buffer: &[u8; 1024],
    message_len: usize,
) -> Result<(), Box<dyn Error>> {
    println!("{}", String::from_utf8_lossy(&buffer[..message_len]));
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

            println!("Trying to bind address {}: 3478", &parsed_address);
            return (
                SocketAddr::new(address, 3478),
                StunServerEnum::MultiplexedStunServer,
            );
        }
        3 => {
            let parsed_address = &input[1];
            let address = IpAddr::V4(Ipv4Addr::from_str(parsed_address).unwrap());

            let parsed_port = &input[1];
            let port = parsed_port.parse::<u16>().unwrap();

            println!("Trying to bind address {}:{}", &parsed_address, &port);
            return (
                SocketAddr::new(address, port),
                StunServerEnum::MultiplexedStunServer,
            );
        }

        _ => {
            return (
                SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3478),
                StunServerEnum::MultiplexedStunServer,
            );
        }
    }
}
