use byteorder::{ByteOrder, BigEndian};

pub const BINDING_REQUEST: u16 = 0x0001;
pub const BINDING_RESPONSE: u16 = 0x0101;
pub const BINDING_ERROR_RESPONSE: u16 = 0x0111;
pub const BINDING_INDICATION: u16 = 0x0011;
pub const MAGIC_COOKIE: u32 = 0x2112_A442;

#[derive(Debug)]
pub struct BindingRequest {
    type_: u16;
    length: u16;
    magic_cookie: u32;
    transaction_id: u128;
}

impl BindingRequest{
    pub fn new(length:u16,transaction_id:u128) -> Self {
        BindingRequest{
            type_: BINDING_REQUEST,
            length: length,
            magic_cookie: MAGIC_COOKIE,
            transaction_id: transaction_id
        }
    }
}

pub struct BindingResponse {
    type_: u16 = BINDING_RESPONSE;
    length: u16;
    const MAGIC_COOKIE: u32 = 0x2112_A442; 
    transactionId: u128;
}

pub struct BindingIndication {
    type_: u16 = BINDING_INDICATION;
    length: u16;
    const MAGIC_COOKIE: u32 = 0x2112_A442; 
    transactionId: u128;
}

pub struct BindingErrorResponse {
    type_: u16 = BINDING_ERROR_RESPONSE;
    length: u16;
    const MAGIC_COOKIE: u32 = 0x2112_A442; 
    transactionId: u128;
}

enum HeaderType {
    REQUEST(BindingRequest),
    RESPONSE(BindingResponse),
    INDICATION(BindingIndication),
    ERROR_RESPONSE(BindingErrorResponse)
};


pub struct StunHeader {
    type_: u16;
    length: u16;
    magic_cookie: u32;
    transaction_id: u128;
}

impl StunHeader {
    pub fn serialize(&self) {
        let mut vec = Vec::new();
        vec.write_u8::<BigEndian>(self.type_);
        vec.write_u8::<BigEndian>(self.length);
        vec.write_u8::<BigEndian>(self.magic_cookie);
        vec.write_u8::<BigEndian>(self.transaction_id); //denne kan legge til ekstra nuller (0), se struct
        return vec;

    }

    pub fn new(type_:u16,length:u16,transaction_id:u128) -> self {
        StunHeader{
            type_: type_,
            length: length,
            magic_cookie: MAGIC_COOKIE,
            transaction_id: transaction_id
        }
    }
}

pub struct Attribute {
    type_: u16;
    length: u16;
    value: u128;
}

impl Attribute {
    pub fn serialize(&self) {
        let mut vec = Vec::new();
        vec.write_u8::<BigEndian>(self.type_);
        vec.write_u8::<BigEndian>(self.length);
        vec.write_u8::<BigEndian>(self.value); //denne kan legge til ekstra nuller (0), se struct
        return vec;
    }
}

pub struct StunBody {
    attributes: Vec<Attribute>
}

impl StunBody {
    pub fn serialize(&self) {
        let mut vec = Vec::new();
        for attribute in attributes {
            vec.append(attribute.serialize);
        }
        return vec;
    }
}


struct StunMessage {
    stunHeader: StunHeader;
    stunPayload: StunBody
}

impl StunMessage {
    pub fn serialize(&self) {
        let mut vec = Vec::new();
        vec.append(stunHeader.serialize());
        vec.append(stunPayload.serialize());
        return vec;
    }
}