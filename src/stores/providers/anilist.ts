import { createTauriStore } from '@tauri-store/zustand';
import { create } from 'zustand';

import { AnimeListService } from '@/services/backend/AnimeList';
import {
  SynchronizedAnimeList,
  SynchronizedMangaList
} from '@/services/backend/types';
import { AnimeListUserStatus, IAnimeList } from '@/types/AnimeList';
import { Provider } from '@/types/List';
import { IMangaList, MangaListUserStatus } from '@/types/MangaList';
import { Statistics } from '@/types/User';
import { updateAnimeListData, updateMangaListData } from './utils';

type ListStatus =
  | { type: 'anime'; value: AnimeListUserStatus }
  | { type: 'manga'; value: MangaListUserStatus };

type MangaProgressType = 'chapters' | 'volumes';

type AniListStore = {
  id: number | null;
  username: string | null;
  profilePictureUrl: string | null;
  statistics: Statistics | null;
  isAuthenticating: boolean;
  isAuthenticated: boolean;
  isReauthenticating: boolean;
  animeListData: SynchronizedAnimeList | null;
  mangaListData: SynchronizedMangaList | null;
  setId: (id: number | null) => void;
  setUsername: (username: string | null) => void;
  setProfilePictureUrl: (url: string | null) => void;
  setStatistics: (statistics: Statistics | null) => void;
  setIsAuthenticating: (isAuthenticating: boolean) => void;
  setIsAuthenticated: (isAuthenticated: boolean) => void;
  setIsReauthenticating: (isReauthenticating: boolean) => void;
  setAnimeListData: (animeListData: SynchronizedAnimeList | null) => void;
  setMangaListData: (mangaListData: SynchronizedMangaList | null) => void;
  signOut: () => void;
  setProgress: (
    animeId: number,
    status: ListStatus,
    newProgress: number,
    progressType?: MangaProgressType
  ) => void;
  setScore: (animeId: number, status: ListStatus, newScore: number) => void;
  updateAnimeList: (
    entryId: number,
    status: AnimeListUserStatus,
    data: Partial<IAnimeList>
  ) => void;
  updateMangaList: (
    entryId: number,
    status: MangaListUserStatus,
    data: Partial<IMangaList>
  ) => void;
};

