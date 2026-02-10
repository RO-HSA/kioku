import { AnimeListUserStatus, IAnimeList } from '@/types/AnimeList';

export type SynchronizedAnimeList = Record<AnimeListUserStatus, IAnimeList[]>;

export interface AnimeListUpdateRequest {
  providerId: string;
  entryId: number;
  userStatus?: AnimeListUserStatus;
  userScore?: number;
  userEpisodesWatched?: number;
  isRewatching?: boolean;
  userStartDate?: string;
  userFinishDate?: string;
}
