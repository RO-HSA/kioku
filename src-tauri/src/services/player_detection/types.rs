use serde::{Deserialize, Serialize};

use super::util::normalize_process_name;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SupportedPlayer {
    Mpv,
    MpcHc,
    MpcBe,
}

impl SupportedPlayer {
    pub(crate) fn all() -> Vec<Self> {
        vec![Self::Mpv, Self::MpcHc, Self::MpcBe]
    }

    fn process_aliases(self) -> &'static [&'static str] {
        match self {
            Self::Mpv => &["mpv", "mpv.exe", "mpvnet", "mpvnet.exe", "io.mpv.mpv"],
            Self::MpcHc => &["mpc-hc", "mpc-hc.exe", "mpc-hc64", "mpc-hc64.exe"],
            Self::MpcBe => &["mpc-be", "mpc-be.exe", "mpc-be64", "mpc-be64.exe"],
        }
    }

    pub(crate) fn matches_process_name(self, value: &str) -> bool {
        let normalized = normalize_process_name(value);
        self.process_aliases()
            .iter()
            .any(|alias| normalized == *alias)
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DetectPlayingAnimeRequest {
    pub players: Option<Vec<SupportedPlayer>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AnimePlaybackDetection {
    pub player: SupportedPlayer,
    pub process_id: u32,
    pub source: String,
    pub anime_title: String,
    pub episode: Option<u32>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaybackObserverSnapshot {
    pub active: Option<AnimePlaybackDetection>,
    pub last_observed: Option<AnimePlaybackDetection>,
    pub observed_process_id: Option<u32>,
    pub observed_player: Option<SupportedPlayer>,
    pub selected_players: Vec<SupportedPlayer>,
    pub enabled: bool,
    pub poll_interval_ms: u64,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ConfigurePlaybackObserverRequest {
    pub enabled: Option<bool>,
    pub players: Option<Vec<SupportedPlayer>>,
    pub poll_interval_ms: Option<u64>,
}
