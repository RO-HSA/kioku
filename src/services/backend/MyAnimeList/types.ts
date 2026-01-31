export type MyAnimeListUserStatus =
  | 'watching'
  | 'completed'
  | 'on_hold'
  | 'dropped'
  | 'plan_to_watch';

export enum MyAnimeListUserStatusEnum {
  'watching' = 'watching',
  'completed' = 'completed',
  'on_hold' = 'onHold',
  'dropped' = 'dropped',
  'plan_to_watch' = 'planToWatch'
}

export type MyAnimeListStatus =
  | 'finished_airing'
  | 'not_yet_aired'
  | 'currently_airing';

export type MyAnimeListSource =
  | 'anime'
  | 'manga'
  | 'light_novel'
  | 'visual_novel'
  | 'original';

export enum MyAnimeListSourceEnum {
  'anime' = 'Anime',
  'manga' = 'Manga',
  'light_novel' = 'Light Novel',
  'visual_novel' = 'Visual Novel',
  'original' = 'Original'
}

export enum MyAnimeListStatusEnum {
  'finished_airing' = 'Finished Airing',
  'not_yet_aired' = 'Not Yet Aired',
  'currently_airing' = 'Currently Airing'
}

export interface MyAnimeListListEntry {
  node: {
    id: number;
    title: string;
    main_picture?: { medium: string; large: string };
    synopsis?: string;
    alternative_titles?: {
      synonyms?: string[];
      en?: string;
      ja?: string;
    };
    mean?: number;
    source?: MyAnimeListSource;
    num_episodes?: number;
    status: MyAnimeListStatus;
    genres: { id: number; name: string }[];
    nsfw?: string;
    start_season?: { year: number; season: string };
    media_type?: string;
    studios?: { id: number; name: string }[];
  };
  list_status: {
    status: MyAnimeListUserStatus;
    score?: number;
    num_episodes_watched?: number;
    is_rewatching?: boolean;
    updated_at?: string;
    start_date?: string;
    finish_date?: string;
  };
}

export type SynchronizeMyAnimeListResult = Record<
  MyAnimeListUserStatus,
  MyAnimeListListEntry[]
>;
