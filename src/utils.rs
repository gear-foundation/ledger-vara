use nanos_sdk::ecc::{self, Ed25519, SeedDerive};

pub const fn bytes_to_u16(bytes: &[u8]) -> u16 {
    let mut i = 0;
    let mut acc = 0;

    while i < bytes.len() {
        let c = bytes[i];
        match c {
            b'0'..=b'9' => {
                acc = (c - b'0') as u32;
            }
            _ => panic!("expected digit"),
        }
        i += 1;
    }

    if acc > 65535 {
        panic!("too big version element value");
    }

    acc as u16
}

pub fn get_public_key() -> [u8; 32] {
    let path: [u32; 5] = ecc::make_bip32_path(b"m/44'/913'/0'/0/0");
    let key = Ed25519::derive_from_path(&path).public_key().unwrap();
    key.pubkey[1..33].try_into().unwrap()
}
