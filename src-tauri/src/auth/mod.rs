use rand::{distributions::Alphanumeric, Rng};
use std::collections::HashMap;
use tauri::{Emitter, Manager};
use tauri_plugin_http::reqwest;
use tauri_plugin_opener::OpenerExt;

pub mod anilist;
pub mod mal;
pub mod oauth;
pub mod request;
pub mod secure_store;
pub mod token_manager;
use crate::auth::anilist::PROVIDER_ID as ANILIST_PROVIDER_ID;
use crate::auth::mal::PROVIDER_ID as MAL_PROVIDER_ID;
use crate::auth::oauth::pkce::generate_pkce;
use crate::auth::token_manager::{
    build_authorize_url, exchange_authorization_code, store_access_token,
};
pub use request::oauth_request;
pub use secure_store::{init_stronghold_key, StrongholdKeyState};
pub use token_manager::{ProviderConfig, TokenManagerState};

fn emit_auth_failed(app: &tauri::AppHandle, provider_id: &str) {
    let failed_event_name = format!("{provider_id}-auth-failed");
    if let Err(err) = app.emit(failed_event_name.as_str(), ()) {
        eprintln!("Failed to emit {failed_event_name}: {err}");
    }
}

fn emit_auth_succeeded(app: &tauri::AppHandle, provider_id: &str) -> Result<(), String> {
    let success_event_name = format!("{provider_id}-auth-callback");
    app.emit(success_event_name.as_str(), ())
        .map_err(|e| e.to_string())
}

fn callback_hint_candidates(parsed: &reqwest::Url) -> Vec<String> {
    let mut hints = Vec::new();
    if let Some(host) = parsed.host_str().filter(|h| !h.is_empty()) {
        hints.push(host.to_string());
    }

    if let Some(segment) = parsed
        .path_segments()
        .and_then(|mut segments| segments.next())
        .filter(|segment| !segment.is_empty())
    {
        hints.push(segment.to_string());
    }

    hints
}

fn collect_callback_params(parsed: &reqwest::Url) -> Result<HashMap<String, String>, String> {
    let mut params: HashMap<String, String> = parsed
        .query_pairs()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

    if let Some(fragment) = parsed.fragment() {
        let fragment_url = reqwest::Url::parse(&format!("kioku://callback/?{fragment}"))
            .map_err(|e| e.to_string())?;
        params.extend(
            fragment_url
                .query_pairs()
                .map(|(k, v)| (k.to_string(), v.to_string())),
        );
    }

    Ok(params)
}

fn resolve_provider_id_from_callback(
    token_manager: &TokenManagerState,
    parsed: &reqwest::Url,
    params: &HashMap<String, String>,
    provider_override: Option<&str>,
) -> Result<String, String> {
    if let Some(provider) = provider_override {
        return Ok(provider.to_string());
    }

    if let Some(state) = params.get("state") {
        if let Some(provider_id) = token_manager.get_oauth_state_provider(state)? {
            return Ok(provider_id);
        }
    }

    for hint in callback_hint_candidates(parsed) {
        if let Some(provider_id) = token_manager.get_provider_from_callback_hint(&hint)? {
            return Ok(provider_id);
        }
    }

    if let Some(provider_id) = token_manager.infer_provider_from_callback_params(params)? {
        return Ok(provider_id);
    }

    Err("Unable to determine provider from callback URL".to_string())
}

