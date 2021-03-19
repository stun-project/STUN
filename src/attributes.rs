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

enum Attribute{
    ERROR_CODE(ErrorCode),
    MAPPED_ADDRESS,
    XOR_MAPPED_ADDRESS,
    UNKNOWN_ATTRIBUTES
}

struct ErrorCode{
    //generell attribute:
    type_: u16;
    length: u16;
    //Spesielt til error:
    status_code: u32;
    reason_phrase: String;
}

impl ErrorCode {
    pub fn new(status_code:u32,reason_phrase: String) -> self{
        ErrorCode{
            type_:ERROR_CODE,
            length:(4+reason_phrase.len()),
            status_code:status_code,
            reason_phrase:reason_phrase
        }
    }

    pub fn serialize(&self) {
        let mut vec = Vec::new();
        //TODO
        return vec;
    }
}

//-----

struct MappedAddress{
    //generell attribute:
    type_: u16;
    length: u16;
    //Spesielt til error:
    family: u8;
    port: u16;
    address: u128;
}

impl MappedAddress {
    pub fn new(family:u8,port: u16, address:u128) -> self{
        MappedAddress{
            type_:MAPPED_ADDRESS,
            length:(4+address.len), //Funker ikke, addressen varierer fra 32-128 bits
            family:family,
            port:port,
            address:address
        }
    }

    pub fn serialize(&self) {
        let mut vec = Vec::new();
        //TODO
        return vec;
    }
}

//-----
//TODO: alt med xor-mapped...

//
// The XOR-MAPPED-ADDRESS attribute is identical to the MAPPED-ADDRESS
// attribute, except that the reflexive transport address is obfuscated
// through the XOR function.

// The format of the XOR-MAPPED-ADDRESS is:

//    0                   1                   2                   3
//    0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
//   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//   |x x x x x x x x|    Family     |         X-Port                |
//   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//   |                X-Address (Variable)
//   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

//           Figure 6: Format of XOR-MAPPED-ADDRESS Attribute

// The Family represents the IP address family, and is encoded
// identically to the Family in MAPPED-ADDRESS.

// X-Port is computed by taking the mapped port in host byte order,
// XOR'ing it with the most significant 16 bits of the magic cookie, and
// then the converting the result to network byte order.  If the IP
// address family is IPv4, X-Address is computed by taking the mapped IP
// address in host byte order, XOR'ing it with the magic cookie, and
// converting the result to network byte order.  If the IP address
// family is IPv6, X-Address is computed by taking the mapped IP address
// in host byte order, XOR'ing it with the concatenation of the magic
// cookie and the 96-bit transaction ID, and converting the result to
// network byte order.

// The rules for encoding and processing the first 8 bits of the
// attribute's value, the rules for handling multiple occurrences of the
// attribute, and the rules for processing address families are the same
// as for MAPPED-ADDRESS.

// Note: XOR-MAPPED-ADDRESS and MAPPED-ADDRESS differ only in their
// encoding of the transport address.  The former encodes the transport
// address by exclusive-or'ing it with the magic cookie.  The latter
// encodes it directly in binary.  RFC 3489 originally specified only
// MAPPED-ADDRESS.  However, deployment experience found that some NATs
// rewrite the 32-bit binary payloads containing the NAT's public IP
// address, such as STUN's MAPPED-ADDRESS attribute, in the well-meaning
// but misguided attempt at providing a generic ALG function.  Such
// behavior interferes with the operation of STUN and also causes
// failure of STUN's message-integrity checking.
struct XorMappedAddress{
    //generell attribute:
    type_: u16;
    length: u16;
    //Spesielt til error:
    family: u8;
    port: u16;
    address: u128;
}

impl XorMappedAddress {
    pub fn new(family:u8,port: u16, address:u128, transaction_id:u128) -> self{
        let leng:u16;
        if(family == 0x01) leng = 8; //er IPv4
        else leng = 20;              //er IPv6

        let xor_port:u16;
        let mc:u16; // de 16 mest signifikante bitsene av magic cookien

        // X-Port is computed by taking the mapped port in host byte order,
        // XOR'ing it with the most significant 16 bits of the magic cookie, and
        // then the converting the result to network byte order.
        xor_port = port ^ mc; // funker selvfølgelig ikke

        let mc32: u32 = 0x2112_A442;
        let addr:u128 //er bare 32 for ipv4
        //address
        if(family == 0x01){
        //  If the IP
        //    address family is IPv4, X-Address is computed by taking the mapped IP
        //    address in host byte order, XOR'ing it with the magic cookie, and
        //    converting the result to network byte order. 
            addr = address ^ mc32
        }else{
            // If the IP address
            // family is IPv6, X-Address is computed by taking the mapped IP address
            // in host byte order, XOR'ing it with the concatenation of the magic
            // cookie and the 96-bit transaction ID, and converting the result to
            // network byte order.
            let xor_value:u128; //concaticating...
            addr = address ^ xor_value;
        }


        return XorMappedAddress{
            type_:XOR_MAPPED_ADDRESS,
            length:leng,
            family:family,
            port:xor_port, // denne skal xores
            address:addr // denne skal xores
        }
    }

    pub fn serialize(&self) {
        let mut vec = Vec::new();
        //TODO
        return vec;
    }

    pub fn 
}


//-----

struct UnknownAttributes{
    //generell attribute:
    type_: u16;
    length: u16;
    //Spesielt til error:
    attributes: Vec::new();
}

impl UnknownAttributes {
    pub fn new(vec:&[u16]) -> self{
        UnknownAttributes{
            type_:UNKNOWN_ATTRIBUTES,
            length:(vec.len*2), //Funker ikke, addressen varierer fra 32-128 bits
            attributes: vec
        }
    }

    pub fn serialize(&self) {
        let mut vec = Vec::new();
        //TODO
        return vec;
    }
}