export const useAniListStore = create<AniListStore>((set) => ({
  id: null,
  username: null,
  profilePictureUrl: null,
  statistics: null,
  isAuthenticating: false,
  isAuthenticated: false,
  isReauthenticating: false,
  animeListData: null,
  mangaListData: null,
  setId: (id) => set(() => ({ id })),
  setUsername: (username) => set(() => ({ username })),
  setProfilePictureUrl: (url) => set(() => ({ profilePictureUrl: url })),
  setStatistics: (statistics) => set(() => ({ statistics })),
  setIsAuthenticated: (isAuthenticated) => set(() => ({ isAuthenticated })),
  setIsAuthenticating: (isAuthenticating) => set(() => ({ isAuthenticating })),
  setIsReauthenticating: (isReauthenticating) =>
    set(() => ({ isReauthenticating })),
  setAnimeListData: (animeListData) => set(() => ({ animeListData })),
  setMangaListData: (mangaListData) => set(() => ({ mangaListData })),
  signOut: () =>
    set(() => ({
      id: null,
      username: null,
      profilePictureUrl: null,
      statistics: null,
      isAuthenticated: false,
      isAuthenticating: false,
      isReauthenticating: false,
      animeListData: null,
      mangaListData: null
    })),
  setProgress: (animeId, status, newProgress, progressType = 'chapters') =>
    set((state) => {
      switch (status.type) {
        case 'anime':
          if (!state.animeListData) return {};

          const anime = state.animeListData[status.value].find(
            (item) => item.id === animeId
          );

          const updatedAnimeListData = updateAnimeListData({
            animeId,
            status: status.value,
            state: state.animeListData,
            data: { userEpisodesWatched: newProgress }
          });

          const { updatedAnimeList, updatedAnime } = updatedAnimeListData || {};

          if (anime?.entryId === undefined) {
            return { animeListData: updatedAnimeList || null };
          }

          AnimeListService.enqueueListUpdate({
            providerId: Provider.ANILIST,
            listType: 'anime',
            entryId: anime.entryId,
            userEpisodesWatched: newProgress,
            userStatus: updatedAnime ? updatedAnime.userStatus : undefined
          });

          return { animeListData: updatedAnimeList || null };
        case 'manga':
          if (!state.mangaListData) return {};

          const progressData =
            progressType === 'volumes'
              ? { userVolumesRead: newProgress }
              : { userChaptersRead: newProgress };

          const manga = state.mangaListData[status.value].find(
            (item) => item.id === animeId
          );

          const updatedMangaListData = updateMangaListData({
            mangaId: animeId,
            status: status.value,
            state: state.mangaListData,
            data: progressData
          });

          const { updatedMangaList } = updatedMangaListData || {};

          if (manga?.entryId === undefined) {
            return { mangaListData: updatedMangaList || null };
          }

          AnimeListService.enqueueListUpdate({
            providerId: Provider.ANILIST,
            listType: 'manga',
            entryId: manga.entryId,
            ...progressData
          });

          return { mangaListData: updatedMangaList || null };
        default:
          return {};
      }
    }),
  setScore: (animeId, status, newScore) =>
    set((state) => {
      switch (status.type) {
        case 'anime':
          if (!state.animeListData) return {};

          const anime = state.animeListData[status.value].find(
            (item) => item.id === animeId
          );

          const updatedAnimeListData = updateAnimeListData({
            animeId,
            status: status.value,
            state: state.animeListData,
            data: { userScore: newScore }
          });

          const { updatedAnimeList } = updatedAnimeListData || {};

          if (anime?.entryId === undefined) {
            return { animeListData: updatedAnimeList || null };
          }

          AnimeListService.enqueueListUpdate({
            providerId: Provider.ANILIST,
            listType: 'anime',
            entryId: anime.entryId,
            userScore: newScore
          });

          return { animeListData: updatedAnimeList || null };
        case 'manga':
          if (!state.mangaListData) return {};

          const manga = state.mangaListData[status.value].find(
            (item) => item.id === animeId
          );

          const updatedMangaListData = updateMangaListData({
            mangaId: animeId,
            status: status.value,
            state: state.mangaListData,
            data: { userScore: newScore }
          });

          const { updatedMangaList } = updatedMangaListData || {};

          if (manga?.entryId === undefined) {
            return { mangaListData: updatedMangaList || null };
          }

          AnimeListService.enqueueListUpdate({
            providerId: Provider.ANILIST,
            listType: 'manga',
            entryId: manga.entryId,
            userScore: newScore
          });

          return { mangaListData: updatedMangaList || null };
        default:
          return {};
      }
    }),
  updateAnimeList: (
    entryId: number,
    currentStatus: AnimeListUserStatus,
    data: Partial<IAnimeList>
  ) =>
    set((state) => {
      if (!state.animeListData) return {};

      const anime = state.animeListData[currentStatus].find(
        (item) => item.id === entryId
      );

      const updatedAnimeListData = updateAnimeListData({
        animeId: entryId,
        state: state.animeListData,
        status: currentStatus,
        data,
        isSingleUpdate: false
      });

      const { updatedAnimeList } = updatedAnimeListData || {};

      if (anime?.entryId === undefined) {
        return { animeListData: updatedAnimeList || null };
      }

      AnimeListService.enqueueListUpdate({
        providerId: Provider.ANILIST,
        listType: 'anime',
        entryId: anime.entryId,
        ...data
      });

      return { animeListData: updatedAnimeList || null };
    }),
  updateMangaList: (
    entryId: number,
    currentStatus: MangaListUserStatus,
    data: Partial<IMangaList>
  ) =>
    set((state) => {
      if (!state.mangaListData) return {};

      const manga = state.mangaListData[currentStatus].find(
        (item) => item.id === entryId
      );

      const updatedMangaListData = updateMangaListData({
        mangaId: entryId,
        state: state.mangaListData,
        status: currentStatus,
        data,
        isSingleUpdate: false
      });

      const { updatedMangaList } = updatedMangaListData || {};

      if (manga?.entryId === undefined) {
        return { mangaListData: updatedMangaList || null };
      }

      AnimeListService.enqueueListUpdate({
        providerId: Provider.ANILIST,
        listType: 'manga',
        entryId: manga.entryId,
        ...data
      });

      return { mangaListData: updatedMangaList || null };
    })
}));

export const tauriHandler = createTauriStore('anilist', useAniListStore, {
  autoStart: true,
  saveOnChange: true
});
