use byteorder::{BigEndian, ByteOrder};
use std::net::{IpAddr, SocketAddr, Ipv4Addr};
use crate::message::MAGIC_COOKIE;

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

        add_padding(self.length,&mut stun_attribute);

        return stun_attribute;
    }
}

//-----

pub struct MappedAddress {
    //generell attribute:
    type_: u16,
    length: u16,
    //Spesielt til MappedAddress:
    address: SocketAddr,
}

impl MappedAddress {
    pub fn new(address: SocketAddr) -> Self {
        let len:u16;

        match address.ip() {
            IpAddr::V4(_ip) => {
                len = 8;
            }
            IpAddr::V6(_ip) => {
                len = 20;
            }
        }

        return MappedAddress {
            type_: MAPPED_ADDRESS,
            length: len,
            address: address,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut stun_attribute: Vec<u8> = Vec::new();

        BigEndian::write_u16(&mut stun_attribute, self.type_);
        BigEndian::write_u16(&mut stun_attribute, self.length);
        match self.address.ip(){
            IpAddr::V4(ip) => {
                BigEndian::write_u16(&mut stun_attribute, 0x01);
                BigEndian::write_u16(&mut stun_attribute,self.address.port());
                stun_attribute.append(&mut ip.octets().to_vec());
            }
            IpAddr::V6(ip) => {
                BigEndian::write_u16(&mut stun_attribute, 0x02);
                BigEndian::write_u16(&mut stun_attribute, self.address.port());
                stun_attribute.append(&mut ip.octets().to_vec());
            }
        }
        add_padding(self.length,&mut stun_attribute);
        return stun_attribute;
    }
}

//-----

pub struct XorMappedAddress {
    //generell attribute:
     type_: u16,
    length: u16,
    //Spesielt til XorMappedAddress:
    address: SocketAddr,
}

impl XorMappedAddress {
    pub fn new(addr: SocketAddr, transaction_id: &[u8]) -> Self {
        let leng: u16;
        let xor_port: u16 = addr.port() ^ (MAGIC_COOKIE >> 16) as u16;
        let address: SocketAddr;
        match addr.ip() {
            IpAddr::V4(ip) => {
                leng = 8;
                let mut value = [0 as u8;4];
                for i in 0..4 {
                    value[i] = ip.octets()[i] ^ ((MAGIC_COOKIE << 8*i) >> 24) as u8;
                }
                address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(value[0],value[1],value[2],value[3])),xor_port);
            }
            IpAddr::V6(ip) => {
                leng = 20;
                
                let mut value = [0 as u8;16];
                for i in 0..4 {
                    value[i] = ip.octets()[i] ^ ((MAGIC_COOKIE << 8*i) >> 24) as u8;
                }
                for i in 4..16 {
                    value[i] = ip.octets()[i] ^ (transaction_id[i-4]);
                }
                let addr = From::from(value);
                address = SocketAddr::new(IpAddr::V6(addr),xor_port);
            }
        }

        return XorMappedAddress{
            type_:XOR_MAPPED_ADDRESS,
            length:leng,
            address:address
        }

    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut stun_attribute: Vec<u8> = Vec::new();

        BigEndian::write_u16(&mut stun_attribute, self.type_);
        BigEndian::write_u16(&mut stun_attribute, self.length);
        match self.address.ip(){
            IpAddr::V4(ip) => {
                BigEndian::write_u16(&mut stun_attribute, 0x01);
                BigEndian::write_u16(&mut stun_attribute, self.address.port());
                stun_attribute.append(&mut ip.octets().to_vec());
            }
            IpAddr::V6(ip) => {
                BigEndian::write_u16(&mut stun_attribute, 0x02);
                BigEndian::write_u16(&mut stun_attribute, self.address.port());
                stun_attribute.append(&mut ip.octets().to_vec());
            }
        }
        

        add_padding(self.length,&mut stun_attribute);
        return stun_attribute;
    }
}

//-----

pub struct UnknownAttributes {
    //generell attribute:
    type_: u16,
    length: u16,
    //Spesielt til denne:
    attributes: Vec<u16>,
}

impl UnknownAttributes {
    pub fn new(vec: Vec<u16>) -> Self {
        UnknownAttributes {
            type_: UNKNOWN_ATTRIBUTES,
            length: (vec.len() * 2) as u16, 
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

        add_padding(self.length,&mut stun_attribute);
        return stun_attribute;
    }
}

fn add_padding(length:u16, stun_attribute:&mut Vec<u8>){
    if length%4 != 0{
        for _i in 0..(4-(length%4)) {
            stun_attribute.push(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    const TRANSACTION_ID: [u8; 12] = [
        0xb7, 0xe7, 0xa7, 0x01, 0xbc, 0x34, 0xd6, 0x86, 0xfa, 0x87, 0xdf, 0xae,
    ];

    #[test]
    fn test_xor_encoding_v4() {
        let base_address: SocketAddr = "192.0.2.1:32853".parse().unwrap();

        assert_eq!(
            XorMappedAddress::new(base_address, &TRANSACTION_ID).address,
            "225.18.166.67:41287".parse().unwrap()
        );
    }
}
