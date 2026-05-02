use std::{
    any::Any,
    collections::VecDeque,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use serde::Deserialize;
use tauri::Manager;
use tauri_plugin_http::reqwest;
use tokio::sync::{Mutex, Notify};

use crate::auth::anilist::PROVIDER_ID as ANILIST_PROVIDER_ID;
use crate::auth::mal::PROVIDER_ID as MAL_PROVIDER_ID;
use crate::services::anilist::update_anilist_list_entry;
use crate::services::myanimelist::update_myanimelist_list_entry;

const UPDATE_INTERVAL_MS: u64 = 1000;
const UPDATE_QUEUE_CAPACITY: usize = 256;
const WORKER_RESTART_DELAY_MS: u64 = 1000;
const WORKER_CRASH_RETRY_LIMIT: u8 = 3;

macro_rules! update_worker_log {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            eprintln!($($arg)*);
        }
    };
}

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

#[derive(Debug, Clone)]
struct QueuedAnimeListUpdate {
    request: AnimeListUpdateRequest,
    crash_retries: u8,
}

impl QueuedAnimeListUpdate {
    fn new(request: AnimeListUpdateRequest) -> Self {
        Self {
            request,
            crash_retries: 0,
        }
    }

    fn schedule_retry(mut self) -> Option<Self> {
        if self.crash_retries >= WORKER_CRASH_RETRY_LIMIT {
            return None;
        }

        self.crash_retries += 1;
        Some(self)
    }
}

struct PendingAnimeListUpdates {
    items: Mutex<VecDeque<QueuedAnimeListUpdate>>,
    notify: Notify,
    capacity: usize,
}

impl PendingAnimeListUpdates {
    fn new(capacity: usize) -> Self {
        Self {
            items: Mutex::new(VecDeque::with_capacity(capacity)),
            notify: Notify::new(),
            capacity,
        }
    }

    async fn enqueue(&self, update: AnimeListUpdateRequest) -> Result<(), String> {
        self.push_back(QueuedAnimeListUpdate::new(update)).await
    }

    async fn push_back(&self, update: QueuedAnimeListUpdate) -> Result<(), String> {
        let mut items = self.items.lock().await;
        if items.len() >= self.capacity {
            return Err("Update queue is full".to_string());
        }

        items.push_back(update);
        drop(items);
        self.notify.notify_one();
        Ok(())
    }

    async fn requeue_front(&self, update: QueuedAnimeListUpdate) -> Result<(), String> {
        let mut items = self.items.lock().await;
        if items.len() >= self.capacity {
            return Err("Update queue is full while restarting worker".to_string());
        }

        items.push_front(update);
        drop(items);
        self.notify.notify_one();
        Ok(())
    }

    async fn pop_front(&self) -> QueuedAnimeListUpdate {
        loop {
            let notified = self.notify.notified();
            if let Some(update) = {
                let mut items = self.items.lock().await;
                items.pop_front()
            } {
                return update;
            }

            notified.await;
        }
    }

    #[cfg(test)]
    async fn len(&self) -> usize {
        self.items.lock().await.len()
    }
}

struct AnimeListUpdateQueueState {
    app: tauri::AppHandle,
    pending_updates: PendingAnimeListUpdates,
    supervisor_running: AtomicBool,
}

impl AnimeListUpdateQueueState {
    fn new(app: tauri::AppHandle) -> Self {
        Self {
            app,
            pending_updates: PendingAnimeListUpdates::new(UPDATE_QUEUE_CAPACITY),
            supervisor_running: AtomicBool::new(false),
        }
    }

    fn ensure_worker_supervisor(self: &Arc<Self>) {
        if self
            .supervisor_running
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            return;
        }

        let state = Arc::clone(self);
        tauri::async_runtime::spawn(async move {
            let _supervisor_guard = WorkerSupervisorGuard::new(Arc::clone(&state));
            run_worker_supervisor(state).await;
        });
    }
}

struct WorkerSupervisorGuard {
    state: Arc<AnimeListUpdateQueueState>,
}

impl WorkerSupervisorGuard {
    fn new(state: Arc<AnimeListUpdateQueueState>) -> Self {
        Self { state }
    }
}

impl Drop for WorkerSupervisorGuard {
    fn drop(&mut self) {
        self.state
            .supervisor_running
            .store(false, Ordering::Release);
    }
}

pub struct AnimeListUpdateQueue {
    state: Arc<AnimeListUpdateQueueState>,
}

impl AnimeListUpdateQueue {
    pub fn new(app: tauri::AppHandle) -> Self {
        let queue = Self {
            state: Arc::new(AnimeListUpdateQueueState::new(app)),
        };
        queue.ensure_worker_supervisor();
        queue
    }

    pub async fn enqueue(&self, update: AnimeListUpdateRequest) -> Result<(), String> {
        self.state.pending_updates.enqueue(update).await?;
        self.ensure_worker_supervisor();
        Ok(())
    }

    fn ensure_worker_supervisor(&self) {
        self.state.ensure_worker_supervisor();
    }
}

#[tauri::command]
pub async fn enqueue_anime_list_update(
    update: AnimeListUpdateRequest,
    app: tauri::AppHandle,
) -> Result<(), String> {
    app.state::<AnimeListUpdateQueue>().enqueue(update).await
}

fn update_log_context(update: &AnimeListUpdateRequest) -> String {
    format!(
        "provider={}, list_type={:?}, entry_id={:?}, media_id={:?}",
        update.provider_id,
        update.list_type.unwrap_or_default(),
        update.entry_id,
        update.media_id
    )
}

