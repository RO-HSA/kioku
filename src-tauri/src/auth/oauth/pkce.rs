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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_pkce_returns_plain_alphanumeric_pair_for_mal() {
        let pair = generate_pkce();

        assert_eq!(pair.code_verifier.len(), 64);
        assert_eq!(pair.code_challenge, pair.code_verifier);
        assert!(pair
            .code_verifier
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric()));
    }
}
