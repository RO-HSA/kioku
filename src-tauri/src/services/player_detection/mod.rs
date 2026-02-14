mod detector;
mod observer;
mod parser;
mod processes;
mod types;
mod util;

pub use detector::detect_playing_anime;
pub use observer::{
    configure_playback_observer, get_playback_observer_state, start_playback_observer,
    PlaybackObserverState,
};
pub use types::{
    AnimePlaybackDetection, ConfigurePlaybackObserverRequest, DetectPlayingAnimeRequest,
    PlaybackObserverSnapshot, SupportedPlayer,
};
