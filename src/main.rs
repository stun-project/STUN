extern crate tokio;

use std::error::Error;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("localhost:6969").await?;
    let state = Arc::new(Mutex::new(String::new()));

    loop {
        let (stream, client_adress) = listener.accept().await?;
        let state = Arc::clone(&state);
        tokio::spawn(async move {
            println!("Accepted connection from {}", &client_adress);

            if let Err(e) = handle_ws_connection().await {
                println!("an error occurred; error = {:?}", e);
            }
        });
    }
}

async fn handle_ws_connection() -> Result<(), Box<dyn Error>> {
    Ok(())
}
