pub enum ErrorCodeEnum {
    BadRequest = 400,
    Unauthorized = 401,
    TryAlternate = 300,
    UnknownAttributes = 420,
    StaleNonce = 438,
    ServerError = 500,
}
impl ErrorCodeEnum {
    pub fn reason_phrase(&self) -> &str {
        match *self {
            ErrorCodeEnum::BadRequest => BAD_REQUEST_REASON,
            ErrorCodeEnum::Unauthorized => UNAUTHORIZED_REASON,
            ErrorCodeEnum::TryAlternate => TRY_ALTERNATE_REASON,
            ErrorCodeEnum::StaleNonce => STALE_NONCE_REASON,
            ErrorCodeEnum::UnknownAttributes => UNKNOWN_ATTRIBUTE_REASON,
            ErrorCodeEnum::ServerError => SERVER_ERROR_REASON,
        }
    }
}

const BAD_REQUEST_REASON: &str = "Bad Request: The request was malformed.  The client SHOULD NOT
        retry the request without modification from the previous
        attempt.  The server may not be able to generate a valid
        MESSAGE-INTEGRITY for this error, so the client MUST NOT expect
        a valid MESSAGE-INTEGRITY attribute on this response.
";

const UNAUTHORIZED_REASON: &str = "Unauthorized: The request did not contain the correct
        credentials to proceed.  The client should retry the request
        with proper credentials.";

const TRY_ALTERNATE_REASON: &str =
    "Try Alternate: The client should contact an alternate server for
        this request.  This error response MUST only be sent if the
        request included a USERNAME attribute and a valid MESSAGE-
        INTEGRITY attribute; otherwise, it MUST NOT be sent and error
        code 400 (Bad Request) is suggested.  This error response MUST
        be protected with the MESSAGE-INTEGRITY attribute, and receivers
        MUST validate the MESSAGE-INTEGRITY of this response before
        redirecting themselves to an alternate server.";

const UNKNOWN_ATTRIBUTE_REASON: &str =
    "Unknown Attribute: The server received a STUN packet containing
        a comprehension-required attribute that it did not understand.
        The server MUST put this unknown attribute in the UNKNOWN-
        ATTRIBUTE attribute of its error response.";

const STALE_NONCE_REASON: &str = "Stale Nonce: The NONCE used by the client was no longer valid.
        The client should retry, using the NONCE provided in the
        response.";
const SERVER_ERROR_REASON: &str = "Server Error: The server has suffered a temporary error.  The
        client should try again.";

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(1, 1);
    }
}
