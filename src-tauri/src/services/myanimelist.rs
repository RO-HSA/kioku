use serde::Deserialize;
use serde_json::{Map, Value};
use tauri_plugin_http::reqwest;

use crate::auth::mal::PROVIDER_ID as MAL_PROVIDER_ID;
use crate::auth::token_manager::get_access_token;

const BASE_URL: &str = "https://api.myanimelist.net/v2/users/@me/animelist";
const FIELDS: &str = "list_status,synopsis,alternative_titles,source,num_episodes,nsfw,start_season,media_type,studios,mean";
const LIMIT: u32 = 100;
const STATUSES: [&str; 5] = ["watching", "completed", "on_hold", "dropped", "plan_to_watch"];

#[derive(Deserialize)]
struct MalListResponse {
    data: Vec<Value>,
    paging: Option<MalPaging>,
}

#[derive(Deserialize)]
struct MalPaging {
    next: Option<String>,
}

fn build_animelist_url(status: &str, offset: u32) -> Result<String, String> {
    let params = vec![
        ("fields".to_string(), FIELDS.to_string()),
        ("status".to_string(), status.to_string()),
        ("limit".to_string(), LIMIT.to_string()),
        ("offset".to_string(), offset.to_string()),
    ];
    reqwest::Url::parse_with_params(BASE_URL, params)
        .map_err(|e| e.to_string())
        .map(|url| url.to_string())
}

fn parse_next_offset(next_url: &str) -> Option<u32> {
    let url = reqwest::Url::parse(next_url).ok()?;
    url.query_pairs()
        .find(|(key, _)| key == "offset")
        .and_then(|(_, value)| value.parse::<u32>().ok())
}

async fn fetch_status(
    client: &reqwest::Client,
    token: &str,
    status: &str,
) -> Result<Vec<Value>, String> {
    let mut results: Vec<Value> = Vec::new();
    let mut offset: u32 = 0;

    loop {
        let url = build_animelist_url(status, offset)?;
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
                "MyAnimeList request failed ({status}): {} - {}",
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
    let client = reqwest::Client::new();

    let mut handles = Vec::with_capacity(STATUSES.len());
    for status in STATUSES {
        let client = client.clone();
        let token = token.clone();
        let status = status.to_string();
        handles.push(tauri::async_runtime::spawn(async move {
            let items = fetch_status(&client, &token, &status).await?;
            Ok::<(String, Vec<Value>), String>((status, items))
        }));
    }

    let mut result = Map::new();
    for handle in handles {
        let (status, items) = handle
            .await
            .map_err(|e| e.to_string())??;
        result.insert(status, Value::Array(items));
    }

    Ok(Value::Object(result))
}