async fn run_worker_supervisor(state: Arc<AnimeListUpdateQueueState>) {
    let client = reqwest::Client::new();
    let interval = Duration::from_millis(UPDATE_INTERVAL_MS);
    let restart_delay = Duration::from_millis(WORKER_RESTART_DELAY_MS);

    update_worker_log!(
        "Anime list update worker supervisor started (queue_capacity={}, interval_ms={}, crash_retry_limit={})",
        UPDATE_QUEUE_CAPACITY,
        UPDATE_INTERVAL_MS,
        WORKER_CRASH_RETRY_LIMIT
    );

    loop {
        let queued_update = state.pending_updates.pop_front().await;
        let context = update_log_context(&queued_update.request);
        update_worker_log!("Anime list update received ({context})");

        let app = state.app.clone();
        let client = client.clone();
        let update = queued_update.request.clone();
        let worker =
            tauri::async_runtime::spawn(async move { handle_update(&app, &client, &update).await });

        match worker.await {
            Ok(Ok(())) => {
                update_worker_log!("Anime list update completed ({context})");
            }
            Ok(Err(err)) => {
                update_worker_log!("Anime list update failed ({context}): {err}");
            }
            Err(join_err) => {
                let failure = join_error_message(join_err);
                update_worker_log!("Anime list update worker crashed ({context}): {failure}");

                if let Some(retry_update) = queued_update.schedule_retry() {
                    if let Err(err) = state.pending_updates.requeue_front(retry_update).await {
                        update_worker_log!(
                            "Anime list update requeue failed after worker crash ({context}): {err}"
                        );
                    }
                } else {
                    update_worker_log!(
                        "Anime list update dropped after worker crash retries exhausted ({context})"
                    );
                }

                tokio::time::sleep(restart_delay).await;
                continue;
            }
        }

        tokio::time::sleep(interval).await;
    }
}

fn join_error_message(join_err: tauri::Error) -> String {
    match join_err {
        tauri::Error::JoinError(join_err) if join_err.is_panic() => {
            panic_payload_message(join_err.into_panic())
        }
        other => other.to_string(),
    }
}

fn panic_payload_message(payload: Box<dyn Any + Send>) -> String {
    if let Some(message) = payload.downcast_ref::<&'static str>() {
        return (*message).to_string();
    }

    if let Some(message) = payload.downcast_ref::<String>() {
        return message.clone();
    }

    "unknown panic payload".to_string()
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
    use tokio::runtime::Runtime;

    fn sample_update() -> AnimeListUpdateRequest {
        AnimeListUpdateRequest {
            provider_id: MAL_PROVIDER_ID.to_string(),
            list_type: Some(ListType::Anime),
            entry_id: Some(1),
            media_id: None,
            user_status: Some("completed".to_string()),
            user_score: None,
            user_episodes_watched: None,
            user_volumes_read: None,
            user_chapters_read: None,
            is_rewatching: None,
            is_rereading: None,
            user_comments: None,
            user_num_times_rewatched: None,
            user_num_times_reread: None,
            user_start_date: None,
            user_finish_date: None,
        }
    }

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

    #[test]
    fn pending_updates_accept_items_without_worker() {
        let runtime = Runtime::new().expect("runtime should build");
        let pending = PendingAnimeListUpdates::new(2);

        runtime
            .block_on(async { pending.enqueue(sample_update()).await })
            .expect("enqueue should succeed");

        assert_eq!(runtime.block_on(async { pending.len().await }), 1);
    }

    #[test]
    fn pending_updates_requeue_front_preserves_crashed_item() {
        let runtime = Runtime::new().expect("runtime should build");
        let pending = PendingAnimeListUpdates::new(3);
        let first = sample_update();
        let mut second = sample_update();
        second.entry_id = Some(2);

        runtime
            .block_on(async { pending.enqueue(first.clone()).await })
            .expect("first enqueue should succeed");
        runtime
            .block_on(async { pending.enqueue(second.clone()).await })
            .expect("second enqueue should succeed");

        let popped = runtime.block_on(async { pending.pop_front().await });
        runtime
            .block_on(async { pending.requeue_front(popped.clone()).await })
            .expect("requeue should succeed");

        let first_again = runtime.block_on(async { pending.pop_front().await });
        let second_after = runtime.block_on(async { pending.pop_front().await });

        assert_eq!(first_again.request.entry_id, first.entry_id);
        assert_eq!(second_after.request.entry_id, second.entry_id);
    }

    #[test]
    fn pending_updates_reject_when_queue_is_full() {
        let runtime = Runtime::new().expect("runtime should build");
        let pending = PendingAnimeListUpdates::new(1);

        runtime
            .block_on(async { pending.enqueue(sample_update()).await })
            .expect("first enqueue should succeed");

        let error = runtime.block_on(async { pending.enqueue(sample_update()).await });

        assert_eq!(error.unwrap_err(), "Update queue is full");
    }

    #[test]
    fn queued_update_limits_worker_crash_retries() {
        let queued = QueuedAnimeListUpdate::new(sample_update());

        let queued = queued.schedule_retry().expect("retry 1 should exist");
        let queued = queued.schedule_retry().expect("retry 2 should exist");
        let queued = queued.schedule_retry().expect("retry 3 should exist");

        assert!(queued.schedule_retry().is_none());
    }
}