fn missing_callback_payload_error(
    provider_id: &str,
    provider: &ProviderConfig,
) -> Result<(), String> {
    let mut expected = Vec::new();
    if let Some(access_param) = provider.callback_access_token_param.as_ref() {
        expected.push(access_param.as_str());
    }
    if let Some(code_param) = provider.callback_code_param.as_ref() {
        expected.push(code_param.as_str());
    }

    if expected.is_empty() {
        return Err(format!(
            "Provider {provider_id} has no callback payload fields configured"
        ));
    }

    Err(format!(
        "Missing callback payload for provider {provider_id}; expected one of: {}",
        expected.join(", ")
    ))
}

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
    let state = provider.uses_state.then(|| {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect::<String>()
    });
    let pkce = if provider.use_pkce {
        Some(generate_pkce())
    } else {
        None
    };
    let code_challenge = pkce.as_ref().map(|p| p.code_challenge.as_str());

    let authorize_url = build_authorize_url(&app, provider_id, code_challenge, state.as_deref())?;

    if let Some(state) = state {
        app.state::<TokenManagerState>().set_oauth_state(
            state,
            provider_id,
            pkce.as_ref().map(|pkce| pkce.code_verifier.clone()),
        )?;
    } else if let Some(pkce) = pkce.as_ref() {
        app.state::<TokenManagerState>()
            .set_pkce_verifier(provider_id, pkce.code_verifier.clone())?;
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
    let params = collect_callback_params(&parsed)?;
    let provider_id = resolve_provider_id_from_callback(
        &app.state::<TokenManagerState>(),
        &parsed,
        &params,
        provider_override,
    )?;

    let provider = app
        .state::<TokenManagerState>()
        .get_provider(&provider_id)?;
    let callback_state = params.get(&provider.callback_state_param).cloned();

    let callback_result: Result<(), String> = async {
        if let Some(error) = params.get("error") {
            return Err(format!("Error during authorization: {error}"));
        }

        let state = callback_state.clone();

        let state_entry = if provider.uses_state {
            let state = state
                .as_ref()
                .ok_or_else(|| "Missing OAuth state in callback".to_string())?;
            let state_entry = app
                .state::<TokenManagerState>()
                .take_oauth_state(state)?
                .ok_or_else(|| "Unknown or expired OAuth state".to_string())?;
            if state_entry.0 != provider_id {
                return Err("OAuth state/provider mismatch".to_string());
            }
            Some(state_entry)
        } else {
            None
        };

        if let Some(access_token_param) = provider.callback_access_token_param.as_ref() {
            if let Some(access_token) = params.get(access_token_param) {
                let expires_in =
                    provider
                        .resolve_callback_expires_in(&params)
                        .ok_or_else(|| {
                            "Missing access token expiration in callback and no default configured"
                                .to_string()
                        })?;
                store_access_token(app, &provider_id, access_token, expires_in)?;
                emit_auth_succeeded(app, &provider_id)?;
                return Ok(());
            }
        }

        if let Some(code_param) = provider.callback_code_param.as_ref() {
            if let Some(code) = params.get(code_param) {
                let code_verifier = if provider.use_pkce {
                    if provider.uses_state {
                        let (_, verifier) = state_entry
                            .as_ref()
                            .ok_or_else(|| "Missing OAuth state for PKCE validation".to_string())?;
                        Some(verifier.clone().ok_or_else(|| {
                            "Missing PKCE verifier associated with OAuth state".to_string()
                        })?)
                    } else {
                        app.state::<TokenManagerState>()
                            .take_pkce_verifier(&provider_id)?
                    }
                } else {
                    None
                };

                exchange_authorization_code(app, &provider_id, code, code_verifier).await?;
                emit_auth_succeeded(app, &provider_id)?;
                return Ok(());
            }
        }

        missing_callback_payload_error(&provider_id, &provider)
    }
    .await;

    if let Err(err) = callback_result {
        if provider.use_pkce && !provider.uses_state {
            if let Err(cleanup_err) = app
                .state::<TokenManagerState>()
                .take_pkce_verifier(&provider_id)
            {
                eprintln!("PKCE cleanup failed for {provider_id}: {cleanup_err}");
            }
        }
        if provider.uses_state {
            if let Some(state) = callback_state.as_ref() {
                if let Err(cleanup_err) = app.state::<TokenManagerState>().take_oauth_state(state) {
                    eprintln!("OAuth state cleanup failed for {provider_id}: {cleanup_err}");
                }
            }
        }
        emit_auth_failed(app, &provider_id);
        return Err(err);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn register_provider(state: &TokenManagerState, provider_id: &str, config: ProviderConfig) {
        state
            .register_provider(provider_id, config)
            .expect("provider should register");
    }

    #[test]
    fn callback_hint_candidates_collect_host_and_first_path_segment() {
        let url =
            reqwest::Url::parse("kioku://myanimelist/callback?code=1").expect("url should parse");
        assert_eq!(
            callback_hint_candidates(&url),
            vec!["myanimelist".to_string(), "callback".to_string()]
        );

        let url = reqwest::Url::parse("kioku://callback").expect("url should parse");
        assert_eq!(callback_hint_candidates(&url), vec!["callback".to_string()]);
    }

    #[test]
    fn collect_callback_params_merges_query_and_fragment_pairs() {
        let url = reqwest::Url::parse(
            "kioku://callback?code=query-code&state=query-state#access_token=fragment-token&state=fragment-state",
        )
        .expect("url should parse");

        let params = collect_callback_params(&url).expect("params should collect");

        assert_eq!(params.get("code").map(String::as_str), Some("query-code"));
        assert_eq!(
            params.get("access_token").map(String::as_str),
            Some("fragment-token")
        );
        assert_eq!(
            params.get("state").map(String::as_str),
            Some("fragment-state")
        );
    }

    #[test]
    fn resolve_provider_id_from_callback_obeys_override_state_hint_and_payload_inference() {
        let state = TokenManagerState::default();
        register_provider(
            &state,
            "myanimelist",
            ProviderConfig::new("id", "https://auth", "https://token")
                .with_callback_provider_hint("mal")
                .with_callback_code_param("code"),
        );
        register_provider(
            &state,
            "anilist",
            ProviderConfig::new("id", "https://auth", "https://token")
                .with_state(false)
                .without_callback_code()
                .with_callback_access_token_param("access_token")
                .with_callback_provider_hint("anilist"),
        );
        state
            .set_oauth_state("known-state".to_string(), "myanimelist", None)
            .expect("oauth state should store");

        let override_url = reqwest::Url::parse("kioku://callback").expect("url should parse");
        let empty = HashMap::new();
        assert_eq!(
            resolve_provider_id_from_callback(&state, &override_url, &empty, Some("forced"))
                .unwrap(),
            "forced"
        );

        let state_url =
            reqwest::Url::parse("kioku://callback?state=known-state").expect("url should parse");
        let state_params = collect_callback_params(&state_url).expect("params should collect");
        assert_eq!(
            resolve_provider_id_from_callback(&state, &state_url, &state_params, None).unwrap(),
            "myanimelist"
        );

        let hint_url = reqwest::Url::parse("kioku://mal/callback").expect("url should parse");
        assert_eq!(
            resolve_provider_id_from_callback(&state, &hint_url, &HashMap::new(), None).unwrap(),
            "myanimelist"
        );

        let payload_url =
            reqwest::Url::parse("kioku://callback#access_token=token").expect("url should parse");
        let payload_params = collect_callback_params(&payload_url).expect("params should collect");
        assert_eq!(
            resolve_provider_id_from_callback(&state, &payload_url, &payload_params, None).unwrap(),
            "anilist"
        );
    }

    #[test]
    fn resolve_provider_id_from_callback_errors_when_unresolvable() {
        let state = TokenManagerState::default();
        let url = reqwest::Url::parse("kioku://callback").expect("url should parse");
        let error =
            resolve_provider_id_from_callback(&state, &url, &HashMap::new(), None).unwrap_err();
        assert_eq!(error, "Unable to determine provider from callback URL");
    }

    #[test]
    fn missing_callback_payload_error_reports_expected_params() {
        let provider = ProviderConfig::new("id", "https://auth", "https://token")
            .with_callback_access_token_param("access_token")
            .with_callback_code_param("code");
        assert_eq!(
            missing_callback_payload_error("provider", &provider).unwrap_err(),
            "Missing callback payload for provider provider; expected one of: access_token, code"
        );

        let provider = ProviderConfig::new("id", "https://auth", "https://token")
            .without_callback_code()
            .without_callback_access_token();
        assert_eq!(
            missing_callback_payload_error("provider", &provider).unwrap_err(),
            "Provider provider has no callback payload fields configured"
        );
    }
}
