use std::collections::HashMap;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_http::reqwest;

use crate::auth::anilist::PROVIDER_ID as ANILIST_PROVIDER_ID;
use crate::auth::mal::PROVIDER_ID as MAL_PROVIDER_ID;
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

fn allowed_oauth_request_hosts(provider_id: &str) -> Option<&'static [&'static str]> {
    match provider_id {
        ANILIST_PROVIDER_ID => Some(&["graphql.anilist.co"]),
        MAL_PROVIDER_ID => Some(&["api.myanimelist.net"]),
        _ => None,
    }
}

fn ensure_oauth_request_url(provider_id: &str, parsed: &reqwest::Url) -> Result<(), String> {
    if parsed.scheme() != "https" {
        return Err("OAuth requests require an HTTPS URL".to_string());
    }

    let host = parsed
        .host_str()
        .ok_or_else(|| "OAuth request URL must include a host".to_string())?;
    let allowed_hosts = allowed_oauth_request_hosts(provider_id)
        .ok_or_else(|| format!("OAuth requests are not allowed for provider {provider_id}"))?;

    if !allowed_hosts
        .iter()
        .any(|allowed_host| host == *allowed_host)
    {
        return Err(format!(
            "OAuth request host is not allowed for provider {provider_id}"
        ));
    }

    Ok(())
}

fn parse_oauth_request_url(provider_id: &str, url: &str) -> Result<reqwest::Url, String> {
    let parsed = reqwest::Url::parse(url).map_err(|e| e.to_string())?;
    ensure_oauth_request_url(provider_id, &parsed)?;
    Ok(parsed)
}

fn build_oauth_http_client() -> Result<reqwest::Client, String> {
    reqwest::Client::builder()
        .https_only(true)
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|e| e.to_string())
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
        provider_id: request.provider_id.clone(),
        method: parse_http_method(&request.method)?,
        url: parse_oauth_request_url(&request.provider_id, &request.url)?,
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
    ensure_oauth_request_url(&request.provider_id, &request.url)?;
    let token = get_access_token(&app, &request.provider_id).await?;
    let client = build_oauth_http_client()?;

    let mut builder = client.request(request.method, request.url).bearer_auth(token);

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
    fn parse_oauth_request_url_allows_known_provider_hosts_and_rejects_others() {
        let parsed =
            parse_oauth_request_url(ANILIST_PROVIDER_ID, "https://graphql.anilist.co/query")
                .expect("anilist graphql url should work");
        assert_eq!(parsed.scheme(), "https");
        assert_eq!(parsed.host_str(), Some("graphql.anilist.co"));

        let mal =
            parse_oauth_request_url(MAL_PROVIDER_ID, "https://api.myanimelist.net/v2/users/@me")
                .expect("mal api url should work");
        assert_eq!(mal.host_str(), Some("api.myanimelist.net"));

        assert_eq!(
            parse_oauth_request_url(ANILIST_PROVIDER_ID, "http://graphql.anilist.co/query")
                .err()
                .as_deref(),
            Some("OAuth requests require an HTTPS URL")
        );
        assert!(parse_oauth_request_url(ANILIST_PROVIDER_ID, "not a url").is_err());
        assert_eq!(
            parse_oauth_request_url(ANILIST_PROVIDER_ID, "https://api.myanimelist.net/v2/users")
                .err()
                .as_deref(),
            Some("OAuth request host is not allowed for provider anilist")
        );
        assert_eq!(
            parse_oauth_request_url("unknown", "https://example.com")
                .err()
                .as_deref(),
            Some("OAuth requests are not allowed for provider unknown")
        );
    }

    #[test]
    fn ensure_oauth_request_url_revalidates_prepared_urls() {
        let secure = reqwest::Url::parse("https://api.myanimelist.net/v2/anime")
            .expect("url should parse");
        ensure_oauth_request_url(MAL_PROVIDER_ID, &secure).expect("known https host should pass");

        let insecure =
            reqwest::Url::parse("http://api.myanimelist.net/v2/anime").expect("url should parse");
        assert_eq!(
            ensure_oauth_request_url(MAL_PROVIDER_ID, &insecure)
                .err()
                .as_deref(),
            Some("OAuth requests require an HTTPS URL")
        );
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
            url: "https://graphql.anilist.co/query".to_string(),
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
        assert_eq!(prepared.url.as_str(), "https://graphql.anilist.co/query");
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
            provider_id: MAL_PROVIDER_ID.to_string(),
            method: "not a method".to_string(),
            url: "https://api.myanimelist.net/v2/anime".to_string(),
            headers: None,
            body: None,
            json_body: None,
            timeout_ms: None,
        };
        assert!(prepare_oauth_request(invalid).is_err());

        let insecure_url = OAuthRequest {
            provider_id: MAL_PROVIDER_ID.to_string(),
            method: "get".to_string(),
            url: "http://api.myanimelist.net/v2/anime".to_string(),
            headers: None,
            body: None,
            json_body: None,
            timeout_ms: None,
        };
        assert_eq!(
            prepare_oauth_request(insecure_url).err().as_deref(),
            Some("OAuth requests require an HTTPS URL")
        );

        let wrong_host = OAuthRequest {
            provider_id: MAL_PROVIDER_ID.to_string(),
            method: "get".to_string(),
            url: "https://graphql.anilist.co/query".to_string(),
            headers: None,
            body: None,
            json_body: None,
            timeout_ms: None,
        };
        assert_eq!(
            prepare_oauth_request(wrong_host).err().as_deref(),
            Some("OAuth request host is not allowed for provider myanimelist")
        );

        let request = OAuthRequest {
            provider_id: MAL_PROVIDER_ID.to_string(),
            method: "get".to_string(),
            url: "https://api.myanimelist.net/v2/anime".to_string(),
            headers: None,
            body: Some("body".to_string()),
            json_body: None,
            timeout_ms: None,
        };
        let prepared = prepare_oauth_request(request).expect("request should prepare");

        assert_eq!(prepared.provider_id, MAL_PROVIDER_ID);
        assert_eq!(prepared.method, reqwest::Method::GET);
        assert!(prepared.headers.is_empty());
        assert_eq!(prepared.timeout, None);
        assert_eq!(prepared.body, OAuthRequestBody::Text("body".to_string()));
    }
}
