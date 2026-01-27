use serde::Serialize;
use std::collections::HashMap;
use tauri::Manager;
use tauri_plugin_http::reqwest;
use tauri_plugin_opener::OpenerExt;

pub mod oauth;
pub mod secure_store;
pub mod token_manager;
pub mod mal;
pub mod request;
pub use secure_store::{init_stronghold_key, StrongholdKeyState};
pub use token_manager::{ProviderConfig, TokenManagerState};
pub use request::oauth_request;
use crate::auth::oauth::pkce::generate_pkce;
use crate::auth::token_manager::{build_authorize_url, exchange_authorization_code, TokenResponse};
use crate::auth::mal::PROVIDER_ID as MAL_PROVIDER_ID;

#[derive(Serialize)]
pub struct AuthorizePayload {
    pub authorize_url: String,
    pub code_verifier: String,
}

#[tauri::command]
pub async fn authorize_provider(
    provider_id: String,
    app: tauri::AppHandle,
) -> Result<AuthorizePayload, String> {
    authorize_provider_impl(&provider_id, app).await
}

#[tauri::command]
pub async fn authorize_myanimelist(app: tauri::AppHandle) -> Result<AuthorizePayload, String> {
    authorize_provider_impl(MAL_PROVIDER_ID, app).await
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

async fn authorize_provider_impl(
    provider_id: &str,
    app: tauri::AppHandle,
) -> Result<AuthorizePayload, String> {
    let provider = app.state::<TokenManagerState>().get_provider(provider_id)?;
    let pkce = if provider.use_pkce { Some(generate_pkce()) } else { None };
    let code_challenge = pkce.as_ref().map(|p| p.code_challenge.as_str());

    let authorize_url = build_authorize_url(&app, provider_id, code_challenge)?;

    if let Some(pkce) = pkce.as_ref() {
        app.state::<TokenManagerState>()
            .set_pkce_verifier(provider_id, pkce.code_verifier.clone())?;
    }

    app.opener()
        .open_url(&authorize_url, None::<String>)
        .map_err(|e| e.to_string())?;

    Ok(AuthorizePayload {
        authorize_url,
        code_verifier: pkce
            .as_ref()
            .map(|p| p.code_verifier.clone())
            .unwrap_or_default(),
    })
}

async fn handle_oauth_callback_impl(
    app: &tauri::AppHandle,
    url: &str,
    provider_override: Option<&str>,
) -> Result<(), String> {
    println!("Handling OAuth callback with URL: {}", url);
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

    let params: HashMap<String, String> = parsed
        .query_pairs()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

    if let Some(error) = params.get("error") {
        return Err(format!("Error during authorization: {error}"));
    }

    let code = params
        .get("code")
        .ok_or_else(|| "Missing authorization code".to_string())?;

    let code_verifier = app.state::<TokenManagerState>().take_pkce_verifier(&provider_id)?;
    let _response: TokenResponse =
        exchange_authorization_code(app, &provider_id, code, code_verifier).await?;
    Ok(())
}
