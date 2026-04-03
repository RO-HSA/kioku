import { invoke } from '@tauri-apps/api/core';

import { ListType } from '@/types/List';
import { SynchronizedAnimeList, SynchronizedMangaList } from '../types';
import { MyAnimeListUserInfo } from './types';

export class MyAnimeListService {
  static async authorize(): Promise<void> {
    return invoke('authorize_myanimelist');
  }

  static async synchronizeList(
    listType?: 'anime'
  ): Promise<SynchronizedAnimeList>;
  static async synchronizeList(
    listType: 'manga'
  ): Promise<SynchronizedMangaList>;
  static async synchronizeList(
    listType: ListType = 'anime'
  ): Promise<SynchronizedAnimeList | SynchronizedMangaList> {
    return invoke<SynchronizedAnimeList | SynchronizedMangaList>(
      'synchronize_myanimelist',
      {
        listType
      }
    );
  }

  static async fetchUserInfo(): Promise<MyAnimeListUserInfo> {
    return invoke<MyAnimeListUserInfo>('fetch_myanimelist_user_info');
  }
}
