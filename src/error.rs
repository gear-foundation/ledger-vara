use core::{array::TryFromSliceError, num::ParseIntError};
use ledger_device_sdk::{ecc::CxError, io::Reply};
use schnorrkel::SignatureError;

#[derive(Copy, Clone)]
#[repr(u16)]
pub enum ErrorCode {
    NothingReceived = 0x6982,
    Unknown = 0x6d00,
    Unimplemented = 0x6d01,
    BadCla = 0x6e00,
    BadIns = 0x6e01,
    BadP1P2 = 0x6e02,
    BadLen = 0x6e03,
    BadPath = 0x6f00,
    UserCancelled = 0x6e04,
    ParsingError = 0x7f00,
    ConversionError = 0x7f01,
    CryptoError = 0x7f02,
    SignatureError = 0x7f03,
    Ok = 0x9000,
    Panic = 0xe000,
}

impl From<ErrorCode> for Reply {
    fn from(value: ErrorCode) -> Self {
        Reply(value as u16)
    }
}

impl From<ParseIntError> for ErrorCode {
    fn from(_value: ParseIntError) -> Self {
        ErrorCode::ParsingError
    }
}

impl From<TryFromSliceError> for ErrorCode {
    fn from(_value: TryFromSliceError) -> Self {
        ErrorCode::ConversionError
    }
}

impl From<CxError> for ErrorCode {
    fn from(_value: CxError) -> Self {
        ErrorCode::CryptoError
    }
}

impl From<SignatureError> for ErrorCode {
    fn from(_value: SignatureError) -> Self {
        ErrorCode::SignatureError
    }
}
