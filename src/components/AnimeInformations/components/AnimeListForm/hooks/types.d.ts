import { AnimeListUserStatus } from '@/services/backend/types';

export type AnimeListFormData = {
  userStatus: AnimeListUserStatus;
  userScore: number;
  userEpisodesWatched: number;
  isRewatching: boolean;
  userComments: string;
  userNumTimesRewatched: number;
  userStartDate?: string;
  userFinishDate?: string;
};
