use byteorder::{BigEndian, ByteOrder};
use std::net::{IpAddr, SocketAddr};

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
    ErrorCode(ErrorCode),
    MappedAddress(MappedAddress),
    XorMappedAddress(XorMappedAddress),
    UnknownAttributes(UnknownAttributes),
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

        return stun_attribute;
    }
}

//-----

pub struct MappedAddress {
    //generell attribute:
    _type_: u16,
    _length: u16,
    //Spesielt til error:
    _family: u8,
    _port: u16,
    _address: IpAddr,
}

impl MappedAddress {
    pub fn new(family: u8, port: u16, address: IpAddr) -> Self {
        MappedAddress {
            _type_: MAPPED_ADDRESS,
            _length: (4 + address.to_string().len()) as u16, //Funker ikke, addressen varierer fra 32-128 bits
            _family: family,
            _port: port,
            _address: address,
        }
    }

    pub fn serialize(&self) {
        //let mut vec: Vec<u8> = Vec::new();
        //TODO
        //return vec;
    }
}

//-----
//TODO: alt med xor-mapped...
pub struct XorMappedAddress {
    //generell attribute:
    _type_: u16,
    _length: u16,
    //Spesielt til error:
    _address: SocketAddr,
}

impl XorMappedAddress {
    pub fn new(addr: SocketAddr, _transaction_id: &[u8]) -> Self {
        let mc: u16 = (0x2112_A442 >> 16) as u16;
        let _mc32: u32 = 0x2112_A442;
        let mut _leng: u16;
        let mut _xor_port: u16 = addr.port() ^ mc;
        match addr.ip() {
            IpAddr::V4(_ip) => {
                //leng = 8;
                //let dg = ip.octets();
            }
            IpAddr::V6(_ip) => {
                //leng = 20;
            }
        }

        // X-Port is computed by taking the mapped port in host byte order,
        // XOR'ing it with the most significant 16 bits of the magic cookie, and
        // then the converting the result to network byte order.

        //address
        //if family == 0x01 {
        //  If the IP
        //    address family is IPv4, X-Address is computed by taking the mapped IP
        //    address in host byte order, XOR'ing it with the magic cookie, and
        //    converting the result to network byte order.
        //addr = address ^ 0x2112_A442
        //} else {
        // If the IP address
        // family is IPv6, X-Address is computed by taking the mapped IP address
        // in host byte order, XOR'ing it with the concatenation of the magic
        // cookie and the 96-bit transaction ID, and converting the result to
        // network byte order.
        //let xor_value: u128; //concaticating...
        //addr = address ^ xor_value;
        //}

        //Fikse adresse,

        todo!();
    }

    pub fn serialize(&self) {
        todo!()
    }
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

    pub fn serialize(&self) -> Vec<u8> {
        let mut stun_attribute: Vec<u8> = Vec::new();

        BigEndian::write_u16(&mut stun_attribute, self.type_);
        BigEndian::write_u16(&mut stun_attribute, self.length);
        for &attribute in &self.attributes {
            BigEndian::write_u16(&mut stun_attribute, attribute)
        }

        return stun_attribute;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    const transaction_id: [u8; 12] = [
        0xb7, 0xe7, 0xa7, 0x01, 0xbc, 0x34, 0xd6, 0x86, 0xfa, 0x87, 0xdf, 0xae,
    ];

    #[test]
    fn test_xor_encoding_v4() {
        let base_address: SocketAddr = "192.0.2.1:32853".parse().unwrap();

        assert_eq!(
            XorMappedAddress::new(base_address, &transaction_id)._address,
            "225.18.166.67:41287".parse().unwrap()
        );
    }
}
