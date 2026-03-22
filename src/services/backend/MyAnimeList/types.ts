import type { SynchronizedAnimeList } from '../types';
import type { ProviderUserInfo } from '@/types/User';

export type MyAnimeListUserStatus =
  | 'watching'
  | 'completed'
  | 'on_hold'
  | 'dropped'
  | 'plan_to_watch';

export type MyAnimeListMangaUserStatus =
  | 'reading'
  | 'completed'
  | 'on_hold'
  | 'dropped'
  | 'plan_to_read';

export enum MyAnimeListUserStatusEnum {
  'watching' = 'watching',
  'completed' = 'completed',
  'on_hold' = 'onHold',
  'dropped' = 'dropped',
  'plan_to_watch' = 'planToWatch'
}

export enum MyAnimeListMangaUserStatusEnum {
  'reading' = 'reading',
  'completed' = 'completed',
  'on_hold' = 'onHold',
  'dropped' = 'dropped',
  'plan_to_read' = 'planToRead'
}

export type MyAnimeListStatus =
  | 'finished_airing'
  | 'not_yet_aired'
  | 'currently_airing';

export type MyAnimeListMangaStatus =
  | 'finished'
  | 'currently_publishing'
  | 'not_yet_published'
  | 'on_hiatus'
  | 'discontinued';

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

export enum MyAnimeListMangaStatusEnum {
  'finished' = 'Finished',
  'currently_publishing' = 'Currently Publishing',
  'not_yet_published' = 'Not Yet Published',
  'on_hiatus' = 'On Hiatus',
  'discontinued' = 'Discontinued'
}

export type MyAnimeListMediaType =
  | 'tv'
  | 'tv_special'
  | 'movie'
  | 'special'
  | 'ona'
  | 'ova'
  | 'unknown';

export type MyAnimeListMangaMediaType =
  | 'manga'
  | 'novel'
  | 'light_novel'
  | 'one_shot'
  | 'doujinshi'
  | 'manhwa'
  | 'manhua'
  | 'oel'
  | 'unknown';

export enum MyAnimeListMediaTypeEnum {
  'tv' = 'TV',
  'tv_special' = 'Special',
  'movie' = 'Movie',
  'special' = 'Special',
  'ona' = 'ONA',
  'ova' = 'OVA',
  'unknown' = 'Unknown'
}

export enum MyAnimeListMangaMediaTypeEnum {
  'manga' = 'Manga',
  'novel' = 'Novel',
  'light_novel' = 'Light Novel',
  'one_shot' = 'One-shot',
  'doujinshi' = 'Doujinshi',
  'manhwa' = 'Manhwa',
  'manhua' = 'Manhua',
  'oel' = 'OEL',
  'unknown' = 'Unknown'
}

export type MyAnimeListUserInfo = ProviderUserInfo;

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
    media_type?: MyAnimeListMediaType;
    studios?: { id: number; name: string }[];
  };
  list_status: {
    status: MyAnimeListUserStatus;
    score?: number;
    num_episodes_watched?: number;
    is_rewatching?: boolean;
    comments?: string;
    num_times_rewatched?: number;
    updated_at?: string;
    start_date?: string;
    finish_date?: string;
  };
}

export interface MyAnimeListMangaListEntry {
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
    status?: MyAnimeListMangaStatus;
    genres: { id: number; name: string }[];
    media_type?: MyAnimeListMangaMediaType;
    num_volumes?: number;
    num_chapters?: number;
    authors?: {
      node?: {
        first_name?: string;
        last_name?: string;
      };
    }[];
    serialization?: {
      name?: string;
    };
  };
  list_status: {
    status: MyAnimeListMangaUserStatus;
    score?: number;
    num_volumes_read?: number;
    num_chapters_read?: number;
    is_rereading?: boolean;
    comments?: string;
    num_times_reread?: number;
    updated_at?: string;
    start_date?: string;
    finish_date?: string;
  };
}

export type SynchronizeMyAnimeListResult = SynchronizedAnimeList;
