- [ ] Try Alternate
- [ ] Bad Request
- [ ] Unauthorized
- [ ] Unknown Attribute
- [ ] Stale Nonce
- [ ] Server Error

struct BadRequest{
    pub const ERROR_CODE: u16 = 400;
}

struct Unauthorized{
    pub const ERROR_CODE: u16 = 401;
}

struct StaleNonce{
    pub const ERROR_CODE: u16 = 438;
}

struct TryAlternate {
    pub const ERROR_CODE: u16 = 300;
}

struct UnknownAttribute {
    pub const ERROR_CODE: u16 = 420;
}

struct ServerError {
    pub const ERROR_CODE: u16 = 500;
}

