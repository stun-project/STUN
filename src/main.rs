extern crate tokio;
mod stunserver;

use std::env;
use std::error::Error;
use stunserver::{parse_program_arguments, StunServerBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server_args = parse_program_arguments(env::args().collect());

    let server = StunServerBuilder::build(server_args.0, server_args.1).await?;

    server.run().await?;

    Ok(())
}
