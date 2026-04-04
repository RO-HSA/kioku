use std::time::Duration;

use serde::Deserialize;
use tauri::Manager;
use tauri_plugin_http::reqwest;
use tokio::sync::mpsc;

use crate::auth::anilist::PROVIDER_ID as ANILIST_PROVIDER_ID;
use crate::auth::mal::PROVIDER_ID as MAL_PROVIDER_ID;
use crate::services::anilist::update_anilist_list_entry;
use crate::services::myanimelist::update_myanimelist_list_entry;

const UPDATE_INTERVAL_MS: u64 = 1000;
const UPDATE_QUEUE_CAPACITY: usize = 256;

#[derive(Debug, Copy, Clone, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ListType {
    #[default]
    Anime,
    Manga,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnimeListUpdateRequest {
    pub provider_id: String,
    #[serde(default)]
    pub list_type: Option<ListType>,
    #[serde(alias = "id")]
    pub entry_id: Option<u64>,
    #[serde(default, alias = "mediaId")]
    pub media_id: Option<u64>,
    pub user_status: Option<String>,
    pub user_score: Option<u32>,
    pub user_episodes_watched: Option<u32>,
    pub user_volumes_read: Option<u32>,
    pub user_chapters_read: Option<u32>,
    pub is_rewatching: Option<bool>,
    pub is_rereading: Option<bool>,
    pub user_comments: Option<String>,
    pub user_num_times_rewatched: Option<u32>,
    pub user_num_times_reread: Option<u32>,
    pub user_start_date: Option<String>,
    pub user_finish_date: Option<String>,
}

pub struct AnimeListUpdateQueue {
    sender: mpsc::Sender<AnimeListUpdateRequest>,
}

impl AnimeListUpdateQueue {
    pub fn new(app: tauri::AppHandle) -> Self {
        let (sender, receiver) = mpsc::channel(UPDATE_QUEUE_CAPACITY);
        spawn_update_worker(app, receiver);
        Self { sender }
    }

    pub async fn enqueue(&self, update: AnimeListUpdateRequest) -> Result<(), String> {
        self.sender
            .send(update)
            .await
            .map_err(|_| "Update queue is unavailable".to_string())
    }
}

#[tauri::command]
pub async fn enqueue_anime_list_update(
    update: AnimeListUpdateRequest,
    app: tauri::AppHandle,
) -> Result<(), String> {
    app.state::<AnimeListUpdateQueue>().enqueue(update).await
}

fn spawn_update_worker(
    app: tauri::AppHandle,
    mut receiver: mpsc::Receiver<AnimeListUpdateRequest>,
) {
    tauri::async_runtime::spawn(async move {
        let client = reqwest::Client::new();
        let interval = Duration::from_millis(UPDATE_INTERVAL_MS);

        while let Some(update) = receiver.recv().await {
            if let Err(err) = handle_update(&app, &client, &update).await {
                eprintln!(
                    "Anime list update failed (provider={}, entry_id={:?}): {}",
                    update.provider_id, update.entry_id, err
                );
            }

            tokio::time::sleep(interval).await;
        }
    });
}

fn validate_supported_provider(provider_id: &str) -> Result<(), String> {
    match provider_id {
        ANILIST_PROVIDER_ID | MAL_PROVIDER_ID => Ok(()),
        _ => Err(format!("Provider not supported: {provider_id}")),
    }
}

async fn handle_update(
    app: &tauri::AppHandle,
    client: &reqwest::Client,
    update: &AnimeListUpdateRequest,
) -> Result<(), String> {
    validate_supported_provider(&update.provider_id)?;

    match update.provider_id.as_str() {
        ANILIST_PROVIDER_ID => update_anilist_list_entry(app, client, update).await,
        MAL_PROVIDER_ID => update_myanimelist_list_entry(app, client, update).await,
        _ => unreachable!("provider should have been validated"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_type_defaults_to_anime_and_request_aliases_deserialize() {
        let request: AnimeListUpdateRequest = serde_json::from_value(serde_json::json!({
            "providerId": "myanimelist",
            "id": 42,
            "mediaId": 7,
            "userStatus": "completed"
        }))
        .expect("request should deserialize");

        assert_eq!(request.provider_id, "myanimelist");
        assert!(matches!(
            request.list_type.unwrap_or_default(),
            ListType::Anime
        ));
        assert_eq!(request.entry_id, Some(42));
        assert_eq!(request.media_id, Some(7));
        assert_eq!(request.user_status.as_deref(), Some("completed"));
    }

    #[test]
    fn validate_supported_provider_accepts_known_ids_and_rejects_others() {
        assert!(validate_supported_provider(ANILIST_PROVIDER_ID).is_ok());
        assert!(validate_supported_provider(MAL_PROVIDER_ID).is_ok());
        assert_eq!(
            validate_supported_provider("unknown").unwrap_err(),
            "Provider not supported: unknown"
        );
    }
}
