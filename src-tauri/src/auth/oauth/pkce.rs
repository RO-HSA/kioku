use rand::{distributions::Alphanumeric, Rng};
use sha2::{Digest, Sha256};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

pub struct PkcePair {
    pub code_verifier: String,
    pub code_challenge: String,
}

pub fn generate_pkce() -> PkcePair {
    let code_verifier: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();

    let hash = Sha256::digest(code_verifier.as_bytes());

    let code_challenge = URL_SAFE_NO_PAD.encode(hash);

    PkcePair {
        code_verifier,
        code_challenge,
    }
}