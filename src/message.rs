use crate::attributes::*;
use byteorder::{BigEndian, ByteOrder};

pub const MAGIC_COOKIE: u32 = 0x2112_A442;

pub struct StunHeader {
    type_: u16,
    length: u16,
    transaction_id: [u8; 12],
}

impl StunHeader {
    pub fn serialize(&self) -> Vec<u8> {
        let mut stun_header: Vec<u8> = vec![0 as u8; 8];

        BigEndian::write_u16(&mut stun_header, self.type_);
        BigEndian::write_u16(&mut stun_header, self.length);
        BigEndian::write_u32(&mut stun_header, MAGIC_COOKIE);
        stun_header.append(&mut self.transaction_id.to_vec());

        return stun_header;
    }

    pub fn new(type_: u16, length: u16, transaction_id: [u8; 12]) -> Self {
        StunHeader {
            type_: type_,
            length: length,
            transaction_id: transaction_id,
        }
    }
}

pub struct StunBody {
    pub attributes: Vec<Box<dyn Attribute>>,
}

impl StunBody {
    pub fn serialize(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        for attribute in &self.attributes {
            vec.append(&mut attribute.serialize());
        }
        return vec;
    }
}

pub struct StunMessage {
    pub stun_header: StunHeader,
    pub stun_body: StunBody,
}

impl StunMessage {
    pub fn serialize(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        vec.append(&mut self.stun_header.serialize());
        vec.append(&mut self.stun_body.serialize());
        return vec;
    }
}

// let mut vec = Vec::new();
// vec.write_u8::<BigEndian>(self.type_);
// vec.write_u8::<BigEndian>(self.length);
// vec.write_u8::<BigEndian>(self.magic_cookie);
// vec.write_u8::<BigEndian>(self.transaction_id); //denne kan legge til ekstra nuller (0), se struct
// return vec;

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(1, 1);
    }
}
