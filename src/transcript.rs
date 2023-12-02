use ledger_device_sdk::random::LedgerRng;
use merlin::Transcript;
use rand_core::{CryptoRng, RngCore};
use schnorrkel::context::SigningTranscript;

pub struct LedgerTranscript(Transcript);

impl SigningTranscript for LedgerTranscript {
    fn commit_bytes(&mut self, label: &'static [u8], bytes: &[u8]) {
        self.0.append_message(label, bytes);
    }

    fn challenge_bytes(&mut self, label: &'static [u8], dest: &mut [u8]) {
        self.0.challenge_bytes(label, dest);
    }

    fn witness_bytes_rng<R>(
        &self,
        label: &'static [u8],
        dest: &mut [u8],
        nonce_seeds: &[&[u8]],
        _rng: R,
    ) where
        R: CryptoRng + RngCore,
    {
        let rng = LedgerRng;
        self.0.witness_bytes_rng(label, dest, nonce_seeds, rng);
    }
}

impl LedgerTranscript {
    pub fn new() -> Self {
        let mut transcript = Transcript::new(b"SigningContext");
        // Append empty context
        transcript.append_message(b"", b"");
        Self(transcript)
    }

    pub fn append(&mut self, bytes: &[u8]) {
        self.0.append_message(b"sign-bytes", bytes);
    }
}
