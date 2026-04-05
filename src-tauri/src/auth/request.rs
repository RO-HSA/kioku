use std::collections::HashMap;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_http::reqwest;

use crate::auth::token_manager::get_access_token;

#[derive(Debug, PartialEq)]
enum OAuthRequestBody {
    Json(serde_json::Value),
    Text(String),
    Empty,
}

#[derive(Debug, PartialEq)]
struct PreparedOAuthRequest {
    provider_id: String,
    method: reqwest::Method,
    url: reqwest::Url,
    headers: Vec<(String, String)>,
    body: OAuthRequestBody,
    timeout: Option<Duration>,
}

fn parse_http_method(method: &str) -> Result<reqwest::Method, String> {
    reqwest::Method::from_bytes(method.to_uppercase().as_bytes()).map_err(|e| e.to_string())
}

fn request_timeout(timeout_ms: Option<u64>) -> Option<Duration> {
    timeout_ms.map(Duration::from_millis)
}

fn parse_oauth_request_url(url: &str) -> Result<reqwest::Url, String> {
    let parsed = reqwest::Url::parse(url).map_err(|e| e.to_string())?;

    if parsed.scheme() != "https" {
        return Err("OAuth requests require an HTTPS URL".to_string());
    }

    if parsed.host_str().is_none() {
        return Err("OAuth request URL must include a host".to_string());
    }

    Ok(parsed)
}

fn select_request_body(
    json_body: Option<serde_json::Value>,
    body: Option<String>,
) -> OAuthRequestBody {
    if let Some(json_body) = json_body {
        OAuthRequestBody::Json(json_body)
    } else if let Some(body) = body {
        OAuthRequestBody::Text(body)
    } else {
        OAuthRequestBody::Empty
    }
}

fn prepare_oauth_request(request: OAuthRequest) -> Result<PreparedOAuthRequest, String> {
    Ok(PreparedOAuthRequest {
        provider_id: request.provider_id,
        method: parse_http_method(&request.method)?,
        url: parse_oauth_request_url(&request.url)?,
        headers: request.headers.unwrap_or_default().into_iter().collect(),
        body: select_request_body(request.json_body, request.body),
        timeout: request_timeout(request.timeout_ms),
    })
}

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
pub async fn oauth_request(app: AppHandle, request: OAuthRequest) -> Result<OAuthResponse, String> {
    let request = prepare_oauth_request(request)?;
    let token = get_access_token(&app, &request.provider_id).await?;

    let mut builder = reqwest::Client::new()
        .request(request.method, request.url)
        .bearer_auth(token);

    if let Some(timeout) = request.timeout {
        builder = builder.timeout(timeout);
    }

    for (key, value) in request.headers {
        builder = builder.header(key, value);
    }

    match request.body {
        OAuthRequestBody::Json(json_body) => {
            builder = builder.json(&json_body);
        }
        OAuthRequestBody::Text(body) => {
            builder = builder.body(body);
        }
        OAuthRequestBody::Empty => {}
    }

    let response = builder.send().await.map_err(|e| e.to_string())?;
    let status = response.status().as_u16();
    let body = response.text().await.map_err(|e| e.to_string())?;

    Ok(OAuthResponse { status, body })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parse_http_method_accepts_common_methods_case_insensitively() {
        assert_eq!(parse_http_method("get").unwrap(), reqwest::Method::GET);
        assert_eq!(parse_http_method("PoSt").unwrap(), reqwest::Method::POST);
        assert!(parse_http_method("not a method").is_err());
    }

    #[test]
    fn request_timeout_maps_milliseconds_to_duration() {
        assert_eq!(request_timeout(None), None);
        assert_eq!(
            request_timeout(Some(1500)),
            Some(Duration::from_millis(1500))
        );
    }

    #[test]
    fn parse_oauth_request_url_allows_https_and_rejects_cleartext_or_invalid_urls() {
        let parsed =
            parse_oauth_request_url("https://example.com/graphql").expect("https url should work");
        assert_eq!(parsed.scheme(), "https");
        assert_eq!(parsed.host_str(), Some("example.com"));

        assert_eq!(
            parse_oauth_request_url("http://example.com/graphql")
                .err()
                .as_deref(),
            Some("OAuth requests require an HTTPS URL")
        );
        assert!(parse_oauth_request_url("not a url").is_err());
    }

    #[test]
    fn select_request_body_prefers_json_and_falls_back_to_text() {
        assert_eq!(
            select_request_body(Some(json!({"ok": true})), Some("ignored".to_string())),
            OAuthRequestBody::Json(json!({"ok": true}))
        );
        assert_eq!(
            select_request_body(None, Some("body".to_string())),
            OAuthRequestBody::Text("body".to_string())
        );
        assert_eq!(select_request_body(None, None), OAuthRequestBody::Empty);
    }

    #[test]
    fn prepare_oauth_request_normalizes_method_headers_timeout_and_body() {
        let request = OAuthRequest {
            provider_id: "anilist".to_string(),
            method: "pAtCh".to_string(),
            url: "https://example.com/graphql".to_string(),
            headers: Some(HashMap::from([
                ("X-Test".to_string(), "1".to_string()),
                ("Content-Type".to_string(), "application/json".to_string()),
            ])),
            body: Some("ignored".to_string()),
            json_body: Some(json!({"ok": true})),
            timeout_ms: Some(2500),
        };

        let prepared = prepare_oauth_request(request).expect("request should prepare");

        assert_eq!(prepared.provider_id, "anilist");
        assert_eq!(prepared.method, reqwest::Method::PATCH);
        assert_eq!(prepared.url.as_str(), "https://example.com/graphql");
        assert_eq!(prepared.timeout, Some(Duration::from_millis(2500)));
        assert_eq!(prepared.body, OAuthRequestBody::Json(json!({"ok": true})));
        assert_eq!(prepared.headers.len(), 2);
        assert!(prepared
            .headers
            .contains(&("X-Test".to_string(), "1".to_string())));
        assert!(prepared
            .headers
            .contains(&("Content-Type".to_string(), "application/json".to_string())));
    }

    #[test]
    fn prepare_oauth_request_rejects_invalid_methods_and_handles_missing_optionals() {
        let invalid = OAuthRequest {
            provider_id: "mal".to_string(),
            method: "not a method".to_string(),
            url: "https://example.com".to_string(),
            headers: None,
            body: None,
            json_body: None,
            timeout_ms: None,
        };
        assert!(prepare_oauth_request(invalid).is_err());

        let insecure_url = OAuthRequest {
            provider_id: "mal".to_string(),
            method: "get".to_string(),
            url: "http://example.com".to_string(),
            headers: None,
            body: None,
            json_body: None,
            timeout_ms: None,
        };
        assert_eq!(
            prepare_oauth_request(insecure_url).err().as_deref(),
            Some("OAuth requests require an HTTPS URL")
        );

        let request = OAuthRequest {
            provider_id: "mal".to_string(),
            method: "get".to_string(),
            url: "https://example.com".to_string(),
            headers: None,
            body: Some("body".to_string()),
            json_body: None,
            timeout_ms: None,
        };
        let prepared = prepare_oauth_request(request).expect("request should prepare");

        assert_eq!(prepared.provider_id, "mal");
        assert_eq!(prepared.method, reqwest::Method::GET);
        assert!(prepared.headers.is_empty());
        assert_eq!(prepared.timeout, None);
        assert_eq!(prepared.body, OAuthRequestBody::Text("body".to_string()));
    }
}
