use std::path::PathBuf;
use std::sync::Mutex;

use base64::{engine::general_purpose, Engine as _};
use rand::rngs::OsRng;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_stronghold::stronghold::Stronghold;

const VAULT_FILE_NAME: &str = "stronghold.hold";
const KEYRING_ENTRY: &str = "stronghold-master-key";
const KEY_LENGTH: usize = 32;
const CLIENT_ID: &[u8] = b"oauth";
const REFRESH_TOKEN_KEY: &str = "refresh_token";
const ACCESS_TOKEN_KEY: &str = "access_token";

#[derive(Deserialize, Serialize)]
struct AccessTokenRecord {
    token: String,
    expires_at_unix_secs: u64,
}

#[derive(Default)]
pub struct StrongholdKeyState(Mutex<Option<Vec<u8>>>);

impl StrongholdKeyState {
    fn set_key(&self, key: Vec<u8>) -> Result<(), String> {
        let mut guard = self
            .0
            .lock()
            .map_err(|_| "Stronghold key lock poisoned".to_string())?;
        *guard = Some(key);
        Ok(())
    }

    fn get_key(&self) -> Result<Vec<u8>, String> {
        let guard = self
            .0
            .lock()
            .map_err(|_| "Stronghold key lock poisoned".to_string())?;
        guard
            .as_ref()
            .cloned()
            .ok_or_else(|| "Stronghold key not initialized".to_string())
    }
}

fn app_data_dir<R: Runtime>(app: &AppHandle<R>) -> Result<PathBuf, String> {
    let dir = app.path().app_local_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir)
}

fn vault_path<R: Runtime>(app: &AppHandle<R>) -> Result<PathBuf, String> {
    Ok(app_data_dir(app)?.join(VAULT_FILE_NAME))
}

fn keyring_entry<R: Runtime>(app: &AppHandle<R>) -> Result<keyring::Entry, String> {
    let service = app.config().identifier.clone();
    keyring::Entry::new(&service, KEYRING_ENTRY).map_err(|e| e.to_string())
}

fn encode_master_key(key: &[u8]) -> String {
    general_purpose::STANDARD.encode(key)
}

fn decode_master_key(encoded: &str) -> Result<Vec<u8>, String> {
    let decoded = general_purpose::STANDARD
        .decode(encoded.as_bytes())
        .map_err(|e| e.to_string())?;
    if decoded.len() != KEY_LENGTH {
        return Err("Stronghold key has invalid length".to_string());
    }
    Ok(decoded)
}

fn generate_master_key() -> Vec<u8> {
    let mut key = [0u8; KEY_LENGTH];
    OsRng.fill_bytes(&mut key);
    key.to_vec()
}

fn load_or_create_master_key<R: Runtime>(app: &AppHandle<R>) -> Result<Vec<u8>, String> {
    let entry = keyring_entry(app)?;
    match entry.get_password() {
        Ok(encoded) => decode_master_key(&encoded),
        Err(err) => {
            if matches!(err, keyring::Error::NoEntry) {
                let key = generate_master_key();
                let encoded = encode_master_key(&key);
                entry.set_password(&encoded).map_err(|e| e.to_string())?;
                Ok(key)
            } else {
                Err(err.to_string())
            }
        }
    }
}

fn open_stronghold_at_path(path: PathBuf, key: &[u8]) -> Result<Stronghold, String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    Stronghold::new(path, key.to_vec()).map_err(|e| e.to_string())
}

fn open_stronghold<R: Runtime>(app: &AppHandle<R>, key: &[u8]) -> Result<Stronghold, String> {
    open_stronghold_at_path(vault_path(app)?, key)
}

fn token_record_key(provider_id: &str, token_key: &str) -> String {
    format!("{provider_id}:{token_key}")
}

fn refresh_record_key(provider_id: &str) -> String {
    token_record_key(provider_id, REFRESH_TOKEN_KEY)
}

fn access_record_key(provider_id: &str) -> String {
    token_record_key(provider_id, ACCESS_TOKEN_KEY)
}

fn encode_access_token_record(token: &str, expires_at_unix_secs: u64) -> Result<Vec<u8>, String> {
    serde_json::to_vec(&AccessTokenRecord {
        token: token.to_string(),
        expires_at_unix_secs,
    })
    .map_err(|e| e.to_string())
}

fn decode_access_token_record(raw: &[u8]) -> Result<(String, u64), String> {
    let record = serde_json::from_slice::<AccessTokenRecord>(raw).map_err(|e| e.to_string())?;
    Ok((record.token, record.expires_at_unix_secs))
}

fn save_secret_bytes(
    stronghold: &Stronghold,
    record_key: &str,
    value: Vec<u8>,
) -> Result<(), String> {
    let client = stronghold
        .load_client(CLIENT_ID)
        .or_else(|_| stronghold.create_client(CLIENT_ID))
        .map_err(|e| e.to_string())?;

    client
        .store()
        .insert(record_key.as_bytes().to_vec(), value, None)
        .map_err(|e| e.to_string())?;

    stronghold.save().map_err(|e| e.to_string())
}

