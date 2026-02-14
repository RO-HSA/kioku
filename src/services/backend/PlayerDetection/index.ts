import { invoke } from '@tauri-apps/api/core';
import { listen, UnlistenFn } from '@tauri-apps/api/event';

import {
  AnimePlaybackDetection,
  ConfigurePlaybackObserverRequest,
  DetectPlayingAnimeRequest,
  PlaybackObserverSnapshot
} from '../types';

export const PLAYBACK_EPISODE_DETECTED_EVENT =
  'player-detection:episode-detected';
export const PLAYBACK_EPISODE_CLOSED_EVENT = 'player-detection:episode-closed';

export class PlayerDetectionService {
  static async detectPlayingAnime(
    request?: DetectPlayingAnimeRequest
  ): Promise<AnimePlaybackDetection | null> {
    return invoke<AnimePlaybackDetection | null>('detect_playing_anime', {
      request
    });
  }

  static async getPlaybackObserverState(): Promise<PlaybackObserverSnapshot> {
    return invoke<PlaybackObserverSnapshot>('get_playback_observer_state');
  }

  static async configurePlaybackObserver(
    request: ConfigurePlaybackObserverRequest
  ): Promise<PlaybackObserverSnapshot> {
    return invoke<PlaybackObserverSnapshot>('configure_playback_observer', {
      request
    });
  }

  static async listenEpisodeDetected(
    onDetected: (detection: AnimePlaybackDetection) => void
  ): Promise<UnlistenFn> {
    return listen<AnimePlaybackDetection>(
      PLAYBACK_EPISODE_DETECTED_EVENT,
      ({ payload }) => {
        onDetected(payload);
      }
    );
  }

  static async listenEpisodeClosed(
    onClosed: (detection: AnimePlaybackDetection) => void
  ): Promise<UnlistenFn> {
    return listen<AnimePlaybackDetection>(
      PLAYBACK_EPISODE_CLOSED_EVENT,
      ({ payload }) => {
        onClosed(payload);
      }
    );
  }
}
