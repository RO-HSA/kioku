use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, SystemTime};

use serde::Deserialize;
use serde_json;
use tauri::{AppHandle, Manager};
use tauri_plugin_http::reqwest;

use crate::auth::anilist::PROVIDER_ID as ANILIST_PROVIDER_ID;
use crate::auth::mal::PROVIDER_ID as MAL_PROVIDER_ID;
use crate::auth::secure_store::{
    read_access_token, read_refresh_token, save_access_token, save_refresh_token,
};

const REFRESH_EARLY_SECS: u64 = 60;

#[derive(Clone)]
pub struct ProviderConfig {
    pub client_id: String,
    pub authorize_url: String,
    pub token_url: String,
    pub use_pkce: bool,
    pub authorize_extra_params: Vec<(String, String)>,
    pub token_extra_params: Vec<(String, String)>,
    pub refresh_extra_params: Vec<(String, String)>,
}

impl ProviderConfig {
    pub fn new(
        client_id: impl Into<String>,
        authorize_url: impl Into<String>,
        token_url: impl Into<String>,
    ) -> Self {
        Self {
            client_id: client_id.into(),
            authorize_url: authorize_url.into(),
            token_url: token_url.into(),
            use_pkce: true,
            authorize_extra_params: Vec::new(),
            token_extra_params: Vec::new(),
            refresh_extra_params: Vec::new(),
        }
    }

    pub fn with_pkce(mut self, enabled: bool) -> Self {
        self.use_pkce = enabled;
        self
    }

    pub fn with_authorize_param(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.authorize_extra_params.push((key.into(), value.into()));
        self
    }

    pub fn with_token_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.token_extra_params.push((key.into(), value.into()));
        self
    }

    pub fn with_refresh_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.refresh_extra_params.push((key.into(), value.into()));
        self
    }
}

#[derive(Default)]
pub struct TokenManagerState(Mutex<TokenManagerInner>);

#[derive(Default)]
struct TokenManagerInner {
    providers: HashMap<String, ProviderConfig>,
    tokens: HashMap<String, TokenState>,
    pkce_verifiers: HashMap<String, String>,
    pkce_states: HashMap<String, (String, String)>,
}

#[derive(Default)]
struct TokenState {
    access_token: Option<String>,
    expires_at: Option<SystemTime>,
}

#[derive(Deserialize, Clone)]
pub struct TokenResponse {
    pub token_type: String,
    pub expires_in: u64,
    pub access_token: String,
    pub refresh_token: Option<String>,
}

impl TokenManagerState {
    pub fn register_provider(
        &self,
        provider_id: &str,
        config: ProviderConfig,
    ) -> Result<(), String> {
        let mut guard = self
            .0
            .lock()
            .map_err(|_| "Token manager lock poisoned".to_string())?;
        guard.providers.insert(provider_id.to_string(), config);
        Ok(())
    }

    pub fn get_provider(&self, provider_id: &str) -> Result<ProviderConfig, String> {
        let guard = self
            .0
            .lock()
            .map_err(|_| "Token manager lock poisoned".to_string())?;
        guard
            .providers
            .get(provider_id)
            .cloned()
            .ok_or_else(|| format!("Provider not registered: {provider_id}"))
    }

    pub fn set_pkce_verifier(
        &self,
        provider_id: &str,
        code_verifier: String,
    ) -> Result<(), String> {
        let mut guard = self
            .0
            .lock()
            .map_err(|_| "Token manager lock poisoned".to_string())?;
        guard
            .pkce_verifiers
            .insert(provider_id.to_string(), code_verifier);
        Ok(())
    }

    pub fn take_pkce_verifier(&self, provider_id: &str) -> Result<Option<String>, String> {
        let mut guard = self
            .0
            .lock()
            .map_err(|_| "Token manager lock poisoned".to_string())?;
        Ok(guard.pkce_verifiers.remove(provider_id))
    }

    pub fn set_pkce_state(
        &self,
        state: String,
        provider_id: &str,
        code_verifier: String,
    ) -> Result<(), String> {
        let mut guard = self
            .0
            .lock()
            .map_err(|_| "Token manager lock poisoned".to_string())?;
        guard
            .pkce_states
            .insert(state, (provider_id.to_string(), code_verifier));
        Ok(())
    }

    pub fn take_pkce_state(&self, state: &str) -> Result<Option<(String, String)>, String> {
        let mut guard = self
            .0
            .lock()
            .map_err(|_| "Token manager lock poisoned".to_string())?;
        Ok(guard.pkce_states.remove(state))
    }

