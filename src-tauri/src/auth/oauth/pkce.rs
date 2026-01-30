use rand::{distributions::Alphanumeric, Rng};

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

    PkcePair {
        code_verifier: code_verifier.clone(),
        code_challenge: code_verifier,
    }
}
