export type AnimeListUserStatus =
  | 'watching'
  | 'completed'
  | 'onHold'
  | 'dropped'
  | 'planToWatch';

export interface IAnimeList {
  id: number;
  title: string;
  imageUrl: string;
  synopsis: string;
  alternativeTitles: string;
  score: number;
  source: string;
  status: string;
  totalEpisodes: number;
  genres: string;
  startSeason: string;
  studios: string;
  mediaType: string;
  userStatus: {
    status: AnimeListUserStatus;
    score: number;
    episodesWatched: number;
    isRewatching: boolean;
    startDate?: string;
    finishDate?: string;
    updatedAt?: string;
  };
}

export type SynchronizedAnimeList = Record<AnimeListUserStatus, IAnimeList[]>;
