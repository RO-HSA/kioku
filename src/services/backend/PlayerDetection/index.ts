import { invoke } from '@tauri-apps/api/core';

import { AnimePlaybackDetection, DetectPlayingAnimeRequest } from '../types';

export class PlayerDetectionService {
  static async detectPlayingAnime(
    request?: DetectPlayingAnimeRequest
  ): Promise<AnimePlaybackDetection | null> {
    return invoke<AnimePlaybackDetection | null>('detect_playing_anime', {
      request
    });
  }
}
