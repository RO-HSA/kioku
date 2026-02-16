import { invoke } from '@tauri-apps/api/core';

import { AnimeListUpdateRequest } from '../types';

export class AnimeListService {
  static async enqueueListUpdate(
    update: AnimeListUpdateRequest
  ): Promise<void> {
    return invoke('enqueue_anime_list_update', { update });
  }
}
