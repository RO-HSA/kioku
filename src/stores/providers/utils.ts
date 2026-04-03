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

interface UpdateAnimeListReturn {
  updatedAnimeList: SynchronizedAnimeList | null;
  updatedAnime?: IAnimeList;
}

interface UpdateMangaListReturn {
  updatedMangaList: SynchronizedMangaList | null;
  updatedManga?: IMangaList;
}

export const moveAnimeBetweenStatuses = ({
  state,
  animeToMove,
  fromStatus,
  toStatus
}: MoveAnimeBetweenStatusesProps): UpdateAnimeListReturn => {
  if (!animeToMove || !state) return {} as UpdateAnimeListReturn;

  const updatedState = { ...state };

  updatedState[fromStatus] = updatedState[fromStatus].filter(
    (anime) => anime.id !== animeToMove.id
  );

  updatedState[toStatus] = [...updatedState[toStatus], animeToMove];

  return { updatedAnimeList: updatedState, updatedAnime: animeToMove };
};

export const updateAnimeListData = ({
  state,
  animeId,
  status,
  isSingleUpdate = true,
  data
}: UpdateAnimeListDataProps): UpdateAnimeListReturn | null => {
  if (!state) return null;

  const updatedState = { ...state };

  const animeToUpdate = updatedState[status].find(
    (anime) => anime.id === animeId
  );

  if (!animeToUpdate) return { updatedAnimeList: updatedState };

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

  return {
    updatedAnimeList: updatedAnimeListData,
    updatedAnime: updatedAnime
  };
};

export const moveMangaBetweenStatuses = ({
  state,
  mangaToMove,
  fromStatus,
  toStatus
}: MoveMangaBetweenStatusesProps): UpdateMangaListReturn => {
  if (!mangaToMove || !state) return {} as UpdateMangaListReturn;

  const updatedState = { ...state };

  updatedState[fromStatus] = updatedState[fromStatus].filter(
    (manga) => manga.id !== mangaToMove.id
  );

  updatedState[toStatus] = [...updatedState[toStatus], mangaToMove];

  return { updatedMangaList: updatedState, updatedManga: mangaToMove };
};

export const updateMangaListData = ({
  state,
  mangaId,
  status,
  isSingleUpdate = true,
  data
}: UpdateMangaListDataProps): UpdateMangaListReturn | null => {
  if (!state) return null;

  const updatedState = { ...state };

  const mangaToUpdate = updatedState[status].find(
    (manga) => manga.id === mangaId
  );

  if (!mangaToUpdate) return { updatedMangaList: updatedState };

  const updatedManga: IMangaList = {
    ...mangaToUpdate,
    ...data
  };

  let targetStatus = data.userStatus ?? updatedManga.userStatus;
  const hasReachedChapterTotal =
    data.userChaptersRead !== undefined &&
    mangaToUpdate.totalChapters > 0 &&
    data.userChaptersRead >= mangaToUpdate.totalChapters;
  const hasReachedVolumeTotal =
    data.userVolumesRead !== undefined &&
    mangaToUpdate.totalVolumes > 0 &&
    data.userVolumesRead >= mangaToUpdate.totalVolumes;

  if (
    isSingleUpdate &&
    (hasReachedChapterTotal ||
      (hasReachedVolumeTotal && mangaToUpdate.totalChapters === 0))
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

  return { updatedMangaList: updatedMangaListData, updatedManga: updatedManga };
};
