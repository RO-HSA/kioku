export type AnimeListStatus =
  | 'Finished Airing'
  | 'Not Yet Aired'
  | 'Currently Airing';

export type AnimeListUserStatus =
  | 'watching'
  | 'completed'
  | 'onHold'
  | 'dropped'
  | 'planToWatch';

export type AnimeListBroadcast = {
  dayOfTheWeek: string | null;
  startTime: string | null;
};

export interface IAnime {
  id: number;
  title: string;
  imageUrl: string;
  synopsis: string;
  alternativeTitles: string;
  score: number;
  source: string;
  status: AnimeListStatus;
  totalEpisodes: number;
  genres: string;
  startSeason: string;
  startDate: string | null;
  broadcast: AnimeListBroadcast;
  studios: string;
  mediaType: string;
}

export interface IAnimeUserList {
  userStatus: AnimeListUserStatus;
  userScore: number;
  userEpisodesWatched: number;
  isRewatching: boolean;
  userComments: string;
  userNumTimesRewatched: number;
  userStartDate?: string;
  userFinishDate?: string;
  updatedAt?: string;
}

export interface IAnimeList extends IAnime, IAnimeUserList {}
