use rand::{distributions::Alphanumeric, Rng};
use std::collections::HashMap;
use tauri::{Emitter, Manager};
use tauri_plugin_http::reqwest;
use tauri_plugin_opener::OpenerExt;

pub mod mal;
pub mod anilist;
pub mod oauth;
pub mod request;
pub mod secure_store;
pub mod token_manager;
use crate::auth::mal::PROVIDER_ID as MAL_PROVIDER_ID;
use crate::auth::anilist::PROVIDER_ID as ANILIST_PROVIDER_ID;
use crate::auth::oauth::pkce::generate_pkce;
use crate::auth::token_manager::{
    build_authorize_url, exchange_authorization_code, store_access_token,
};
pub use request::oauth_request;
pub use secure_store::{init_stronghold_key, StrongholdKeyState};
pub use token_manager::{ProviderConfig, TokenManagerState};

const ANILIST_TOKEN_EXPIRES_IN_SECS: u64 = 60 * 60 * 24 * 365;

#[tauri::command]
pub async fn authorize_provider(provider_id: String, app: tauri::AppHandle) -> Result<(), String> {
    authorize_provider_impl(&provider_id, app).await
}

#[tauri::command]
pub async fn authorize_myanimelist(app: tauri::AppHandle) -> Result<(), String> {
    authorize_provider_impl(MAL_PROVIDER_ID, app).await
}

#[tauri::command]
pub async fn authorize_anilist(app: tauri::AppHandle) -> Result<(), String> {
    authorize_provider_impl(ANILIST_PROVIDER_ID, app).await
}

pub async fn handle_oauth_callback(app: tauri::AppHandle, url: String) {
    if let Err(err) = handle_oauth_callback_impl(&app, &url, None).await {
        eprintln!("OAuth callback error: {}", err);
    }
}

pub async fn handle_myanimelist_callback(app: tauri::AppHandle, url: String) {
    if let Err(err) = handle_oauth_callback_impl(&app, &url, Some(MAL_PROVIDER_ID)).await {
        eprintln!("MyAnimeList callback error: {}", err);
    }
}

async fn authorize_provider_impl(provider_id: &str, app: tauri::AppHandle) -> Result<(), String> {
    let provider = app.state::<TokenManagerState>().get_provider(provider_id)?;
    let state: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();
    let pkce = if provider.use_pkce {
        Some(generate_pkce())
    } else {
        None
    };
    let code_challenge = pkce.as_ref().map(|p| p.code_challenge.as_str());

    let authorize_url = build_authorize_url(&app, provider_id, code_challenge, Some(&state))?;

    if let Some(pkce) = pkce.as_ref() {
        app.state::<TokenManagerState>()
            .set_pkce_verifier(provider_id, pkce.code_verifier.clone())?;
        app.state::<TokenManagerState>().set_pkce_state(
            state,
            provider_id,
            pkce.code_verifier.clone(),
        )?;
    }

    app.opener()
        .open_url(&authorize_url, None::<String>)
        .map_err(|e| e.to_string())?;

    Ok(())
}

async fn handle_oauth_callback_impl(
    app: &tauri::AppHandle,
    url: &str,
    provider_override: Option<&str>,
) -> Result<(), String> {
    let parsed = reqwest::Url::parse(url).map_err(|e| e.to_string())?;

    let provider_id = if let Some(provider) = provider_override {
        provider.to_string()
    } else {
        parsed
            .host_str()
            .map(|h| h.to_string())
            .filter(|h| !h.is_empty())
            .or_else(|| {
                parsed
                    .path_segments()
                    .and_then(|mut segments| segments.next())
                    .map(|s| s.to_string())
            })
            .ok_or_else(|| "Missing provider id in callback URL".to_string())?
    };

    let mut params: HashMap<String, String> = parsed
        .query_pairs()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
    println!("OAuth callback params: {:?}", parsed);

    if provider_id == ANILIST_PROVIDER_ID {
        if let Some(fragment) = parsed.fragment() {
            let fragment_url = reqwest::Url::parse(&format!("kioku://anilist/?{fragment}"))
                .map_err(|e| e.to_string())?;
            params.extend(
                fragment_url
                    .query_pairs()
                    .map(|(k, v)| (k.to_string(), v.to_string())),
            );
        }
    }

    let state = params.get("state").cloned();

    if let Some(error) = params.get("error") {
        let failed_event_name = format!("{}-auth-failed", &provider_id);
        app.emit(failed_event_name.as_str(), ())
            .map_err(|e| e.to_string())?;

        return Err(format!("Error during authorization: {error}"));
    }

    if provider_id == ANILIST_PROVIDER_ID {
        let access_token = params
            .get("access_token")
            .ok_or_else(|| "Missing access token".to_string())?;

        if let Some(state) = state.as_ref() {
            match app.state::<TokenManagerState>().take_pkce_state(state)? {
                Some((state_provider_id, _)) => {
                    if state_provider_id != provider_id {
                        return Err("OAuth state/provider mismatch".to_string());
                    }
                }
                None => {
                    return Err("Missing PKCE verifier for state".to_string());
                }
            }
        }

        store_access_token(
            app,
            &provider_id,
            access_token,
            ANILIST_TOKEN_EXPIRES_IN_SECS,
        )?;

        let success_event_name = format!("{}-auth-callback", &provider_id);
        app.emit(success_event_name.as_str(), ())
            .map_err(|e| e.to_string())?;
        return Ok(());
    }

    let code = params
        .get("code")
        .ok_or_else(|| "Missing authorization code".to_string())?;

    let code_verifier = if let Some(state) = state.as_ref() {
        match app.state::<TokenManagerState>().take_pkce_state(state)? {
            Some((state_provider_id, verifier)) => {
                if state_provider_id != provider_id {
                    return Err("OAuth state/provider mismatch".to_string());
                }
                Some(verifier)
            }
            None => {
                return Err("Missing PKCE verifier for state".to_string());
            }
        }
    } else {
        app.state::<TokenManagerState>()
            .take_pkce_verifier(&provider_id)?
    };

    exchange_authorization_code(app, &provider_id, code, code_verifier).await?;

    let success_event_name = format!("{}-auth-callback", &provider_id);

    app.emit(success_event_name.as_str(), ())
        .map_err(|e| e.to_string())?;
    Ok(())
}
