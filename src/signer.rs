use crate::{error::ErrorCode, transcript::LedgerTranscript};
use core::mem;
use ledger_device_sdk::ecc::{CurvesId, Ed25519, SeedDerive};
use schnorrkel::{ExpansionMode, MiniSecretKey};

const MAX_MESSAGE_LEN: usize = 256;

pub struct Signer {
    path: [u32; 5],
    scheme: Scheme,
    message_len: usize,
    message: [u8; MAX_MESSAGE_LEN],
}

#[derive(Default, PartialEq)]
#[repr(u8)]
pub enum Scheme {
    #[default]
    Ed25519 = 0,
    Sr25519,
}

#[repr(C)]
struct PrivateKey {
    curve: CurvesId,
    keylength: usize,
    pub key: [u8; 32],
}

impl Default for Signer {
    fn default() -> Self {
        Self {
            path: Default::default(),
            scheme: Default::default(),
            message_len: Default::default(),
            message: [0; MAX_MESSAGE_LEN],
        }
    }
}

impl TryFrom<u8> for Scheme {
    type Error = ErrorCode;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Scheme::Ed25519),
            1 => Ok(Scheme::Sr25519),
            _ => Err(ErrorCode::BadP1P2),
        }
    }
}

impl Signer {
    pub fn clear(&mut self) {
        self.path.fill(0);
        self.scheme = Scheme::Ed25519;
        self.message_len = 0;
        self.message.fill(0);
    }

    pub fn set_path(&mut self, path: [u32; 5]) {
        self.path = path;
    }

    pub fn set_scheme(&mut self, scheme: Scheme) {
        self.scheme = scheme;
    }

    pub fn check_scheme(&self, scheme: Scheme) -> Result<(), ErrorCode> {
        if self.scheme != scheme {
            return Err(ErrorCode::BadP1P2);
        }
        Ok(())
    }

    pub fn append_message(&mut self, data: &[u8]) -> Result<(), ErrorCode> {
        if self.message_len + data.len() > MAX_MESSAGE_LEN {
            return Err(ErrorCode::BadLen);
        }
        self.message[self.message_len..self.message_len + data.len()].copy_from_slice(data);
        self.message_len += data.len();
        Ok(())
    }

    pub fn get_public_key(&self) -> Result<[u8; 32], ErrorCode> {
        if self.path[0] != 0x8000002c || self.path[1] != 0x80000391 {
            return Err(ErrorCode::BadPath);
        }
        let private_key = self.get_private_key();
        let public_key = match self.scheme {
            Scheme::Ed25519 => {
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
            Scheme::Sr25519 => MiniSecretKey::from_bytes(&private_key)?
                .expand(ExpansionMode::Ed25519)
                .to_public()
                .to_bytes(),
        };

        Ok(public_key)
    }

    pub fn sign(&self) -> Result<[u8; 64], ErrorCode> {
        if self.path[0] != 0x8000002c || self.path[1] != 0x80000391 {
            return Err(ErrorCode::BadPath);
        }

        let private_key = self.get_private_key();
        let message = &self.message[..self.message_len];

        let signature = match self.scheme {
            Scheme::Ed25519 => {
                let (signature, _) = Ed25519::from(&private_key).sign(message)?;
                signature
            }
            Scheme::Sr25519 => {
                let pair = MiniSecretKey::from_bytes(&private_key)?
                    .expand(ExpansionMode::Ed25519)
                    .to_keypair();
                let mut transcript: LedgerTranscript = LedgerTranscript::new();
                transcript.append(message);
                pair.sign(transcript).to_bytes()
            }
        };
        Ok(signature)
    }

    fn get_private_key(&self) -> [u8; 32] {
        let k: PrivateKey = unsafe { mem::transmute(Ed25519::derive_from_path(&self.path)) };
        k.key
    }
}
