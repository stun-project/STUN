use std::net::{IpAddr, Ipv4Addr, SocketAddr};
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
    address: SocketAddr,
}

impl XorMappedAddress {
    pub fn new(addr: SocketAddr, transaction_id:&[u8]) -> Self{
        let mc:u16 = (0x2112_A442>>16) as u16;
        let mc32: u32 = 0x2112_A442;
        let mut leng:u16;
        let mut xor_port:u16 = addr.port() ^ mc;
        let mut 
        match addr.ip() {
            IpAddr::V4(ip) => {
                leng = 8;
                let dg = ip.octets();
            }
            IpAddr::V6(ip) => {
                leng = 20;
            }
        }

        


        // X-Port is computed by taking the mapped port in host byte order,
        // XOR'ing it with the most significant 16 bits of the magic cookie, and
        // then the converting the result to network byte order.

        
        //address
        if family == 0x01 {
        //  If the IP
        //    address family is IPv4, X-Address is computed by taking the mapped IP
        //    address in host byte order, XOR'ing it with the magic cookie, and
        //    converting the result to network byte order. 
            //addr = address ^ 0x2112_A442
        }else{
            // If the IP address
            // family is IPv6, X-Address is computed by taking the mapped IP address
            // in host byte order, XOR'ing it with the concatenation of the magic
            // cookie and the 96-bit transaction ID, and converting the result to
            // network byte order.
            let xor_value:u128; //concaticating...
            //addr = address ^ xor_value;
        }

        //Fikse adresse, 


        return XorMappedAddress{
            type_:XOR_MAPPED_ADDRESS,
            length:leng,
            address:address // denne skal xores
        }
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
