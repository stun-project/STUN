# STUN-server

STUN server written in rust, it uses the tokio library to obtain a asynchronous thread model. It is made to follow the specifications of the RFC-5389, but is not yet complete.

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
- Tokio  
&nbsp;&nbsp;&nbsp; SKRIV HER SIGMUND
- Byteorder  
&nbsp;&nbsp;&nbsp;A library that assists with the reading and writing of bytes to buffers. Used mainly for stun-message encoding and decoding. 

## Setup
SKRIV HER SIGMUND

## Testing
SKRIV HER SIGMUND  
Latest CI/CD job:  
---

## Todo

### Protocols

- [ ] Tcp listener
- [ ] Udp listener

### Errors

- [ ] Try Alternate
- [ ] Bad Request
- [ ] Unauthorized
- [ ] Unknown Attribute
- [ ] Stale Nonce
- [ ] ServerError

### Attributes

- [ ] Alternate Server
- [ ] Error-Code
- [ ] FingerPrint
- [ ] Mapped Address
- [ ] Message-integrity
- [ ] Nonce
- [ ] Realm
- [ ] Software
- [ ] Uknown-Attributes
- [ ] Username
- [ ] XOR-Mapped-Address

### Binding requests

- [ ] Binding Support

### Operation

- [ ] XOR address encode and decode operations

### Transaction id's

- [ ] Transaction id's
