import { invoke } from '@tauri-apps/api/core';

import { SynchronizedAnimeList } from '../types';
import MyAnimeListMapper from './helpers/MyAnimeListMapper';
import { SynchronizeMyAnimeListResult } from './types';

export class MyAnimeListService {
  static async authorize(): Promise<void> {
    return invoke('authorize_myanimelist');
  }

  static async synchronizeList(): Promise<SynchronizedAnimeList> {
    const list = await invoke<SynchronizeMyAnimeListResult>(
      'synchronize_myanimelist'
    );

    const watching = list.watching.map(MyAnimeListMapper.mapListEntryToDomain);

    const completed = list.completed.map(
      MyAnimeListMapper.mapListEntryToDomain
    );

    const onHold = list.on_hold.map(MyAnimeListMapper.mapListEntryToDomain);

    const dropped = list.dropped.map(MyAnimeListMapper.mapListEntryToDomain);

    const planToWatch = list.plan_to_watch.map(
      MyAnimeListMapper.mapListEntryToDomain
    );

    return {
      watching,
      completed,
      onHold,
      dropped,
      planToWatch
    };
  }
}
