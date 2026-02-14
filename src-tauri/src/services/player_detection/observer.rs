use std::time::Duration;

use tauri::Manager;
use tokio::sync::{Mutex, RwLock};

use super::detector::{collect_detection_cycle_result, DetectionCycleResult};
use super::types::{
    AnimePlaybackDetection, ConfigurePlaybackObserverRequest, PlaybackObserverSnapshot,
    SupportedPlayer,
};
use super::util::{dedup_players, normalize_poll_interval_ms, DEFAULT_OBSERVER_POLL_INTERVAL_MS};

struct PlaybackObserverStateData {
    active: Option<AnimePlaybackDetection>,
    last_observed: Option<AnimePlaybackDetection>,
    observed_process_id: Option<u32>,
    observed_player: Option<SupportedPlayer>,
    selected_players: Vec<SupportedPlayer>,
    enabled: bool,
    poll_interval_ms: u64,
    last_error: Option<String>,
}

impl Default for PlaybackObserverStateData {
    fn default() -> Self {
        Self {
            active: None,
            last_observed: None,
            observed_process_id: None,
            observed_player: None,
            selected_players: SupportedPlayer::all(),
            enabled: false,
            poll_interval_ms: DEFAULT_OBSERVER_POLL_INTERVAL_MS,
            last_error: None,
        }
    }
}

#[derive(Debug, Clone)]
struct ObserverRuntimeConfig {
    enabled: bool,
    selected_players: Vec<SupportedPlayer>,
    poll_interval_ms: u64,
    observed_process_id: Option<u32>,
}

pub struct PlaybackObserverState {
    data: RwLock<PlaybackObserverStateData>,
    worker: Mutex<Option<tauri::async_runtime::JoinHandle<()>>>,
}

impl PlaybackObserverState {
    pub fn new(enabled: bool, selected_players: Vec<SupportedPlayer>) -> Self {
        let mut state = PlaybackObserverStateData::default();
        state.enabled = enabled;
        state.selected_players = dedup_players(selected_players);

        Self {
            data: RwLock::new(state),
            worker: Mutex::new(None),
        }
    }

    fn snapshot_from_guard(guard: &PlaybackObserverStateData) -> PlaybackObserverSnapshot {
        PlaybackObserverSnapshot {
            active: guard.active.clone(),
            last_observed: guard.last_observed.clone(),
            observed_process_id: guard.observed_process_id,
            observed_player: guard.observed_player,
            selected_players: guard.selected_players.clone(),
            enabled: guard.enabled,
            poll_interval_ms: guard.poll_interval_ms,
            last_error: guard.last_error.clone(),
        }
    }

    pub async fn snapshot(&self) -> PlaybackObserverSnapshot {
        let guard = self.data.read().await;

        Self::snapshot_from_guard(&guard)
    }

    async fn read_runtime_config(&self) -> ObserverRuntimeConfig {
        let guard = self.data.read().await;

        ObserverRuntimeConfig {
            enabled: guard.enabled,
            selected_players: guard.selected_players.clone(),
            poll_interval_ms: guard.poll_interval_ms,
            observed_process_id: guard.observed_process_id,
        }
    }

    pub async fn configure(
        &self,
        app: tauri::AppHandle,
        request: ConfigurePlaybackObserverRequest,
    ) -> PlaybackObserverSnapshot {
        let snapshot = {
            let mut guard = self.data.write().await;

            if let Some(enabled) = request.enabled {
                guard.enabled = enabled;
            }

            if let Some(players) = request.players {
                guard.selected_players = dedup_players(players);
            }

            if let Some(poll_interval_ms) = request.poll_interval_ms {
                guard.poll_interval_ms = normalize_poll_interval_ms(poll_interval_ms);
            }

            if !guard.enabled {
                if guard.active.is_some() {
                    guard.last_observed = guard.active.clone();
                }

                guard.active = None;
                guard.observed_process_id = None;
                guard.observed_player = None;
                guard.last_error = None;
            }

            Self::snapshot_from_guard(&guard)
        };

        if snapshot.enabled {
            self.start_worker(app).await;
        } else {
            self.stop_worker().await;
        }

        snapshot
    }

