import { invoke } from '@tauri-apps/api/core';

import { SynchronizedAnimeList } from '../types';

export class AniListService {
  static async authorize(): Promise<void> {
    return invoke('authorize_anilist');
  }

  static async synchronizeList(): Promise<SynchronizedAnimeList> {
    return invoke<SynchronizedAnimeList>('synchronize_anilist');
  }
}
