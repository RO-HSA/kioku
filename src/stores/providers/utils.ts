import {
  SynchronizedAnimeList,
  SynchronizedMangaList
} from '@/services/backend/types';
import {
  AnimeListUserStatus,
  IAnimeList,
  IAnimeUserList
} from '@/types/AnimeList';
import {
  IMangaList,
  IMangaUserList,
  MangaListUserStatus
} from '@/types/MangaList';

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

interface MoveMangaBetweenStatusesProps {
  state: SynchronizedMangaList | null;
  mangaToMove: IMangaList;
  fromStatus: MangaListUserStatus;
  toStatus: MangaListUserStatus;
}

interface UpdateMangaListDataProps {
  state: SynchronizedMangaList | null;
  mangaId: number;
  status: MangaListUserStatus;
  isSingleUpdate?: boolean;
  data: Partial<IMangaUserList>;
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

    if (animeToUpdate.userStartDate && !animeToUpdate.userFinishDate) {
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

export const moveMangaBetweenStatuses = ({
  state,
  mangaToMove,
  fromStatus,
  toStatus
}: MoveMangaBetweenStatusesProps): SynchronizedMangaList => {
  if (!mangaToMove || !state) return {} as SynchronizedMangaList;

  const updatedState = { ...state };

  updatedState[fromStatus] = updatedState[fromStatus].filter(
    (manga) => manga.id !== mangaToMove.id
  );

  updatedState[toStatus] = [...updatedState[toStatus], mangaToMove];

  return updatedState;
};

export const updateMangaListData = ({
  state,
  mangaId,
  status,
  isSingleUpdate = true,
  data
}: UpdateMangaListDataProps): SynchronizedMangaList | null => {
  if (!state) return null;

  const updatedState = { ...state };

  const mangaToUpdate = updatedState[status].find(
    (manga) => manga.id === mangaId
  );

  if (!mangaToUpdate) return updatedState;

  const updatedManga: IMangaList = {
    ...mangaToUpdate,
    ...data
  };

  let targetStatus = data.userStatus ?? updatedManga.userStatus;

  if (
    data.userChaptersRead !== undefined &&
    isSingleUpdate &&
    mangaToUpdate.totalChapters > 0 &&
    data.userChaptersRead >= mangaToUpdate.totalChapters
  ) {
    targetStatus = 'completed';
    updatedManga.userStatus = 'completed';

    if (mangaToUpdate.userStartDate && !mangaToUpdate.userFinishDate) {
      const now = new Date();
      updatedManga.userFinishDate = `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, '0')}-${String(now.getDate()).padStart(2, '0')}`;
    }
  } else {
    updatedManga.userStatus = targetStatus;
  }

  if (mangaToUpdate.userStatus !== targetStatus) {
    return moveMangaBetweenStatuses({
      state: updatedState,
      mangaToMove: updatedManga,
      fromStatus: mangaToUpdate.userStatus,
      toStatus: targetStatus
    });
  }

  const updatedMangaListData = {
    ...updatedState,
    [updatedManga.userStatus]: updatedState[updatedManga.userStatus].map(
      (manga) => {
        if (manga.id === mangaId) {
          return updatedManga;
        }

        return manga;
      }
    )
  };

  return updatedMangaListData;
};
