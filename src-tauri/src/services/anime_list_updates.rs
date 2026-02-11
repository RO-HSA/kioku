use std::time::Duration;

use serde::Deserialize;
use tauri::Manager;
use tauri_plugin_http::reqwest;
use tokio::sync::mpsc;

use crate::auth::mal::PROVIDER_ID as MAL_PROVIDER_ID;
use crate::services::myanimelist::update_myanimelist_list_entry;

const UPDATE_INTERVAL_MS: u64 = 1000;
const UPDATE_QUEUE_CAPACITY: usize = 256;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnimeListUpdateRequest {
    pub provider_id: String,
    #[serde(alias = "id")]
    pub entry_id: u64,
    pub user_status: Option<String>,
    pub user_score: Option<u32>,
    pub user_episodes_watched: Option<u32>,
    pub is_rewatching: Option<bool>,
    pub user_comments: Option<String>,
    pub user_num_times_rewatched: Option<u32>,
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
                    "Anime list update failed (provider={}, entry_id={}): {}",
                    update.provider_id, update.entry_id, err
                );
            }

            tokio::time::sleep(interval).await;
        }
    });
}

async fn handle_update(
    app: &tauri::AppHandle,
    client: &reqwest::Client,
    update: &AnimeListUpdateRequest,
) -> Result<(), String> {
    match update.provider_id.as_str() {
        MAL_PROVIDER_ID => update_myanimelist_list_entry(app, client, update).await,
        _ => Err(format!("Provider not supported: {}", update.provider_id)),
    }
}
