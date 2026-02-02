import { invoke } from '@tauri-apps/api/core';

import { SynchronizedAnimeList } from '../types';

export class MyAnimeListService {
  static async authorize(): Promise<void> {
    return invoke('authorize_myanimelist');
  }

  static async synchronizeList(): Promise<SynchronizedAnimeList> {
    return invoke<SynchronizedAnimeList>('synchronize_myanimelist');
  }
}
