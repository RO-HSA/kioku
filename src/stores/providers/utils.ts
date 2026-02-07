import {
  AnimeListUserStatus,
  IAnimeList,
  SynchronizedAnimeList
} from '@/services/backend/types';

export const updateAnimeListData = (
  state: SynchronizedAnimeList | null,
  animeId: number,
  status: AnimeListUserStatus,
  data: Partial<IAnimeList>
): SynchronizedAnimeList | null => {
  if (!state) return null;

  const updatedAnimeListData = {
    ...state,
    [status]: state[status].map((anime) => {
      if (anime.id === animeId) {
        return { ...anime, ...data };
      }

      return anime;
    })
  };

  return updatedAnimeListData;
};