    pub async fn start_if_enabled(&self, app: tauri::AppHandle) {
        let enabled = self.data.read().await.enabled;
        if enabled {
            self.start_worker(app).await;
        }
    }

    async fn start_worker(&self, app: tauri::AppHandle) {
        let mut worker = self.worker.lock().await;
        if worker.is_some() {
            return;
        }

        *worker = Some(tauri::async_runtime::spawn(run_playback_observer_loop(app)));
    }

    async fn stop_worker(&self) {
        let mut worker = self.worker.lock().await;
        if let Some(handle) = worker.take() {
            handle.abort();
        }
    }

    async fn apply_cycle_success(
        &self,
        runtime_config: ObserverRuntimeConfig,
        cycle_result: DetectionCycleResult,
    ) {
        let mut guard = self.data.write().await;

        if !runtime_config.enabled || !guard.enabled {
            return;
        }

        guard.last_error = None;

        if guard.selected_players != runtime_config.selected_players {
            return;
        }

        let observed_process_is_running = runtime_config
            .observed_process_id
            .map(|process_id| cycle_result.matched_player_pids.contains(&process_id))
            .unwrap_or(false);

        if let Some(observed_process_id) = runtime_config.observed_process_id {
            if observed_process_is_running {
                if let Some(updated_detection) = cycle_result
                    .detections
                    .iter()
                    .find(|detection| detection.process_id == observed_process_id)
                {
                    guard.active = Some(updated_detection.clone());
                    guard.observed_process_id = Some(updated_detection.process_id);
                    guard.observed_player = Some(updated_detection.player);
                }
            } else {
                if guard.active.is_some() {
                    guard.last_observed = guard.active.clone();
                }

                guard.active = None;
                guard.observed_process_id = None;
                guard.observed_player = None;
            }
        }

        if guard.observed_process_id.is_none() {
            if let Some(first_detection) = cycle_result.detections.first() {
                guard.active = Some(first_detection.clone());
                guard.observed_process_id = Some(first_detection.process_id);
                guard.observed_player = Some(first_detection.player);
            } else {
                guard.active = None;
                guard.observed_player = None;
            }
        }
    }

    async fn apply_cycle_error(&self, error: String) {
        let mut guard = self.data.write().await;
        if !guard.enabled {
            return;
        }

        guard.last_error = Some(error);
    }
}

async fn run_playback_observer_loop(app: tauri::AppHandle) {
    loop {
        let runtime_config = app
            .state::<PlaybackObserverState>()
            .read_runtime_config()
            .await;
        let poll_interval_ms = runtime_config.poll_interval_ms;

        let cycle_result = collect_detection_cycle_result(&runtime_config.selected_players);
        match cycle_result {
            Ok(result) => {
                app.state::<PlaybackObserverState>()
                    .apply_cycle_success(runtime_config, result)
                    .await;
            }
            Err(error) => {
                app.state::<PlaybackObserverState>()
                    .apply_cycle_error(error)
                    .await;
            }
        }

        tokio::time::sleep(Duration::from_millis(poll_interval_ms)).await;
    }
}

pub fn start_playback_observer(app: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {
        app.state::<PlaybackObserverState>()
            .start_if_enabled(app.clone())
            .await;
    });
}

#[tauri::command]
pub async fn get_playback_observer_state(
    app: tauri::AppHandle,
) -> Result<PlaybackObserverSnapshot, String> {
    Ok(app.state::<PlaybackObserverState>().snapshot().await)
}

#[tauri::command]
pub async fn configure_playback_observer(
    request: ConfigurePlaybackObserverRequest,
    app: tauri::AppHandle,
) -> Result<PlaybackObserverSnapshot, String> {
    Ok(app
        .state::<PlaybackObserverState>()
        .configure(app.clone(), request)
        .await)
}
