use byteorder::{BigEndian, ByteOrder};
use std::net::{IpAddr, SocketAddr, Ipv4Addr};

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
    //Spesielt til MappedAddress:
    _address: SocketAddr,
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
            _type_: MAPPED_ADDRESS,
            _length: len,
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

pub struct XorMappedAddress {
    //generell attribute:
     _type: u16,
    _length: u16,
    //Spesielt til XorMappedAddress:
    _address: SocketAddr,
}

impl XorMappedAddress {
    pub fn new(addr: SocketAddr, _transaction_id: &[u8]) -> Self {
        let _mc16: u16 = (0x2112_A442 >> 16) as u16;
        let _mc32: u32 = 0x2112_A442;
        let mut _leng: u16;
        let mut _xor_port: u16 = addr.port() ^ _mc16;
        let mut address: SocketAddr = addr;
        match addr.ip() {
            IpAddr::V4(ip) => {
                _leng = 8;
                let mut value = [0 as u8;4];
                for i in 0..4 {
                    value[i] = ip.octets()[i] ^ ((_mc32 << 8*i) >> 24) as u8;
                }
                address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(value[0],value[1],value[2],value[3])),_xor_port);
            }
            IpAddr::V6(ip) => {
                _leng = 20;
                let mut xor_value_string:String = _mc32.to_string().to_owned();
                let transaction_id_str: &str = str::from_utf8(_transaction_id);
                xor_value_string.push_str(transaction_id_str);
                //Latter som xor_value er en string
                let xor_value: u128 = xor_value_string.parse().unwrap();
                let mut value = [0 as u16;8];
                for i in 0..8 {
                    value[i] = ip.segments()[i] ^ ((xor_value << 16*i) >> 112) as u16;
                }

                let addr = From::from(value);
                address = SocketAddr::new(IpAddr::V6(addr),_xor_port);


            }
        }
        //byte order, mest til minst signifikant

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
        return XorMappedAddress{
            _type:XOR_MAPPED_ADDRESS,
            _length:_leng,
            _address:address
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
    const TRANSACTION_ID: [u8; 12] = [
        0xb7, 0xe7, 0xa7, 0x01, 0xbc, 0x34, 0xd6, 0x86, 0xfa, 0x87, 0xdf, 0xae,
    ];

    #[test]
    fn test_xor_encoding_v4() {
        let base_address: SocketAddr = "192.0.2.1:32853".parse().unwrap();

        assert_eq!(
            XorMappedAddress::new(base_address, &TRANSACTION_ID)._address,
            "225.18.166.67:41287".parse().unwrap()
        );
    }
}
