import { SynchronizedAnimeList } from '@/services/backend/types';
import {
  AnimeListUserStatus,
  IAnimeList,
  IAnimeUserList
} from '@/types/AnimeList';

interface MoveAnimeBetweenStatusesProps {
  state: SynchronizedAnimeList | null;
  animeToMove: IAnimeList;
  fromStatus: AnimeListUserStatus;
  toStatus: AnimeListUserStatus;
}

interface UpdateAnimeListDataProps {
  state: SynchronizedAnimeList | null;
  animeId: number;
  status: AnimeListUserStatus;
  isSingleUpdate?: boolean;
  data: Partial<IAnimeUserList>;
}

export const moveAnimeBetweenStatuses = ({
  state,
  animeToMove,
  fromStatus,
  toStatus
}: MoveAnimeBetweenStatusesProps): SynchronizedAnimeList => {
  if (!animeToMove || !state) return {} as SynchronizedAnimeList;

  const updatedState = { ...state };

  updatedState[fromStatus] = updatedState[fromStatus].filter(
    (anime) => anime.id !== animeToMove.id
  );

  updatedState[toStatus] = [...updatedState[toStatus], animeToMove];

  return updatedState;
};

export const updateAnimeListData = ({
  state,
  animeId,
  status,
  isSingleUpdate = true,
  data
}: UpdateAnimeListDataProps): SynchronizedAnimeList | null => {
  if (!state) return null;

  const updatedState = { ...state };

  const animeToUpdate = updatedState[status].find(
    (anime) => anime.id === animeId
  );

  if (!animeToUpdate) return updatedState;

  const updatedAnime: IAnimeList = {
    ...animeToUpdate,
    ...data
  };

  let targetStatus = data.userStatus ?? updatedAnime.userStatus;

  if (
    data.userEpisodesWatched !== undefined &&
    isSingleUpdate &&
    animeToUpdate.totalEpisodes > 0 &&
    data.userEpisodesWatched >= animeToUpdate.totalEpisodes
  ) {
    targetStatus = 'completed';
    updatedAnime.userStatus = 'completed';

    if (animeToUpdate.startDate) {
      const now = new Date();
      updatedAnime.userFinishDate = `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, '0')}-${String(now.getDate()).padStart(2, '0')}`;
    }
  } else {
    updatedAnime.userStatus = targetStatus;
  }

  if (animeToUpdate.userStatus !== targetStatus) {
    return moveAnimeBetweenStatuses({
      state: updatedState,
      animeToMove: updatedAnime,
      fromStatus: animeToUpdate.userStatus,
      toStatus: targetStatus
    });
  }

  const updatedAnimeListData = {
    ...updatedState,
    [updatedAnime.userStatus]: updatedState[updatedAnime.userStatus].map(
      (anime) => {
        if (anime.id === animeId) {
          return updatedAnime;
        }

        return anime;
      }
    )
  };

  return updatedAnimeListData;
};
