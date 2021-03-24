# STUN-server

STUN server written in rust, it uses the tokio for a asynchronous runtime. It is made to follow the specifications of the RFC-5389, but is not yet complete.

## Implemented functionality

- Asynchronous request handling
- Basic message validity check
- Error-reply in certain cases
- Correct unauthenticated binding request handling
- TCP and UDP support
- Dynamic configuration of listening port, address and protocol from CLI

## Future work

- Authentication
- Handling of binding indication
- Further error handling as more functionality is added
- Further test-implementation
- Support for more Attributes (nonce, realm, software, username, message-integrity, fingerprint, alternate-server)
- TLS support

## Dependencies

- Tokio and threading model
  &nbsp;&nbsp;&nbsp; Tokio is a library in rust for enabling async non-blocking IO operations. Tokio is the main dependency of this project and defines how many of the operations are performed. Rust is built with as little overhead as possible, and has a "You don't pay for what you don't use" mentality. This means that rust does not offer asynchronous operations by default because it does add overhead. In cases where you want the benefits of async operations, you need a runtime like tokio. In web applications, most of the "work" that has to be done, is simply waiting for a peer to respond. If the IO is blocking, it will waste a significant amount of cpu cycles waiting for peers. The Tcp and Udp connections in this stun are handled asynchronously by the Tokio runtime. The can be configured to run in multiplex mode(default), in which it listens to the Udp socket and TcpSocket at the same time. Tokio implements an event loop, and a work-stealing scheduler. It does utilize a thread pool, but only to the extent of the physical cores of the system. Tcp connections are handled like tasks, which means they can be handled concurrently and asynchonously because the tcp connection actually has a "lifetime" compared to the udp protocol. The work done handling a single udp request is too small to warrant pushing it as a unit of work to the scheduler. This means that every udp REQUEST(still listening asynchronously on the socket) is handled synchronously, while every Tcp-stream is handled as a unit of work. The server is currently capable of handling around 1000 udp requests every second when running on a 2 core virtual machine hosted by NTNU. If we increase the amount of requests per second we run into bandwidth limitations and we start seeing dropped packets. This is performance we are very happy with, and at this point, logging might also slow down the server. There is still a possiblility for handling every udp request as a unit of work and adding a third async listener on a channel and listening to it asynchronously for udp responses that are ready to be dispatched. Each request is not bound by heavy compute, but rather the bandwhith of the IO.
- Byteorder  
  &nbsp;&nbsp;&nbsp;A library that assists with the reading and writing of bytes to buffers. Used mainly for stun-message encoding and decoding.
- aasync-trait
  &nbsp;&nbsp;&nbsp; A library for enabling traits to implement futures.

## Setup

### Cargo

To run the server, you need rust. Which can be installed with rustup: [Rustup](https://rustup.rs/).
The simplest way to run this server is using cargo. The server can be run with `cargo run` this will compile the project and run the server with default configuration. If you wish to bind the server to an address or another port, you can write it like this: `cargo run 192.168.1.112 3479` If you want to specify a port, you need to specify an address as well. Lastly, you can configure the server in tcp, udp or multiplex mode, which will listen to both tcp and udp.

Examples:
`cargo run 0.0.0.0` Defaulting to port 3478 and multiplex
`cargo run 0.0.0.0 3479` Defaulting to multiplex
`cargo run 0.0.0.0 3479 udp` No defaults

### Binary

If you want run a production build, run `cargo build`this will put the stun binary in the `target/release` folder, where you can run the run the server with: `./stun` you can specify arguments here as well.

Examples:
`./stun 0.0.0.0` Defaulting to port 3478 and multiplex
`./stun 0.0.0.0 3479` Defaulting to multiplex
`./stun 0.0.0.0 3479 udp` No defaults

### Docker and docker-compose

If you want to run the STUN server with docker, there is an image build at sigmundgranaas/stun. The Dockerfile is located in the root folder. To run it easily, use docker-compose!

to build the image locally, use: `docker-compose build`
to run with docker-compose, use: `docker-compose up`

To edit address, port and mode, edit the .env file!

## Testing

Rust has native support for writing tests into each and every file. tests can be run with `cargo test` tests are run every time you open a new pull request on github. The only way to merge code in the master branch is to open a PR, which will wait for all configured tests to run and succeed. If the test and the build succeeds, you are allowed to merge the code into the master branch. The last run test can be viewed in Github actions.

Last run test:

```
Running target/debug/deps/stun-2c14ba3c2d7d2e09

running 5 tests
test attributes::tests::test_xor_encoding_v4 ... ok
test errors::tests::test ... ok
test handlers::tests::test ... ok
test message::tests::test ... ok
test stunserver::tests::test ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Latest run CI test: view [latest_log.txt](./latest_log.txt)
