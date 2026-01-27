use std::path::PathBuf;
use std::sync::Mutex;

use base64::{engine::general_purpose, Engine as _};
use rand::rngs::OsRng;
use rand::RngCore;
use tauri::{AppHandle, Manager};
use tauri_plugin_stronghold::stronghold::Stronghold;

const VAULT_FILE_NAME: &str = "stronghold.hold";
const KEYRING_ENTRY: &str = "stronghold-master-key";
const KEY_LENGTH: usize = 32;
const CLIENT_ID: &[u8] = b"oauth";
const REFRESH_TOKEN_KEY: &str = "refresh_token";

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

fn app_data_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_local_data_dir()
        .map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir)
}

fn vault_path(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(app_data_dir(app)?.join(VAULT_FILE_NAME))
}

fn keyring_entry(app: &AppHandle) -> Result<keyring::Entry, String> {
    let service = app.config().identifier.clone();
    keyring::Entry::new(&service, KEYRING_ENTRY).map_err(|e| e.to_string())
}

fn generate_master_key() -> Vec<u8> {
    let mut key = [0u8; KEY_LENGTH];
    OsRng.fill_bytes(&mut key);
    key.to_vec()
}

fn load_or_create_master_key(app: &AppHandle) -> Result<Vec<u8>, String> {
    let entry = keyring_entry(app)?;
    match entry.get_password() {
        Ok(encoded) => {
            let decoded = general_purpose::STANDARD
                .decode(encoded.as_bytes())
                .map_err(|e| e.to_string())?;
            if decoded.len() != KEY_LENGTH {
                return Err("Stronghold key has invalid length".to_string());
            }
            Ok(decoded)
        }
        Err(err) => {
            if matches!(err, keyring::Error::NoEntry) {
                let key = generate_master_key();
                let encoded = general_purpose::STANDARD.encode(&key);
                entry
                    .set_password(&encoded)
                    .map_err(|e| e.to_string())?;
                Ok(key)
            } else {
                Err(err.to_string())
            }
        }
    }
}

fn open_stronghold(app: &AppHandle, key: &[u8]) -> Result<Stronghold, String> {
    let path = vault_path(app)?;
    Stronghold::new(path, key.to_vec()).map_err(|e| e.to_string())
}

fn refresh_record_key(provider_id: &str) -> String {
    format!("{provider_id}:{REFRESH_TOKEN_KEY}")
}

pub fn init_stronghold_key(app: &AppHandle) -> Result<(), String> {
    let key = load_or_create_master_key(app)?;
    app.state::<StrongholdKeyState>().set_key(key)
}

pub fn save_refresh_token(
    app: &AppHandle,
    provider_id: &str,
    refresh_token: &str,
) -> Result<(), String> {
    let key = app.state::<StrongholdKeyState>().get_key()?;
    let stronghold = open_stronghold(app, &key)?;
    let client = stronghold
        .load_client(CLIENT_ID)
        .or_else(|_| stronghold.create_client(CLIENT_ID))
        .map_err(|e| e.to_string())?;

    client
        .store()
        .insert(
            refresh_record_key(provider_id).as_bytes().to_vec(),
            refresh_token.as_bytes().to_vec(),
            None,
        )
        .map_err(|e| e.to_string())?;

    stronghold.save().map_err(|e| e.to_string())
}

pub fn read_refresh_token(app: &AppHandle, provider_id: &str) -> Result<Option<String>, String> {
    let key = app.state::<StrongholdKeyState>().get_key()?;
    let stronghold = open_stronghold(app, &key)?;
    let client = stronghold
        .load_client(CLIENT_ID)
        .or_else(|_| stronghold.create_client(CLIENT_ID))
        .map_err(|e| e.to_string())?;

    let raw = client
        .store()
        .get(refresh_record_key(provider_id).as_bytes())
        .map_err(|e| e.to_string())?;

    Ok(raw.and_then(|bytes| String::from_utf8(bytes).ok()))
}
