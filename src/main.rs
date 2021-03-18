extern crate tokio;

use std::error::Error;
// use std::sync::Arc;
use crate::tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream, UdpSocket};
// use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let tcp_listener = TcpListener::bind("localhost:3478").await?;

    let udp_listener = UdpSocket::bind("localhost:3478").await?;

    //let state = Arc::new(Mutex::new(String::new()));

    loop {
        let mut buffer = [0 as u8; 1024];
        tokio::select! {
            udp_message = udp_listener.recv_from(&mut buffer) => {
                match udp_message {
                    Ok(message) => {
                        tokio::spawn(async move {
                            println!("Accepted connection from {}, received {} bytes", &message.1, &message.0);
                            println!("{}", String::from_utf8_lossy(&buffer[..]));

                        });
                    },
                    Err(e) => println!("{:?}", e),
                }
            }
            tcp_stream = tcp_listener.accept() => {
                match tcp_stream {
                    Ok(stream) => {
                        tokio::spawn(async move {
                            println!("Accepted connection from {}", &stream.1);
                            if let Err(e) = handle_ws_connection(stream.0).await {
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

async fn handle_ws_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0 as u8; 1024];
    stream.readable().await?;
    let length = stream.read(&mut buffer).await?;
    println!("{}", String::from_utf8_lossy(&buffer[..length]));
    Ok(())
}
