use crate::error::ErrorCode;
use core::mem;
use nanos_sdk::ecc::{CurvesId, Ed25519, SeedDerive};
use schnorrkel::{ExpansionMode, MiniSecretKey};

const SCHEME_ED25519: u8 = 0;
const SCHEME_SR25519: u8 = 1;

#[repr(C)]
struct PrivateKey {
    curve: CurvesId,
    keylength: usize,
    pub key: [u8; 32],
}

fn get_private_key(path: &[u32]) -> [u8; 32] {
    let k: PrivateKey = unsafe { mem::transmute(Ed25519::derive_from_path(&path)) };
    k.key
}

pub fn get_public_key(scheme: u8, path: &[u32]) -> Result<[u8; 32], ErrorCode> {
    if path.len() != 5 {
        return Err(ErrorCode::BadLen);
    }
    if path[0] != 0x8000002c || path[1] != 0x80000391 {
        return Err(ErrorCode::BadPath);
    }
    let private_key = get_private_key(path);
    let public_key = match scheme {
        SCHEME_ED25519 => {
            let pk = Ed25519::from(&private_key).public_key()?.pubkey;
            let mut key = [0; 32];
            for i in 0..key.len() {
                key[i] = pk[64 - i];
            }
            if (pk[key.len()] & 1) != 0 {
                key[key.len() - 1] |= 0x80;
            }
            key
        }
        SCHEME_SR25519 => MiniSecretKey::from_bytes(&private_key)?
            .expand(ExpansionMode::Ed25519)
            .to_public()
            .to_bytes(),
        _ => return Err(ErrorCode::BadP1P2),
    };

    Ok(public_key)
}
