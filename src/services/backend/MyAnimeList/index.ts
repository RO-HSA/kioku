import { invoke } from '@tauri-apps/api/core';

import { SynchronizedAnimeList } from '../types';
import { MyAnimeListUserInfo } from './types';

export class MyAnimeListService {
  static async authorize(): Promise<void> {
    return invoke('authorize_myanimelist');
  }

  static async synchronizeList(): Promise<SynchronizedAnimeList> {
    return invoke<SynchronizedAnimeList>('synchronize_myanimelist');
  }

  static async fetchUserInfo(): Promise<MyAnimeListUserInfo> {
    return invoke<MyAnimeListUserInfo>('fetch_myanimelist_user_info');
  }
}
