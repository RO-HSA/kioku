import { invoke } from '@tauri-apps/api/core';

import { ListUpdateRequest } from '../types';

export class AnimeListService {
  static async enqueueListUpdate(update: ListUpdateRequest): Promise<void> {
    return invoke('enqueue_anime_list_update', { update });
  }
}
