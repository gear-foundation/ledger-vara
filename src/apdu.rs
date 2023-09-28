use nanos_sdk::io::{ApduHeader, Reply};

#[repr(u8)]
pub enum Ins {
    GetPubkey = 2,
    Sign,
    Menu,
    Exit = 0xFF,
}

impl From<ApduHeader> for Ins {
    fn from(header: ApduHeader) -> Ins {
        match header.ins {
            2 => Ins::GetPubkey,
            3 => Ins::Sign,
            4 => Ins::Menu,
            0xFF => Ins::Exit,
            _ => unreachable!("Invalid INS"),
        }
    }
}

pub fn handle_apdu(_ins: Ins) -> Result<(), Reply> {
    Ok(())
}
