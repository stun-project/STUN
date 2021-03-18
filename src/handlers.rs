mod method;
use method;
mod errors;
use errors;
mod stun;
use stun;
mod attributes;
use attributes;
use byteorder::{ByteOrder, BigEndian};

#[test]
fn testHandlers(){
    println!("{:?}",handle_header());
}

pub fn handle_header(stunHeader: &[u8;20]){ //stunMessage skal defineres som struct
    let type_ = BigEndian::read_u16(&stunHeader[0..1]);
    let length = BigEndian::read_u16(&stunHeader[2..3]);
    let transaction_id = BigEndian::read_u128(&stunHeader[8..19])
    let header_type = match type_{
        BINDING_REQUEST => BindingRequest::new(length,transaction_id);

    }
    return header_type;
}

pub fn handle_message(stunMessage: &[u8]){
    let mut response: Vec<u8> = Vec::new();
    if !check_validity(&stunMessage) {
        return StunMessage{
            stunHeader:StunHeader::new(BINDING_ERROR_RESPONSE,length_body/*hmm*/,MAGIC_COOKIE,BigEndian::read_u128(&stunHeader[8..19])),
            stunPayload:StunBody{
                attributes:vec![Attribute{
                    type_:ERROR_CODE,
                    length:64,
                    value: 0x0009
                }];
            }
        }
    }
    //---
    return StunMessage{
        stunHeader:StunHeader::new(BINDING_RESPONSE,length_body/*hmm*/,MAGIC_COOKIE,BigEndian::read_u128(&stunHeader[8..19])),
        stunPayload:StunBody{

        }
    }

    //---
}

//TODO - check length []
pub fn check_validity(stunMessage: &[u8]){
    if stunMessage[0] >= 64 {
        return false;
    }
    if BigEndian::read_u32(&stunHeader[4..7]) != MAGIC_COOKIE{
        return false;
    }
    let type_ = BigEndian::read_u16(&stunHeader[0..1]);
    if type_ != BINDING_REQUEST && type_ != BINDING_INDICATION{
        return false;
    }
    return true;
}