use std::time::Duration;

use tauri::{Emitter, Manager};
use tokio::sync::{Mutex, RwLock};

use super::detector::{collect_detection_cycle_result, DetectionCycleResult};
use super::types::{
    AnimePlaybackDetection, ConfigurePlaybackObserverRequest, PlaybackObserverSnapshot,
    SupportedPlayer,
};
use super::util::{dedup_players, normalize_poll_interval_ms, DEFAULT_OBSERVER_POLL_INTERVAL_MS};

pub const PLAYBACK_EPISODE_DETECTED_EVENT: &str = "player-detection:episode-detected";
pub const PLAYBACK_EPISODE_CLOSED_EVENT: &str = "player-detection:episode-closed";

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

fn apply_configuration_to_guard(
    guard: &mut PlaybackObserverStateData,
    request: ConfigurePlaybackObserverRequest,
) -> (
    PlaybackObserverSnapshot,
    Option<AnimePlaybackDetection>,
    Option<AnimePlaybackDetection>,
) {
    let previous_active = guard.active.clone();

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

    (
        PlaybackObserverState::snapshot_from_guard(guard),
        previous_active,
        guard.active.clone(),
    )
}

fn apply_cycle_success_to_guard(
    guard: &mut PlaybackObserverStateData,
    runtime_config: &ObserverRuntimeConfig,
    cycle_result: &DetectionCycleResult,
) -> Option<(
    Option<AnimePlaybackDetection>,
    Option<AnimePlaybackDetection>,
)> {
    let previous_active = guard.active.clone();

    if !runtime_config.enabled || !guard.enabled {
        return None;
    }

    guard.last_error = None;

    if guard.selected_players != runtime_config.selected_players {
        return None;
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
                .find(|candidate| candidate.process_id == observed_process_id)
            {
                guard.active = Some(updated_detection.detection.clone());
                guard.observed_process_id = Some(updated_detection.process_id);
                guard.observed_player = Some(updated_detection.detection.player);
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
            guard.active = Some(first_detection.detection.clone());
            guard.observed_process_id = Some(first_detection.process_id);
            guard.observed_player = Some(first_detection.detection.player);
        } else {
            guard.active = None;
            guard.observed_player = None;
        }
    }

    Some((previous_active, guard.active.clone()))
}

fn apply_cycle_error_to_guard(guard: &mut PlaybackObserverStateData, error: String) {
    if !guard.enabled {
        return;
    }

    guard.last_error = Some(error);
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
        let (snapshot, previous_active, current_active) = {
            let mut guard = self.data.write().await;
            apply_configuration_to_guard(&mut guard, request)
        };

        emit_playback_observer_events(&app, previous_active, current_active);

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
        app: &tauri::AppHandle,
        runtime_config: ObserverRuntimeConfig,
        cycle_result: DetectionCycleResult,
    ) {
        let transitions = {
            let mut guard = self.data.write().await;
            apply_cycle_success_to_guard(&mut guard, &runtime_config, &cycle_result)
        };

        if let Some((previous_active, current_active)) = transitions {
            emit_playback_observer_events(app, previous_active, current_active);
        }
    }

    async fn apply_cycle_error(&self, error: String) {
        let mut guard = self.data.write().await;
        apply_cycle_error_to_guard(&mut guard, error);
    }
}

