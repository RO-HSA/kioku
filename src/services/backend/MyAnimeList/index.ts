import { invoke } from '@tauri-apps/api/core';

import { AnimeListUpdateRequest, SynchronizedAnimeList } from '../types';

export class MyAnimeListService {
  static async authorize(): Promise<void> {
    return invoke('authorize_myanimelist');
  }

  static async synchronizeList(): Promise<SynchronizedAnimeList> {
    return invoke<SynchronizedAnimeList>('synchronize_myanimelist');
  }

  static async enqueueListUpdate(
    update: AnimeListUpdateRequest
  ): Promise<void> {
    return invoke('enqueue_anime_list_update', { update });
  }
}
