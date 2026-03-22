import { invoke } from '@tauri-apps/api/core';

import { SynchronizedAnimeList } from '../types';
import { MyAnimeListUserInfo } from './types';
import { ListType } from '@/types/List';

export class MyAnimeListService {
  static async authorize(): Promise<void> {
    return invoke('authorize_myanimelist');
  }

  static async synchronizeList(
    listType: ListType = 'anime'
  ): Promise<SynchronizedAnimeList> {
    return invoke<SynchronizedAnimeList>('synchronize_myanimelist', {
      listType
    });
  }

  static async fetchUserInfo(): Promise<MyAnimeListUserInfo> {
    return invoke<MyAnimeListUserInfo>('fetch_myanimelist_user_info');
  }
}
