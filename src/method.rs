use crate::attributes::*;
//use byteorder::{BigEndian, ByteOrder};
pub const MAGIC_COOKIE: u32 = 0x2112_A442;

pub struct StunHeader {
    type_: u16,
    length: u16,
    magic_cookie: u32,
    transaction_id: u128,
}

impl StunHeader {
    pub fn serialize(&self) -> &mut Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        todo!();
        //return vec;
    }

    pub fn new(type_: u16, length: u16, transaction_id: u128) -> Self {
        StunHeader {
            type_: type_,
            length: length,
            magic_cookie: MAGIC_COOKIE,
            transaction_id: transaction_id,
        }
    }
}

pub struct StunBody {
    pub attributes: Vec<Attribute>,
}

impl StunBody {
    pub fn serialize(&self) -> &mut Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        for attribute in &self.attributes {
            //vec.append(attribute.serialize);
        }
        todo!();
        //return vec;
    }
}

pub struct StunMessage {
    pub stun_header: StunHeader,
    pub stun_body: StunBody,
}

impl StunMessage {
    pub fn serialize(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        vec.append(self.stun_header.serialize());
        vec.append(self.stun_body.serialize());
        return vec;
    }
}

// let mut vec = Vec::new();
// vec.write_u8::<BigEndian>(self.type_);
// vec.write_u8::<BigEndian>(self.length);
// vec.write_u8::<BigEndian>(self.magic_cookie);
// vec.write_u8::<BigEndian>(self.transaction_id); //denne kan legge til ekstra nuller (0), se struct
// return vec;
