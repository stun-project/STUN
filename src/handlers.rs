use crate::attributes::{Attribute, ErrorCode, ERROR_CODE};
use crate::errors::ErrorCodeEnum;
use crate::method::{StunBody, StunHeader, StunMessage, MAGIC_COOKIE};
use byteorder::{BigEndian, ByteOrder};
pub const BINDING_REQUEST: u16 = 0x0001;
pub const BINDING_RESPONSE: u16 = 0x0101;
pub const BINDING_ERROR_RESPONSE: u16 = 0x0111;
pub const BINDING_INDICATION: u16 = 0x0011;

#[test]
fn testHandlers() {

    // println!("{:?}",handle_header());
}

// pub fn handle_header(stunHeader: &[u8;20]){ //stunMessage skal defineres som struct
//     let type_ = BigEndian::read_u16(&stunHeader[0..1]);
//     let length = BigEndian::read_u16(&stunHeader[2..3]);
//     let transaction_id = BigEndian::read_u128(&stunHeader[8..19])
//     let header_type = match type_{
//        // BINDING_REQUEST => BindingRequest::new(length,transaction_id),
//     };
//     return header_type;
// }

const BODY_LENGTH: u16 = 6;

pub fn handle_message(stun_message: &[u8]) -> StunMessage {
    let mut response: Vec<u8> = Vec::new();
    if !check_validity(&stun_message) {
        return StunMessage {
            stun_header: StunHeader::new(
                BINDING_ERROR_RESPONSE,
                BODY_LENGTH,
                BigEndian::read_u128(&stun_message[8..19]),
            ),
            stun_body: StunBody {
                attributes: vec![Attribute::ERROR_CODE({
                    ErrorCode::new(ErrorCodeEnum::BAD_REQUEST as u32, "Yes".to_owned())
                })],
            },
        };
    }
    return StunMessage {
        stun_header: StunHeader::new(
            BINDING_RESPONSE,
            BODY_LENGTH,
            BigEndian::read_u128(&stun_message[8..19]),
        ),
        stun_body: StunBody {
            attributes: vec![Attribute::ERROR_CODE({
                ErrorCode::new(ErrorCodeEnum::BAD_REQUEST as u32, "Yes".to_owned())
            })],
        },
    };
}

//TODO - check length []
pub fn check_validity(stun_message: &[u8]) -> bool {
    if stun_message[0] >= 64 {
        return false;
    }
    if BigEndian::read_u32(&stun_message[4..7]) != MAGIC_COOKIE {
        return false;
    }
    let type_ = BigEndian::read_u16(&stun_message[0..1]);
    if type_ != BINDING_REQUEST && type_ != BINDING_INDICATION {
        return false;
    }
    return true;
}
