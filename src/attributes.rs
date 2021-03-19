use std::net::{IpAddr, Ipv4Addr};
use byteorder::{BigEndian, ByteOrder};
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

pub enum AttributeEnum {
    ERROR_CODE(ErrorCode),
    MAPPED_ADDRESS(MappedAddress),
    XOR_MAPPED_ADDRESS(XorMappedAddress),
    UNKNOWN_ATTRIBUTES(UnknownAttributes),
}
impl Attribute for AttributeEnum {
    fn serialize(&self) -> Vec<u8> {
        return Vec::new();
    }
}

pub trait Attribute {
    fn serialize(&self) -> Vec<u8>;
}

pub struct ErrorCode {
    //generell attribute:
    type_: u16,
    length: u16,
    //Spesielt til error:
    status_code: u32,
    reason_phrase: String,
}

impl ErrorCode {
    pub fn new(status_code: u32, reason_phrase: String) -> Self {
        ErrorCode {
            type_: ERROR_CODE,
            length: (4 + reason_phrase.len()) as u16,
            status_code: status_code,
            reason_phrase: reason_phrase,
        }
    }
}
impl Attribute for ErrorCode {
    fn serialize(&self) -> Vec<u8> {
        let mut stun_attribute: Vec<u8> = Vec::new();

        BigEndian::write_u16(&mut stun_attribute, self.type_);
        BigEndian::write_u16(&mut stun_attribute, self.length);
        BigEndian::write_u32(&mut stun_attribute, self.status_code);
        stun_attribute.append(&mut self.reason_phrase.clone().into_bytes());

        return stun_attribute
    }
}

//-----

pub struct MappedAddress {
    //generell attribute:
    type_: u16,
    length: u16,
    //Spesielt til error:
    family: u8,
    port: u16,
    address: IpAddr,
}

impl MappedAddress {
    pub fn new(family: u8, port: u16, address: IpAddr) -> Self {
        MappedAddress {
            type_: MAPPED_ADDRESS,
            length: (4 + address.to_string().len()) as u16, //Funker ikke, addressen varierer fra 32-128 bits
            family: family,
            port: port,
            address: address,
        }
    }

    pub fn serialize(&self) {
        let mut vec: Vec<u8> = Vec::new();
        //TODO
        //return vec;
    }
}

//-----
//TODO: alt med xor-mapped...
pub struct XorMappedAddress {
    //generell attribute:
    type_: u16,
    length: u16,
    //Spesielt til error:
    family: u8,
    port: u16,
    address: IpAddr,
}

impl XorMappedAddress {
    pub fn new(family: u8, port: u16, address: IpAddr) -> Self {
        XorMappedAddress {
            type_: XOR_MAPPED_ADDRESS,
            length: (4 + address.to_string().len()) as u16, //Funker ikke, addressen varierer fra 32-128 bits
            family: family,
            port: port,
            address: address,
        }
    }

    pub fn serialize(&self) {}
}

//-----

pub struct UnknownAttributes {
    //generell attribute:
    type_: u16,
    length: u16,
    //Spesielt til error:
    attributes: Vec<u16>,
}

impl UnknownAttributes {
    pub fn new(vec: Vec<u16>) -> Self {
        UnknownAttributes {
            type_: UNKNOWN_ATTRIBUTES,
            length: (vec.len() * 2) as u16, //Funker ikke, addressen varierer fra 32-128 bits
            attributes: vec,
        }
    }

    pub fn serialize(&self)  -> Vec<u8> {
        let mut stun_attribute: Vec<u8> = Vec::new();

        BigEndian::write_u16(&mut stun_attribute, self.type_);
        BigEndian::write_u16(&mut stun_attribute, self.length);
        for &attribute in &self.attributes {
            BigEndian::write_u16(&mut stun_attribute, attribute)
        }

        return stun_attribute
    }
}