fn emit_playback_observer_events(
    app: &tauri::AppHandle,
    previous_active: Option<AnimePlaybackDetection>,
    current_active: Option<AnimePlaybackDetection>,
) {
    if previous_active == current_active {
        return;
    }

    if let Some(detection) = current_active.as_ref() {
        if let Err(error) = app.emit(PLAYBACK_EPISODE_DETECTED_EVENT, detection) {
            eprintln!("failed to emit playback episode detected event: {error}");
        }
    }

    if current_active.is_none() {
        if let Some(previous) = previous_active {
            if let Err(error) = app.emit(PLAYBACK_EPISODE_CLOSED_EVENT, previous) {
                eprintln!("failed to emit playback episode closed event: {error}");
            }
        }
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
                    .apply_cycle_success(&app, runtime_config, result)
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

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::super::detector::DetectionCandidate;
    use super::*;

    fn detection(process_id: u32, player: SupportedPlayer, title: &str) -> DetectionCandidate {
        DetectionCandidate {
            process_id,
            detection: AnimePlaybackDetection {
                player,
                anime_title: title.to_string(),
                episode: Some(1),
            },
        }
    }

    fn runtime_config(
        enabled: bool,
        selected_players: Vec<SupportedPlayer>,
        observed_process_id: Option<u32>,
    ) -> ObserverRuntimeConfig {
        ObserverRuntimeConfig {
            enabled,
            selected_players,
            poll_interval_ms: DEFAULT_OBSERVER_POLL_INTERVAL_MS,
            observed_process_id,
        }
    }

    #[test]
    fn configure_guard_deduplicates_players_and_clears_state_when_disabled() {
        let mut guard = PlaybackObserverStateData {
            active: Some(AnimePlaybackDetection {
                player: SupportedPlayer::Mpv,
                anime_title: "Frieren".to_string(),
                episode: Some(12),
            }),
            last_observed: None,
            observed_process_id: Some(99),
            observed_player: Some(SupportedPlayer::Mpv),
            selected_players: SupportedPlayer::all(),
            enabled: true,
            poll_interval_ms: DEFAULT_OBSERVER_POLL_INTERVAL_MS,
            last_error: Some("old error".to_string()),
        };

        let (snapshot, previous_active, current_active) = apply_configuration_to_guard(
            &mut guard,
            ConfigurePlaybackObserverRequest {
                enabled: Some(false),
                players: Some(vec![
                    SupportedPlayer::Mpv,
                    SupportedPlayer::Mpv,
                    SupportedPlayer::MpcBe,
                ]),
                poll_interval_ms: Some(10),
            },
        );

        assert!(previous_active.is_some());
        assert!(current_active.is_none());
        assert!(!snapshot.enabled);
        assert_eq!(
            snapshot.selected_players,
            vec![SupportedPlayer::Mpv, SupportedPlayer::MpcBe]
        );
        assert_eq!(snapshot.poll_interval_ms, 500);
        assert_eq!(
            snapshot
                .last_observed
                .as_ref()
                .map(|item| item.anime_title.as_str()),
            Some("Frieren")
        );
        assert!(snapshot.last_error.is_none());
        assert!(snapshot.active.is_none());
    }

    #[test]
    fn apply_cycle_success_selects_first_detection_when_none_is_observed() {
        let mut guard = PlaybackObserverStateData {
            enabled: true,
            selected_players: vec![SupportedPlayer::Mpv],
            ..Default::default()
        };
        let cycle_result = DetectionCycleResult {
            detections: vec![detection(7, SupportedPlayer::Mpv, "Frieren")],
            matched_player_pids: HashSet::from([7]),
        };

        let transitions = apply_cycle_success_to_guard(
            &mut guard,
            &runtime_config(true, vec![SupportedPlayer::Mpv], None),
            &cycle_result,
        )
        .expect("transition should occur");

        assert!(transitions.0.is_none());
        assert_eq!(
            transitions.1.as_ref().map(|item| item.anime_title.as_str()),
            Some("Frieren")
        );
        assert_eq!(guard.observed_process_id, Some(7));
        assert_eq!(guard.observed_player, Some(SupportedPlayer::Mpv));
    }

    #[test]
    fn apply_cycle_success_updates_existing_observed_process_and_clears_errors() {
        let mut guard = PlaybackObserverStateData {
            active: Some(AnimePlaybackDetection {
                player: SupportedPlayer::Mpv,
                anime_title: "Old".to_string(),
                episode: Some(1),
            }),
            observed_process_id: Some(10),
            observed_player: Some(SupportedPlayer::Mpv),
            selected_players: vec![SupportedPlayer::Mpv],
            enabled: true,
            poll_interval_ms: DEFAULT_OBSERVER_POLL_INTERVAL_MS,
            last_error: Some("boom".to_string()),
            last_observed: None,
        };
        let cycle_result = DetectionCycleResult {
            detections: vec![detection(10, SupportedPlayer::Mpv, "New")],
            matched_player_pids: HashSet::from([10]),
        };

        let transitions = apply_cycle_success_to_guard(
            &mut guard,
            &runtime_config(true, vec![SupportedPlayer::Mpv], Some(10)),
            &cycle_result,
        )
        .expect("transition should occur");

        assert_eq!(
            transitions.0.as_ref().map(|item| item.anime_title.as_str()),
            Some("Old")
        );
        assert_eq!(
            transitions.1.as_ref().map(|item| item.anime_title.as_str()),
            Some("New")
        );
        assert!(guard.last_error.is_none());
        assert_eq!(guard.observed_process_id, Some(10));
    }

    #[test]
    fn apply_cycle_success_closes_missing_process_and_preserves_last_observed() {
        let mut guard = PlaybackObserverStateData {
            active: Some(AnimePlaybackDetection {
                player: SupportedPlayer::Mpv,
                anime_title: "Frieren".to_string(),
                episode: Some(12),
            }),
            observed_process_id: Some(10),
            observed_player: Some(SupportedPlayer::Mpv),
            selected_players: vec![SupportedPlayer::Mpv],
            enabled: true,
            poll_interval_ms: DEFAULT_OBSERVER_POLL_INTERVAL_MS,
            last_error: None,
            last_observed: None,
        };
        let cycle_result = DetectionCycleResult {
            detections: Vec::new(),
            matched_player_pids: HashSet::new(),
        };

        let transitions = apply_cycle_success_to_guard(
            &mut guard,
            &runtime_config(true, vec![SupportedPlayer::Mpv], Some(10)),
            &cycle_result,
        )
        .expect("transition should occur");

        assert_eq!(
            transitions.0.as_ref().map(|item| item.anime_title.as_str()),
            Some("Frieren")
        );
        assert!(transitions.1.is_none());
        assert!(guard.active.is_none());
        assert_eq!(guard.observed_process_id, None);
        assert_eq!(
            guard
                .last_observed
                .as_ref()
                .map(|item| item.anime_title.as_str()),
            Some("Frieren")
        );
    }

    #[test]
    fn apply_cycle_success_ignores_stale_runtime_config_and_disabled_state() {
        let mut stale_guard = PlaybackObserverStateData {
            enabled: true,
            selected_players: vec![SupportedPlayer::Mpv],
            last_error: Some("old".to_string()),
            ..Default::default()
        };
        let cycle_result = DetectionCycleResult {
            detections: vec![detection(5, SupportedPlayer::Mpv, "Frieren")],
            matched_player_pids: HashSet::from([5]),
        };
        assert!(apply_cycle_success_to_guard(
            &mut stale_guard,
            &runtime_config(true, vec![SupportedPlayer::MpcBe], None),
            &cycle_result,
        )
        .is_none());
        assert!(stale_guard.last_error.is_none());
        assert!(stale_guard.active.is_none());

        let mut disabled_guard = PlaybackObserverStateData {
            enabled: false,
            ..Default::default()
        };
        assert!(apply_cycle_success_to_guard(
            &mut disabled_guard,
            &runtime_config(true, vec![SupportedPlayer::Mpv], None),
            &cycle_result,
        )
        .is_none());
    }

    #[test]
    fn apply_cycle_error_only_updates_enabled_observers() {
        let mut enabled_guard = PlaybackObserverStateData {
            enabled: true,
            ..Default::default()
        };
        apply_cycle_error_to_guard(&mut enabled_guard, "boom".to_string());
        assert_eq!(enabled_guard.last_error.as_deref(), Some("boom"));

        let mut disabled_guard = PlaybackObserverStateData {
            enabled: false,
            ..Default::default()
        };
        apply_cycle_error_to_guard(&mut disabled_guard, "boom".to_string());
        assert!(disabled_guard.last_error.is_none());
    }
}