fn read_secret_bytes(stronghold: &Stronghold, record_key: &str) -> Result<Option<Vec<u8>>, String> {
    let client = stronghold
        .load_client(CLIENT_ID)
        .or_else(|_| stronghold.create_client(CLIENT_ID))
        .map_err(|e| e.to_string())?;

    client
        .store()
        .get(record_key.as_bytes())
        .map_err(|e| e.to_string())
}

fn decode_refresh_token(raw: Option<Vec<u8>>) -> Option<String> {
    raw.and_then(|bytes| String::from_utf8(bytes).ok())
}

pub fn init_stronghold_key<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    let key = load_or_create_master_key(app)?;
    app.state::<StrongholdKeyState>().set_key(key)
}

pub fn save_refresh_token<R: Runtime>(
    app: &AppHandle<R>,
    provider_id: &str,
    refresh_token: &str,
) -> Result<(), String> {
    let key = app.state::<StrongholdKeyState>().get_key()?;
    let stronghold = open_stronghold(app, &key)?;
    save_secret_bytes(
        &stronghold,
        &refresh_record_key(provider_id),
        refresh_token.as_bytes().to_vec(),
    )
}

pub fn read_refresh_token<R: Runtime>(
    app: &AppHandle<R>,
    provider_id: &str,
) -> Result<Option<String>, String> {
    let key = app.state::<StrongholdKeyState>().get_key()?;
    let stronghold = open_stronghold(app, &key)?;
    let raw = read_secret_bytes(&stronghold, &refresh_record_key(provider_id))?;
    Ok(decode_refresh_token(raw))
}

pub fn save_access_token<R: Runtime>(
    app: &AppHandle<R>,
    provider_id: &str,
    access_token: &str,
    expires_at_unix_secs: u64,
) -> Result<(), String> {
    let key = app.state::<StrongholdKeyState>().get_key()?;
    let stronghold = open_stronghold(app, &key)?;
    let encoded = encode_access_token_record(access_token, expires_at_unix_secs)?;
    save_secret_bytes(&stronghold, &access_record_key(provider_id), encoded)
}

pub fn read_access_token<R: Runtime>(
    app: &AppHandle<R>,
    provider_id: &str,
) -> Result<Option<(String, u64)>, String> {
    let key = app.state::<StrongholdKeyState>().get_key()?;
    let stronghold = open_stronghold(app, &key)?;
    let raw = read_secret_bytes(&stronghold, &access_record_key(provider_id))?;

    let Some(raw) = raw else {
        return Ok(None);
    };

    decode_access_token_record(&raw).map(Some)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stronghold_key_state_roundtrips_and_errors_before_initialization() {
        let state = StrongholdKeyState::default();
        assert_eq!(
            state.get_key().unwrap_err(),
            "Stronghold key not initialized"
        );

        state.set_key(vec![1, 2, 3]).expect("key should store");
        assert_eq!(state.get_key().unwrap(), vec![1, 2, 3]);
    }

    #[test]
    fn generate_master_key_has_expected_length_and_nonconstant_output() {
        let first = generate_master_key();
        let second = generate_master_key();

        assert_eq!(first.len(), KEY_LENGTH);
        assert_eq!(second.len(), KEY_LENGTH);
        assert_ne!(first, second);
    }

    #[test]
    fn token_record_keys_are_namespaced_per_provider_and_kind() {
        assert_eq!(
            token_record_key("mal", "refresh_token"),
            "mal:refresh_token"
        );
        assert_eq!(refresh_record_key("mal"), "mal:refresh_token");
        assert_eq!(access_record_key("anilist"), "anilist:access_token");
    }

    #[test]
    fn master_key_encoding_roundtrips_and_validates_input() {
        let key = generate_master_key();
        let encoded = encode_master_key(&key);

        assert_eq!(decode_master_key(&encoded).unwrap(), key);
        assert!(decode_master_key("%%%").is_err());
        assert_eq!(
            decode_master_key(&encode_master_key(&[1, 2, 3]))
                .err()
                .as_deref(),
            Some("Stronghold key has invalid length")
        );
    }

    #[test]
    fn access_token_record_encoding_roundtrips_and_rejects_invalid_json() {
        let encoded = encode_access_token_record("token", 123).expect("record should encode");
        assert_eq!(
            decode_access_token_record(&encoded).unwrap(),
            ("token".to_string(), 123)
        );
        assert!(decode_access_token_record(br#"{"token":1}"#).is_err());
    }

    #[test]
    fn decode_refresh_token_returns_none_for_missing_or_invalid_utf8() {
        assert_eq!(decode_refresh_token(None), None);
        assert_eq!(
            decode_refresh_token(Some(b"refresh-token".to_vec())),
            Some("refresh-token".to_string())
        );
        assert_eq!(decode_refresh_token(Some(vec![0xff, 0xfe])), None);
    }
}
