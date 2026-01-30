import { invoke } from '@tauri-apps/api/core';

import { SynchronizeMyAnimeListResult } from './type';

export class MyAnimeListService {
  static async authorize(): Promise<SynchronizeMyAnimeListResult | undefined> {
    try {
      return invoke('authorize_myanimelist');
    } catch (error) {
      console.log({ error });
    }
  }

  static async synchronizeList(): Promise<void> {
    return invoke('synchronize_myanimelist');
  }
}