    pub fn get_pkce_state_provider(&self, state: &str) -> Result<Option<String>, String> {
        let guard = self
            .0
            .lock()
            .map_err(|_| "Token manager lock poisoned".to_string())?;
        Ok(guard
            .pkce_states
            .get(state)
            .map(|(provider_id, _)| provider_id.clone()))
    }

    fn set_access_token(
        &self,
        provider_id: &str,
        access_token: String,
        expires_in: u64,
    ) -> Result<(), String> {
        let expires_at = SystemTime::now()
            .checked_add(Duration::from_secs(expires_in))
            .ok_or_else(|| "Failed to compute access token expiration".to_string())?;
        let mut guard = self
            .0
            .lock()
            .map_err(|_| "Token manager lock poisoned".to_string())?;
        guard.tokens.insert(
            provider_id.to_string(),
            TokenState {
                access_token: Some(access_token),
                expires_at: Some(expires_at),
            },
        );
        Ok(())
    }

    fn get_valid_access_token(&self, provider_id: &str) -> Result<Option<String>, String> {
        let guard = self
            .0
            .lock()
            .map_err(|_| "Token manager lock poisoned".to_string())?;

        let Some(state) = guard.tokens.get(provider_id) else {
            return Ok(None);
        };
        let Some(token) = state.access_token.as_ref() else {
            return Ok(None);
        };
        let Some(expires_at) = state.expires_at else {
            return Ok(None);
        };

        let refresh_at = expires_at
            .checked_sub(Duration::from_secs(REFRESH_EARLY_SECS))
            .unwrap_or(expires_at);
        if SystemTime::now() >= refresh_at {
            return Ok(None);
        }

        Ok(Some(token.clone()))
    }
}

pub fn store_tokens(
    app: &AppHandle,
    provider_id: &str,
    response: &TokenResponse,
) -> Result<(), String> {
    store_access_token(
        app,
        provider_id,
        &response.access_token,
        response.expires_in,
    )?;

    if let Some(refresh_token) = response.refresh_token.as_ref() {
        save_refresh_token(app, provider_id, refresh_token)?;
    }

    Ok(())
}

pub fn store_access_token(
    app: &AppHandle,
    provider_id: &str,
    access_token: &str,
    expires_in: u64,
) -> Result<(), String> {
    app.state::<TokenManagerState>().set_access_token(
        provider_id,
        access_token.to_string(),
        expires_in,
    )?;

    save_access_token(
        app,
        provider_id,
        access_token,
        compute_expires_at_unix_secs(expires_in)?,
    )
}

pub async fn get_access_token(app: &AppHandle, provider_id: &str) -> Result<String, String> {
    if let Some(token) = app
        .state::<TokenManagerState>()
        .get_valid_access_token(provider_id)?
    {
        return Ok(token);
    }

    if let Some(token) = restore_access_token_from_store(app, provider_id)? {
        return Ok(token);
    }

    if provider_id == ANILIST_PROVIDER_ID {
        return Err("AniList access token expired or missing; reauthorize AniList".to_string());
    }

    refresh_access_token(app, provider_id).await
}

pub fn build_authorize_url(
    app: &AppHandle,
    provider_id: &str,
    code_challenge: Option<&str>,
    state: Option<&str>,
) -> Result<String, String> {
    let ProviderConfig {
        client_id,
        authorize_url,
        use_pkce,
        authorize_extra_params,
        ..
    } = app.state::<TokenManagerState>().get_provider(provider_id)?;

    let mut params: Vec<(String, String)> =
        Vec::with_capacity(3 + authorize_extra_params.len() + 1);

    if provider_id == MAL_PROVIDER_ID {
        params.push(("response_type".to_string(), "code".to_string()));
    } else if provider_id == ANILIST_PROVIDER_ID {
        params.push(("response_type".to_string(), "token".to_string()));
    }

    params.push(("client_id".to_string(), client_id));

    if use_pkce {
        let challenge = code_challenge.ok_or_else(|| "Missing PKCE code challenge".to_string())?;
        params.push(("code_challenge".to_string(), challenge.to_string()));
    }

    if let Some(state) = state {
        if provider_id == MAL_PROVIDER_ID {
            params.push(("state".to_string(), state.to_string()));
        }
    }

    params.extend(authorize_extra_params);

    reqwest::Url::parse_with_params(&authorize_url, params)
        .map_err(|e| e.to_string())
        .map(|url| url.to_string())
}

