use serde::Deserialize;
use serde_json::{Map, Value};
use tauri_plugin_http::reqwest;
use tauri_plugin_zustand::ManagerExt;

use crate::auth::mal::PROVIDER_ID as MAL_PROVIDER_ID;
use crate::auth::token_manager::get_access_token;

const BASE_URL: &str = "https://api.myanimelist.net/v2/users/";
const FIELDS: &str = "list_status,synopsis,alternative_titles,source,num_episodes,nsfw,start_season,media_type,studios,mean,status,genres";
const LIMIT: u32 = 1000;
const OUTPUT_STATUSES: [&str; 5] = [
    "watching",
    "completed",
    "on_hold",
    "dropped",
    "plan_to_watch",
];

#[derive(Deserialize)]
struct MalListResponse {
    data: Vec<Value>,
    paging: Option<MalPaging>,
}

#[derive(Deserialize)]
struct MalPaging {
    next: Option<String>,
}

fn build_animelist_url(username: &str, offset: u32) -> Result<String, String> {
    let mut url = reqwest::Url::parse(BASE_URL).map_err(|e| e.to_string())?;
    {
        let mut segments = url
            .path_segments_mut()
            .map_err(|_| "Invalid MyAnimeList base URL".to_string())?;
        segments.push(username);
        segments.push("animelist");
    }

    url.query_pairs_mut()
        .append_pair("fields", FIELDS)
        .append_pair("limit", &LIMIT.to_string())
        .append_pair("offset", &offset.to_string());

    Ok(url.to_string())
}

fn parse_next_offset(next_url: &str) -> Option<u32> {
    let url = reqwest::Url::parse(next_url).ok()?;
    url.query_pairs()
        .find(|(key, _)| key == "offset")
        .and_then(|(_, value)| value.parse::<u32>().ok())
}

async fn fetch_all(
    client: &reqwest::Client,
    token: &str,
    username: &str,
) -> Result<Vec<Value>, String> {
    let mut results: Vec<Value> = Vec::new();
    let mut offset: u32 = 0;

    loop {
        let url = build_animelist_url(username, offset)?;
        let response = client
            .get(url)
            .bearer_auth(token)
            .timeout(std::time::Duration::from_secs(15))
            .send()
            .await
            .map_err(|e| e.to_string())?;
        let status_code = response.status();
        let body = response.text().await.map_err(|e| e.to_string())?;

        if !status_code.is_success() {
            return Err(format!(
                "MyAnimeList request failed: {} - {}",
                status_code, body
            ));
        }

        let parsed: MalListResponse =
            serde_json::from_str(&body).map_err(|e| e.to_string())?;
        results.extend(parsed.data);

        let next = parsed.paging.and_then(|p| p.next);
        let Some(next_url) = next else { break };
        offset = parse_next_offset(&next_url).unwrap_or(offset + LIMIT);
    }

    Ok(results)
}

#[tauri::command]
pub async fn synchronize_myanimelist(
    app: tauri::AppHandle,
) -> Result<Value, String> {
    let token = get_access_token(&app, MAL_PROVIDER_ID).await?;
    let username: Option<String> = app
        .zustand()
        .get_or_default("myanimelist", "username");
    let username = username
        .and_then(|value| {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        })
        .unwrap_or_else(|| "@me".to_string());
    let client = reqwest::Client::new();

    let mut result = Map::new();
    for status in OUTPUT_STATUSES {
        result.insert(status.to_string(), Value::Array(Vec::new()));
    }

    let items = fetch_all(&client, &token, &username).await?;
    for item in items {
        let status = item
            .get("list_status")
            .and_then(|list_status| list_status.get("status"))
            .and_then(|status| status.as_str());
        if let Some(status) = status {
            if let Some(Value::Array(list)) = result.get_mut(status) {
                list.push(item);
            }
        }
    }

    Ok(Value::Object(result))
}
