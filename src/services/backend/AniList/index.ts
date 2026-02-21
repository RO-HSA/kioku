import { invoke } from '@tauri-apps/api/core';
import { SynchronizedAnimeList } from '../types';
import { AniListUserInfo } from './types';

export class AniListService {
  static async authorize(): Promise<void> {
    return invoke('authorize_anilist');
  }

  static async synchronizeList(): Promise<SynchronizedAnimeList> {
    return invoke<SynchronizedAnimeList>('synchronize_anilist');
  }

  static async fetchUserInfo(): Promise<AniListUserInfo> {
    return invoke<AniListUserInfo>('fetch_anilist_user_info');
  }
}
