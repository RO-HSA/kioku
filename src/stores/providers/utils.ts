import { SynchronizedAnimeList } from '@/services/backend/types';
import { AnimeListUserStatus, IAnimeList } from '@/types/AnimeList';

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
