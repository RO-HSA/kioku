import { AnimeListUserStatus } from '@/services/backend/types';

export type AnimeListFormData = {
  userStatus: AnimeListUserStatus;
  userScore: number;
  userEpisodesWatched: number;
  isRewatching: boolean;
  userStartDate?: string;
  userFinishDate?: string;
};
