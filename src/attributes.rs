pub const ERROR_CODE: u16 = 0x0009;
pub const MAPPED_ADDRESS: u16 = 0x0001;
pub const USERNAME: u16 = 0x0006; 
pub const MESSAGE_INTEGRITY: u16 = 0x0008;
pub const UNKNOWN_ATTRIBUTES: u16 = 0x000A;
pub const REALM: u16 = 0x0014; 
pub const NONCE: u16 = 0x0015;
pub const XOR_MAPPED_ADDRESS: u16 = 0x0020;
pub const SOFTWARE: u16 = 0x8022; 
pub const ALTERNATE_SERVER: u16 = 0x8023;
pub const FINGERPRINT: u16 = 0x8028;

pub enum Attribute{
    ERROR_CODE(ErrorCode),
    MAPPED_ADDRESS,
    XOR_MAPPED_ADDRESS,
    UNKNOWN_ATTRIBUTES
}

struct ErrorCode{
    //generell attribute:
    type_: u16;
    length: u16;
    //Spesielt til error:
    status_code: u32;
    reason_phrase: String;
}

impl ErrorCode {
    pub fn new(status_code:u32,reason_phrase: String) -> self{
        ErrorCode{
            type_:ERROR_CODE,
            length:(4+reason_phrase.len()),
            status_code:status_code,
            reason_phrase:reason_phrase
        }
    }

    pub fn serialize(&self) {
        let mut vec = Vec::new();
        //TODO
        return vec;
    }
}

//-----

struct MappedAddress{
    //generell attribute:
    type_: u16;
    length: u16;
    //Spesielt til error:
    family: u8;
    port: u16;
    address: u128;
}

impl MappedAddress {
    pub fn new(family:u8,port: u16, address:u128) -> self{
        MappedAddress{
            type_:MAPPED_ADDRESS,
            length:(4+address.len), //Funker ikke, addressen varierer fra 32-128 bits
            family:family,
            port:port,
            address:address
        }
    }

    pub fn serialize(&self) {
        let mut vec = Vec::new();
        //TODO
        return vec;
    }
}

//-----
//TODO: alt med xor-mapped...
struct XorMappedAddress{
    //generell attribute:
    type_: u16;
    length: u16;
    //Spesielt til error:
    family: u8;
    port: u16;
    address: u128;
}

impl XorMappedAddress {
    pub fn new(family:u8,port: u16, address:u128) -> self{
        XorMappedAddress{
            type_:XOR_MAPPED_ADDRESS,
            length:(4+address.len), //Funker ikke, addressen varierer fra 32-128 bits
            family:family,
            port:port,
            address:address
        }
    }

    pub fn serialize(&self) {
        let mut vec = Vec::new();
        //TODO
        return vec;
    }
}


//-----

struct UnknownAttributes{
    //generell attribute:
    type_: u16;
    length: u16;
    //Spesielt til error:
    attributes: Vec::new();
}

impl UnknownAttributes {
    pub fn new(vec:&[u16]) -> self{
        UnknownAttributes{
            type_:UNKNOWN_ATTRIBUTES,
            length:(vec.len*2), //Funker ikke, addressen varierer fra 32-128 bits
            attributes: vec
        }
    }

    pub fn serialize(&self) {
        let mut vec = Vec::new();
        //TODO
        return vec;
    }
}

