use serde_json::{Value, json};

use crate::auth::{mal::PROVIDER_ID as MAL_PROVIDER_ID};
use crate::auth::request::{OAuthRequest, oauth_request};

#[tauri::command]
pub async fn synchronize_myanimelist(
    app: tauri::AppHandle,
) -> Result<Value, String> {
    let request = OAuthRequest {
        provider_id: MAL_PROVIDER_ID.to_string(),
        method: "GET".to_string(),
        url: "https://api.myanimelist.net/v2/users/@me/animelist".to_string(),
        headers: None,
        body: None,
        json_body: None,
        timeout_ms: Some(15_000),
    };

    let response = oauth_request(app, request).await?;
    Ok(json!(response.body))
}
