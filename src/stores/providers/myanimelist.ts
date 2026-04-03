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

type MyAnimeListStore = {
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
    entryId: number,
    status: ListStatus,
    newProgress: number,
    progressType?: MangaProgressType
  ) => void;
  setScore: (entryId: number, status: ListStatus, newScore: number) => void;
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

export const useMyAnimeListStore = create<MyAnimeListStore>((set) => ({
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
  setProgress: (entryId, status, newProgress, progressType = 'chapters') =>
    set((state) => {
      switch (status.type) {
        case 'anime':
          if (!state.animeListData) return {};

          const updatedAnimeListData = updateAnimeListData({
            animeId: entryId,
            status: status.value,
            state: state.animeListData,
            data: { userEpisodesWatched: newProgress }
          });

          const { updatedAnimeList, updatedAnime } = updatedAnimeListData || {};

          AnimeListService.enqueueListUpdate({
            providerId: Provider.MY_ANIME_LIST,
            listType: 'anime',
            entryId,
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

          const updatedMangaListData = updateMangaListData({
            mangaId: entryId,
            status: status.value,
            state: state.mangaListData,
            data: progressData
          });

          const { updatedMangaList, updatedManga } = updatedMangaListData || {};

          AnimeListService.enqueueListUpdate({
            providerId: Provider.MY_ANIME_LIST,
            listType: 'manga',
            entryId,
            userStatus: updatedManga ? updatedManga.userStatus : undefined,
            ...progressData
          });

          return { mangaListData: updatedMangaList || null };
        default:
          return {};
      }
    }),
  setScore: (entryId, status, newScore) =>
    set((state) => {
      switch (status.type) {
        case 'anime':
          if (!state.animeListData) return {};

          AnimeListService.enqueueListUpdate({
            providerId: Provider.MY_ANIME_LIST,
            listType: 'anime',
            entryId,
            userScore: newScore
          });

          const updatedAnimeListData = updateAnimeListData({
            animeId: entryId,
            status: status.value,
            state: state.animeListData,
            data: { userScore: newScore }
          });

          const { updatedAnimeList } = updatedAnimeListData || {};

          return { animeListData: updatedAnimeList || null };
        case 'manga':
          if (!state.mangaListData) return {};

          AnimeListService.enqueueListUpdate({
            providerId: Provider.MY_ANIME_LIST,
            listType: 'manga',
            entryId,
            userScore: newScore
          });

          const updatedMangaListData = updateMangaListData({
            mangaId: entryId,
            status: status.value,
            state: state.mangaListData,
            data: { userScore: newScore }
          });

          const { updatedMangaList } = updatedMangaListData || {};

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

      const updatedAnimeListData = updateAnimeListData({
        animeId: entryId,
        state: state.animeListData,
        status: currentStatus,
        data,
        isSingleUpdate: false
      });

      const { updatedAnimeList } = updatedAnimeListData || {};

      AnimeListService.enqueueListUpdate({
        providerId: Provider.MY_ANIME_LIST,
        listType: 'anime',
        entryId,
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

      const updatedMangaListData = updateMangaListData({
        mangaId: entryId,
        state: state.mangaListData,
        status: currentStatus,
        data,
        isSingleUpdate: false
      });

      const { updatedMangaList } = updatedMangaListData || {};

      AnimeListService.enqueueListUpdate({
        providerId: Provider.MY_ANIME_LIST,
        listType: 'manga',
        entryId,
        ...data
      });

      return { mangaListData: updatedMangaList };
    })
}));

export const tauriHandler = createTauriStore(
  'myanimelist',
  useMyAnimeListStore,
  { autoStart: true, saveOnChange: true }
);
