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

  let updatedState = { ...state };

  const animeToUpdate = updatedState[status].find(
    (anime) => anime.id === animeId
  );

  if (!animeToUpdate) return updatedState;

  const updatedAnime: IAnimeList = {
    ...animeToUpdate,
    ...data
  };

  if (
    data.userEpisodesWatched !== undefined &&
    isSingleUpdate &&
    animeToUpdate.totalEpisodes > 0 &&
    data.userEpisodesWatched >= animeToUpdate.totalEpisodes
  ) {
    updatedAnime.userStatus = 'completed';

    if (animeToUpdate.startDate) {
      const now = new Date();
      updatedAnime.userFinishDate = `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, '0')}-${String(now.getDate()).padStart(2, '0')}`;
    }

    updatedState = moveAnimeBetweenStatuses({
      state,
      animeToMove: updatedAnime,
      fromStatus: status,
      toStatus: 'completed'
    });
  }

  if (data.userStatus !== status || status !== updatedAnime.userStatus) {
    updatedState = moveAnimeBetweenStatuses({
      state,
      animeToMove: updatedAnime,
      fromStatus: animeToUpdate.userStatus,
      toStatus: data.userStatus ?? updatedAnime.userStatus
    });
  }

  const updatedAnimeListData = {
    ...updatedState,
    [updatedAnime.userStatus]: updatedState[updatedAnime.userStatus].map(
      (anime) => {
        if (anime.id === animeId) {
          return { ...anime, ...data };
        }

        return anime;
      }
    )
  };

  return updatedAnimeListData;
};
