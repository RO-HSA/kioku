use std::collections::HashMap;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_http::reqwest;

use crate::auth::token_manager::get_access_token;

#[derive(Deserialize)]
pub struct OAuthRequest {
    pub provider_id: String,
    pub method: String,
    pub url: String,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<String>,
    pub json_body: Option<serde_json::Value>,
    pub timeout_ms: Option<u64>,
}

#[derive(Serialize)]
pub struct OAuthResponse {
    pub status: u16,
    pub body: String,
}

#[tauri::command]
pub async fn oauth_request(
    app: AppHandle,
    request: OAuthRequest,
) -> Result<OAuthResponse, String> {
    let token = get_access_token(&app, &request.provider_id).await?;
    let method = reqwest::Method::from_bytes(request.method.to_uppercase().as_bytes())
        .map_err(|e| e.to_string())?;

    let mut builder = reqwest::Client::new()
        .request(method, &request.url)
        .bearer_auth(token);

    if let Some(timeout_ms) = request.timeout_ms {
        builder = builder.timeout(Duration::from_millis(timeout_ms));
    }

    if let Some(headers) = request.headers {
        for (key, value) in headers {
            builder = builder.header(key, value);
        }
    }

    if let Some(json_body) = request.json_body {
        builder = builder.json(&json_body);
    } else if let Some(body) = request.body {
        builder = builder.body(body);
    }

    let response = builder.send().await.map_err(|e| e.to_string())?;
    let status = response.status().as_u16();
    let body = response.text().await.map_err(|e| e.to_string())?;

    Ok(OAuthResponse { status, body })
}