pub async fn exchange_authorization_code(
    app: &AppHandle,
    provider_id: &str,
    code: &str,
    code_verifier: Option<String>,
) -> Result<TokenResponse, String> {
    let ProviderConfig {
        client_id,
        token_url,
        use_pkce,
        token_extra_params,
        ..
    } = app.state::<TokenManagerState>().get_provider(provider_id)?;

    if token_url.trim().is_empty() {
        return Err(format!(
            "Missing token URL configuration for provider {provider_id}"
        ));
    }

    if use_pkce && code_verifier.is_none() {
        return Err("Missing PKCE code verifier".to_string());
    }

    let mut params: Vec<(String, String)> = Vec::with_capacity(3 + token_extra_params.len() + 1);
    params.push(("grant_type".to_string(), "authorization_code".to_string()));
    params.push(("client_id".to_string(), client_id));
    params.push(("code".to_string(), code.to_string()));
    if let Some(verifier) = code_verifier {
        params.push(("code_verifier".to_string(), verifier));
    }
    params.extend(token_extra_params);

    let response = reqwest::Client::new()
        .post(&token_url)
        .form(&params)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let status = response.status();

    let body = response.text().await.map_err(|e| e.to_string())?;
    if !status.is_success() {
        return Err(format!("Token exchange failed: {} - {}", status, body));
    }

    let token_response = serde_json::from_str::<TokenResponse>(&body).map_err(|e| e.to_string())?;

    store_tokens(app, provider_id, &token_response)?;
    Ok(token_response)
}

async fn refresh_access_token(app: &AppHandle, provider_id: &str) -> Result<String, String> {
    let refresh_token = read_refresh_token(app, provider_id)?
        .ok_or_else(|| "No refresh token stored".to_string())?;

    let ProviderConfig {
        client_id,
        authorize_url: _,
        token_url,
        use_pkce: _,
        authorize_extra_params: _,
        token_extra_params: _,
        refresh_extra_params,
    } = app.state::<TokenManagerState>().get_provider(provider_id)?;

    if token_url.trim().is_empty() {
        return Err(format!(
            "Missing token URL configuration for provider {provider_id}"
        ));
    }

    let mut params: Vec<(String, String)> = Vec::with_capacity(3 + refresh_extra_params.len());
    params.push(("client_id".to_string(), client_id));
    params.push(("grant_type".to_string(), "refresh_token".to_string()));
    params.push(("refresh_token".to_string(), refresh_token));
    params.extend(refresh_extra_params);

    let response = reqwest::Client::new()
        .post(&token_url)
        .form(&params)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let token_response = response
        .json::<TokenResponse>()
        .await
        .map_err(|e| e.to_string())?;

    let access_token = token_response.access_token.clone();
    store_tokens(app, provider_id, &token_response)?;
    Ok(access_token)
}

fn compute_expires_at_unix_secs(expires_in: u64) -> Result<u64, String> {
    let expires_at = SystemTime::now()
        .checked_add(Duration::from_secs(expires_in))
        .ok_or_else(|| "Failed to compute access token expiration".to_string())?;

    expires_at
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|_| "Computed access token expiration is before UNIX epoch".to_string())
        .map(|value| value.as_secs())
}

fn restore_access_token_from_store(
    app: &AppHandle,
    provider_id: &str,
) -> Result<Option<String>, String> {
    let Some((token, expires_at_unix_secs)) = read_access_token(app, provider_id)? else {
        return Ok(None);
    };

    let expires_at = SystemTime::UNIX_EPOCH
        .checked_add(Duration::from_secs(expires_at_unix_secs))
        .ok_or_else(|| "Stored access token expiration is invalid".to_string())?;
    let refresh_at = expires_at
        .checked_sub(Duration::from_secs(REFRESH_EARLY_SECS))
        .unwrap_or(expires_at);
    let now = SystemTime::now();

    if now >= refresh_at {
        return Ok(None);
    }

    let remaining_secs = expires_at
        .duration_since(now)
        .map_err(|_| "Stored access token is expired".to_string())?
        .as_secs();

    if remaining_secs == 0 {
        return Ok(None);
    }

    app.state::<TokenManagerState>().set_access_token(
        provider_id,
        token.clone(),
        remaining_secs,
    )?;

    Ok(Some(token))
}
