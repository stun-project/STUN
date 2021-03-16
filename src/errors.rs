- [ ] Try Alternate
- [ ] Bad Request
- [ ] Unauthorized
- [ ] Unknown Attribute
- [ ] Stale Nonce
- [ ] Server Error

struct BadRequest{
    pub const errorcode: u16 = 400;
}

struct Unauthorized{
    pub const errorcode: u16 = 401;
}

struct StaleNonce{
    pub const errorcode: u16 = 438;
}

struct TryAlternate {
    pub const errorcode: u16 = 300;
}

struct UnknownAttribute {
    pub const errorcode: u16 = 420;
}

struct ServerError {
    oub const errorcode: u16 = 500;
}

