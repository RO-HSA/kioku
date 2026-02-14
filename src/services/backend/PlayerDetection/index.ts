import { invoke } from '@tauri-apps/api/core';

import {
  AnimePlaybackDetection,
  ConfigurePlaybackObserverRequest,
  DetectPlayingAnimeRequest,
  PlaybackObserverSnapshot
} from '../types';

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
}
