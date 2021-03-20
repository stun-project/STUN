use crate::attributes::{AttributeEnum, XorMappedAddress, ErrorCode, MappedAddress};
use std::net::SocketAddr;
use crate::errors::ErrorCodeEnum;
use crate::method::{StunBody, StunHeader, StunMessage, MAGIC_COOKIE};
use byteorder::{BigEndian, ByteOrder};
pub const BINDING_REQUEST: u16 = 0x0001;
pub const BINDING_RESPONSE: u16 = 0x0101;
pub const BINDING_ERROR_RESPONSE: u16 = 0x0111;
pub const BINDING_INDICATION: u16 = 0x0011;
use std::convert::TryInto;

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

pub fn handle_message(stun_message: &[u8],address: SocketAddr) -> StunMessage {
    //let mut response: Vec<u8> = Vec::new();
    if !check_validity(&stun_message) {
        return StunMessage {
            stun_header: StunHeader::new(
                BINDING_ERROR_RESPONSE,
                BODY_LENGTH,
                stun_message[8..20].try_into().unwrap(),
            ),
            stun_body: StunBody {
                attributes: vec![Box::new(AttributeEnum::ErrorCode({
                    ErrorCode::new(
                        ErrorCodeEnum::BadRequest as u32,
                        ErrorCodeEnum::reason_phrase(&ErrorCodeEnum::BadRequest).to_string(),
                    )
                }))],
            },
        };
    }
    return StunMessage {
        stun_header: StunHeader::new(
            BINDING_RESPONSE,
            BODY_LENGTH,
            stun_message[8..20].try_into().unwrap(),
        ),
        stun_body: StunBody {
            attributes: vec![
                Box::new(AttributeEnum::XorMappedAddress({
                XorMappedAddress::new(address,stun_message[8..20].try_into().unwrap())
            })),
                Box::new(AttributeEnum::MappedAddress({
                    MappedAddress::new(address)
                }))
            
            ],
        },
    };
}

//TODO - check length []
pub fn check_validity(stun_message: &[u8]) -> bool {
    if stun_message[0] >= 64 {
        return false;
    }
    if BigEndian::read_u32(&stun_message[4..8]) != MAGIC_COOKIE {
        return false;
    }
    let type_ = BigEndian::read_u16(&stun_message[0..2]);
    if type_ != BINDING_REQUEST && type_ != BINDING_INDICATION {
        return false;
    }
    println!("Message is valid");
    return true;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(1, 1);
    }
}
