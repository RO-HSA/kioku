export type MyAnimeListStatus =
  | 'watching'
  | 'completed'
  | 'on_hold'
  | 'dropped'
  | 'plan_to_watch';

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
    rating?: string;
    source?: string;
    num_episodes?: number;
    nsfw?: string;
    start_season?: { year: number; season: string };
    media_type?: string;
    studios?: { id: number; name: string }[];
    [key: string]: unknown;
  };
  list_status: {
    status: MyAnimeListStatus;
    score?: number;
    num_episodes_watched?: number;
    is_rewatching?: boolean;
    updated_at?: string;
    start_date?: string;
    finish_date?: string;
    [key: string]: unknown;
  };
  [key: string]: unknown;
}

export type SynchronizeMyAnimeListResult = Record<
  MyAnimeListStatus,
  MyAnimeListListEntry[]
>;
