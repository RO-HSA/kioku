import { invoke } from '@tauri-apps/api/core';
import { IAnimeList } from '@/types/AnimeList';
import { ListType } from '@/types/List';
import { IMangaList } from '@/types/MangaList';
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

  static async searchMedia(
    query: string,
    listType?: 'anime',
    limit?: number
  ): Promise<IAnimeList[]>;
  static async searchMedia(
    query: string,
    listType: 'manga',
    limit?: number
  ): Promise<IMangaList[]>;
  static async searchMedia(
    query: string,
    listType: ListType = 'anime',
    limit?: number
  ): Promise<IAnimeList[] | IMangaList[]> {
    return invoke<IAnimeList[] | IMangaList[]>('search_anilist_media', {
      query,
      listType,
      limit
    });
  }
}
