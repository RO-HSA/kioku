import { invoke } from '@tauri-apps/api/core';
import { ListType } from '@/types/List';
import { SynchronizedAnimeList, SynchronizedMangaList } from '../types';
import { AniListUserInfo } from './types';

export class AniListService {
  static async authorize(): Promise<void> {
    return invoke('authorize_anilist');
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
      'synchronize_anilist',
      { listType }
    );
  }

  static async fetchUserInfo(): Promise<AniListUserInfo> {
    return invoke<AniListUserInfo>('fetch_anilist_user_info');
  }
}
