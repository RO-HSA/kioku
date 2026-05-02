use rand::{distributions::Alphanumeric, Rng};
use std::collections::HashMap;
#[cfg(target_os = "linux")]
use std::{
    env,
    ffi::OsString,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
};
use tauri::{Emitter, Manager, Runtime};
use tauri_plugin_http::reqwest;
#[cfg(not(target_os = "linux"))]
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

#[derive(Debug, PartialEq, Eq)]
enum CallbackPayload {
    AccessToken {
        access_token: String,
        expires_in: u64,
    },
    AuthorizationCode {
        code: String,
    },
}

fn emit_auth_failed<R: Runtime>(app: &tauri::AppHandle<R>, provider_id: &str) {
    let failed_event_name = format!("{provider_id}-auth-failed");
    if app.emit(failed_event_name.as_str(), ()).is_err() {
        eprintln!("Failed to emit auth failure event");
    }
}

fn emit_auth_succeeded<R: Runtime>(
    app: &tauri::AppHandle<R>,
    provider_id: &str,
) -> Result<(), String> {
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

fn missing_callback_payload_error(provider_id: &str, provider: &ProviderConfig) -> String {
    let mut expected = Vec::new();
    if let Some(access_param) = provider.callback_access_token_param.as_ref() {
        expected.push(access_param.as_str());
    }
    if let Some(code_param) = provider.callback_code_param.as_ref() {
        expected.push(code_param.as_str());
    }

    if expected.is_empty() {
        return format!("Provider {provider_id} has no callback payload fields configured");
    }

    format!(
        "Missing callback payload for provider {provider_id}; expected one of: {}",
        expected.join(", ")
    )
}

fn validate_callback_state(
    token_manager: &TokenManagerState,
    provider_id: &str,
    provider: &ProviderConfig,
    callback_state: Option<&str>,
) -> Result<Option<(String, Option<String>)>, String> {
    if !provider.uses_state {
        return Ok(None);
    }

    let state = callback_state.ok_or_else(|| "Missing OAuth state in callback".to_string())?;
    let state_entry = token_manager
        .take_oauth_state(state)?
        .ok_or_else(|| "Unknown or expired OAuth state".to_string())?;

    if state_entry.0 != provider_id {
        return Err("OAuth state/provider mismatch".to_string());
    }

    Ok(Some(state_entry))
}

fn extract_callback_payload(
    provider_id: &str,
    provider: &ProviderConfig,
    params: &HashMap<String, String>,
) -> Result<CallbackPayload, String> {
    if let Some(error) = params.get("error") {
        return Err(format!("Error during authorization: {error}"));
    }

    if let Some(access_token_param) = provider.callback_access_token_param.as_ref() {
        if let Some(access_token) = params.get(access_token_param) {
            let expires_in = provider
                .resolve_callback_expires_in(params)
                .ok_or_else(|| {
                    "Missing access token expiration in callback and no default configured"
                        .to_string()
                })?;

            return Ok(CallbackPayload::AccessToken {
                access_token: access_token.clone(),
                expires_in,
            });
        }
    }

    if let Some(code_param) = provider.callback_code_param.as_ref() {
        if let Some(code) = params.get(code_param) {
            return Ok(CallbackPayload::AuthorizationCode { code: code.clone() });
        }
    }

    Err(missing_callback_payload_error(provider_id, provider))
}

fn resolve_pkce_verifier(
    token_manager: &TokenManagerState,
    provider_id: &str,
    provider: &ProviderConfig,
    state_entry: Option<&(String, Option<String>)>,
) -> Result<Option<String>, String> {
    if !provider.use_pkce {
        return Ok(None);
    }

    if provider.uses_state {
        let (_, verifier) =
            state_entry.ok_or_else(|| "Missing OAuth state for PKCE validation".to_string())?;
        return Ok(Some(verifier.clone().ok_or_else(|| {
            "Missing PKCE verifier associated with OAuth state".to_string()
        })?));
    }

    token_manager.take_pkce_verifier(provider_id)
}

#[cfg(target_os = "linux")]
const DEFAULT_SYSTEM_PATH: &str = "/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin";

#[cfg(target_os = "linux")]
const APPIMAGE_ENV_VARS: &[&str] = &[
    "APPDIR",
    "APPIMAGE",
    "ARGV0",
    "GDK_PIXBUF_MODULE_FILE",
    "GDK_PIXBUF_MODULEDIR",
    "GIO_MODULE_DIR",
    "GTK_DATA_PREFIX",
    "GTK_EXE_PREFIX",
    "GTK_PATH",
    "LD_LIBRARY_PATH",
    "LD_PRELOAD",
    "OWD",
    "XDG_DATA_DIRS",
];

#[cfg(target_os = "linux")]
fn cleaned_system_path() -> OsString {
    let Some(path) = env::var_os("PATH") else {
        return OsString::from(DEFAULT_SYSTEM_PATH);
    };
    let appdir = env::var_os("APPDIR").map(PathBuf::from);
    let paths = env::split_paths(&path)
        .filter(|path| match appdir.as_ref() {
            Some(appdir) => !path.starts_with(appdir),
            None => true,
        })
        .collect::<Vec<_>>();

    if paths.is_empty() {
        OsString::from(DEFAULT_SYSTEM_PATH)
    } else {
        env::join_paths(paths).unwrap_or_else(|_| OsString::from(DEFAULT_SYSTEM_PATH))
    }
}

#[cfg(target_os = "linux")]
fn spawn_detached_with_clean_env(mut command: Command) -> Result<(), String> {
    command
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .env("PATH", cleaned_system_path());

    for key in APPIMAGE_ENV_VARS {
        command.env_remove(key);
    }

    let mut child = command.spawn().map_err(|e| e.to_string())?;
    thread::spawn(move || {
        let _ = child.wait();
    });

    Ok(())
}

#[cfg(target_os = "linux")]
fn system_opener_commands(url: &str) -> Vec<Command> {
    let mut commands = Vec::new();

    if Path::new("/usr/bin/xdg-open").exists() {
        let mut command = Command::new("/usr/bin/xdg-open");
        command.arg(url);
        commands.push(command);
    }

    if Path::new("/usr/bin/gio").exists() {
        let mut command = Command::new("/usr/bin/gio");
        command.arg("open").arg(url);
        commands.push(command);
    }

    let mut xdg_open = Command::new("xdg-open");
    xdg_open.arg(url);
    commands.push(xdg_open);

    let mut gio = Command::new("gio");
    gio.arg("open").arg(url);
    commands.push(gio);

    commands
}

#[cfg(target_os = "linux")]
fn open_authorize_url<R: Runtime>(_app: &tauri::AppHandle<R>, url: &str) -> Result<(), String> {
    let mut last_error = None;

    for command in system_opener_commands(url) {
        match spawn_detached_with_clean_env(command) {
            Ok(()) => return Ok(()),
            Err(err) => last_error = Some(err),
        }
    }

    Err(format!(
        "Failed to open authorization URL with the system opener: {}",
        last_error.unwrap_or_else(|| "no opener command available".to_string())
    ))
}

#[cfg(not(target_os = "linux"))]
fn open_authorize_url<R: Runtime>(app: &tauri::AppHandle<R>, url: &str) -> Result<(), String> {
    app.opener()
        .open_url(url, None::<String>)
        .map_err(|e| e.to_string())
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
    if handle_oauth_callback_impl(&app, &url, None).await.is_err() {
        eprintln!("OAuth callback handling failed");
    }
}

pub async fn handle_myanimelist_callback(app: tauri::AppHandle, url: String) {
    if handle_oauth_callback_impl(&app, &url, Some(MAL_PROVIDER_ID))
        .await
        .is_err()
    {
        eprintln!("MyAnimeList callback handling failed");
    }
}

async fn authorize_provider_impl<R: Runtime>(
    provider_id: &str,
    app: tauri::AppHandle<R>,
) -> Result<(), String> {
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

    open_authorize_url(&app, &authorize_url)?;

    Ok(())
}

async fn handle_oauth_callback_impl(
    app: &tauri::AppHandle<impl Runtime>,
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
        let token_manager = app.state::<TokenManagerState>();
        let state_entry = validate_callback_state(
            &token_manager,
            &provider_id,
            &provider,
            callback_state.as_deref(),
        )?;

        match extract_callback_payload(&provider_id, &provider, &params)? {
            CallbackPayload::AccessToken {
                access_token,
                expires_in,
            } => {
                store_access_token(app, &provider_id, &access_token, expires_in)?;
                emit_auth_succeeded(app, &provider_id)?;
                Ok(())
            }
            CallbackPayload::AuthorizationCode { code } => {
                let code_verifier = resolve_pkce_verifier(
                    &token_manager,
                    &provider_id,
                    &provider,
                    state_entry.as_ref(),
                )?;
                exchange_authorization_code(app, &provider_id, &code, code_verifier).await?;
                emit_auth_succeeded(app, &provider_id)?;
                Ok(())
            }
        }
    }
    .await;

    if let Err(err) = callback_result {
        if provider.use_pkce && !provider.uses_state {
            if app
                .state::<TokenManagerState>()
                .take_pkce_verifier(&provider_id)
                .is_err()
            {
                eprintln!("PKCE cleanup failed");
            }
        }
        if provider.uses_state {
            if let Some(state) = callback_state.as_ref() {
                if app
                    .state::<TokenManagerState>()
                    .take_oauth_state(state)
                    .is_err()
                {
                    eprintln!("OAuth state cleanup failed");
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

    fn test_provider() -> ProviderConfig {
        ProviderConfig::new("id", "https://auth", "https://token")
    }

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
        let provider = test_provider()
            .with_callback_access_token_param("access_token")
            .with_callback_code_param("code");
        assert_eq!(
            missing_callback_payload_error("provider", &provider),
            "Missing callback payload for provider provider; expected one of: access_token, code"
        );

        let provider = test_provider()
            .without_callback_code()
            .without_callback_access_token();
        assert_eq!(
            missing_callback_payload_error("provider", &provider),
            "Provider provider has no callback payload fields configured"
        );
    }

    #[test]
    fn validate_callback_state_handles_disabled_missing_unknown_mismatched_and_valid_states() {
        let state = TokenManagerState::default();
        let provider = test_provider();

        assert_eq!(
            validate_callback_state(&state, "provider", &provider, None)
                .err()
                .as_deref(),
            Some("Missing OAuth state in callback")
        );
        assert_eq!(
            validate_callback_state(&state, "provider", &provider, Some("missing"))
                .err()
                .as_deref(),
            Some("Unknown or expired OAuth state")
        );

        state
            .set_oauth_state("wrong".to_string(), "other-provider", None)
            .expect("state should store");
        assert_eq!(
            validate_callback_state(&state, "provider", &provider, Some("wrong"))
                .err()
                .as_deref(),
            Some("OAuth state/provider mismatch")
        );

        state
            .set_oauth_state(
                "known".to_string(),
                "provider",
                Some("verifier".to_string()),
            )
            .expect("state should store");
        assert_eq!(
            validate_callback_state(&state, "provider", &provider, Some("known")).unwrap(),
            Some(("provider".to_string(), Some("verifier".to_string())))
        );
        assert_eq!(state.take_oauth_state("known").unwrap(), None);

        assert_eq!(
            validate_callback_state(&state, "provider", &test_provider().with_state(false), None,)
                .unwrap(),
            None
        );
    }

    #[test]
    fn extract_callback_payload_handles_errors_tokens_codes_and_missing_expiration() {
        let implicit_provider = test_provider()
            .without_callback_code()
            .with_callback_access_token_param("access_token");
        let default_ttl_provider = implicit_provider
            .clone()
            .with_default_access_token_ttl_secs(3600);
        let code_provider = test_provider()
            .without_callback_access_token()
            .with_callback_code_param("authorization_code");

        assert_eq!(
            extract_callback_payload(
                "provider",
                &implicit_provider,
                &HashMap::from([("error".to_string(), "access_denied".to_string())]),
            )
            .err()
            .as_deref(),
            Some("Error during authorization: access_denied")
        );

        assert_eq!(
            extract_callback_payload(
                "provider",
                &implicit_provider,
                &HashMap::from([
                    ("access_token".to_string(), "token".to_string()),
                    ("expires_in".to_string(), "120".to_string()),
                ]),
            )
            .unwrap(),
            CallbackPayload::AccessToken {
                access_token: "token".to_string(),
                expires_in: 120,
            }
        );

        assert_eq!(
            extract_callback_payload(
                "provider",
                &default_ttl_provider,
                &HashMap::from([("access_token".to_string(), "token".to_string())]),
            )
            .unwrap(),
            CallbackPayload::AccessToken {
                access_token: "token".to_string(),
                expires_in: 3600,
            }
        );

        assert_eq!(
            extract_callback_payload(
                "provider",
                &implicit_provider,
                &HashMap::from([("access_token".to_string(), "token".to_string())]),
            )
            .err()
            .as_deref(),
            Some("Missing access token expiration in callback and no default configured")
        );

        assert_eq!(
            extract_callback_payload(
                "provider",
                &code_provider,
                &HashMap::from([("authorization_code".to_string(), "auth-code".to_string())]),
            )
            .unwrap(),
            CallbackPayload::AuthorizationCode {
                code: "auth-code".to_string(),
            }
        );
    }

    #[test]
    fn extract_callback_payload_prefers_access_tokens_and_reports_missing_payload() {
        let hybrid_provider = test_provider()
            .with_callback_access_token_param("access_token")
            .with_callback_code_param("code")
            .with_default_access_token_ttl_secs(90);

        assert_eq!(
            extract_callback_payload(
                "provider",
                &hybrid_provider,
                &HashMap::from([
                    ("access_token".to_string(), "token".to_string()),
                    ("code".to_string(), "auth-code".to_string()),
                ]),
            )
            .unwrap(),
            CallbackPayload::AccessToken {
                access_token: "token".to_string(),
                expires_in: 90,
            }
        );

        assert_eq!(
            extract_callback_payload("provider", &hybrid_provider, &HashMap::new())
                .err()
                .as_deref(),
            Some("Missing callback payload for provider provider; expected one of: access_token, code")
        );
    }

    #[test]
    fn resolve_pkce_verifier_handles_disabled_state_bound_and_standalone_flows() {
        let state = TokenManagerState::default();

        assert_eq!(
            resolve_pkce_verifier(&state, "provider", &test_provider().with_pkce(false), None,)
                .unwrap(),
            None
        );

        let stateful_provider = test_provider();
        assert_eq!(
            resolve_pkce_verifier(&state, "provider", &stateful_provider, None)
                .err()
                .as_deref(),
            Some("Missing OAuth state for PKCE validation")
        );
        assert_eq!(
            resolve_pkce_verifier(
                &state,
                "provider",
                &stateful_provider,
                Some(&("provider".to_string(), None)),
            )
            .err()
            .as_deref(),
            Some("Missing PKCE verifier associated with OAuth state")
        );
        assert_eq!(
            resolve_pkce_verifier(
                &state,
                "provider",
                &stateful_provider,
                Some(&("provider".to_string(), Some("state-verifier".to_string()))),
            )
            .unwrap(),
            Some("state-verifier".to_string())
        );

        let stateless_provider = test_provider().with_state(false);
        state
            .set_pkce_verifier("provider", "standalone-verifier".to_string())
            .expect("pkce verifier should store");
        assert_eq!(
            resolve_pkce_verifier(&state, "provider", &stateless_provider, None).unwrap(),
            Some("standalone-verifier".to_string())
        );
        assert_eq!(
            resolve_pkce_verifier(&state, "provider", &stateless_provider, None).unwrap(),
            None
        );
    }
}
