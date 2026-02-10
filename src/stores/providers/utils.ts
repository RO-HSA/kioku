import { SynchronizedAnimeList } from '@/services/backend/types';
import {
  AnimeListUserStatus,
  IAnimeList,
  IAnimeUserList
} from '@/types/AnimeList';

export const updateAnimeListData = (
  state: SynchronizedAnimeList | null,
  animeId: number,
  status: AnimeListUserStatus,
  data: Partial<IAnimeUserList>
): SynchronizedAnimeList | null => {
  if (!state) return null;

  if (!!data.userStatus && data.userStatus !== status) {
    let animeToMove: IAnimeList | null = null;

    for (const key of Object.keys(state) as AnimeListUserStatus[]) {
      const anime = state[key].find((anime) => anime.id === animeId);

      if (anime) {
        animeToMove = anime;
        break;
      }
    }

    if (!animeToMove) {
      const updatedState = {
        ...state,
        [status]: [...state[status], { id: animeId, ...data } as IAnimeList]
      };

      return updatedState;
    }

    const updatedState = { ...state };

    for (const key of Object.keys(updatedState) as AnimeListUserStatus[]) {
      updatedState[key] = updatedState[key].filter(
        (anime) => anime.id !== animeId
      );
    }

    const updatedAnimeListData = {
      ...updatedState,
      [data.userStatus]: [
        ...updatedState[data.userStatus],
        { ...animeToMove, ...data }
      ]
    };

    return updatedAnimeListData;
  }

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
