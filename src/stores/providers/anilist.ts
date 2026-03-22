import { createTauriStore } from '@tauri-store/zustand';
import { create } from 'zustand';

import { AnimeListService } from '@/services/backend/AnimeList';
import {
  SynchronizedAnimeList,
  SynchronizedMangaList
} from '@/services/backend/types';
import { AnimeListUserStatus, IAnimeList } from '@/types/AnimeList';
import { Provider } from '@/types/List';
import { MangaListUserStatus } from '@/types/MangaList';
import { Statistics } from '@/types/User';
import { updateAnimeListData, updateMangaListData } from './utils';

type ListStatus =
  | { type: 'anime'; value: AnimeListUserStatus }
  | { type: 'manga'; value: MangaListUserStatus };

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
    newProgress: number
  ) => void;
  setScore: (animeId: number, status: ListStatus, newScore: number) => void;
  updateAnimeList: (
    animeId: number,
    status: ListStatus,
    data: Partial<IAnimeList>
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
  setProgress: (animeId, status, newProgress) =>
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

          if (anime?.entryId === undefined) {
            return { animeListData: updatedAnimeListData };
          }

          AnimeListService.enqueueListUpdate({
            providerId: Provider.ANILIST,
            listType: 'anime',
            entryId: anime.entryId,
            userEpisodesWatched: newProgress
          });

          return { animeListData: updatedAnimeListData };
        case 'manga':
          if (!state.mangaListData) return {};

          const manga = state.mangaListData[status.value].find(
            (item) => item.id === animeId
          );

          const updatedMangaListData = updateMangaListData({
            mangaId: animeId,
            status: status.value,
            state: state.mangaListData,
            data: { userChaptersRead: newProgress }
          });

          if (manga?.entryId === undefined) {
            return { mangaListData: updatedMangaListData };
          }

          AnimeListService.enqueueListUpdate({
            providerId: Provider.ANILIST,
            listType: 'manga',
            entryId: manga.entryId,
            userChaptersRead: newProgress
          });

          return { mangaListData: updatedMangaListData };
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

          if (anime?.entryId === undefined) {
            return { animeListData: updatedAnimeListData };
          }

          AnimeListService.enqueueListUpdate({
            providerId: Provider.ANILIST,
            listType: 'anime',
            entryId: anime.entryId,
            userScore: newScore
          });

          return { animeListData: updatedAnimeListData };
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

          if (manga?.entryId === undefined) {
            return { mangaListData: updatedMangaListData };
          }

          AnimeListService.enqueueListUpdate({
            providerId: Provider.ANILIST,
            listType: 'manga',
            entryId: manga.entryId,
            userScore: newScore
          });

          return { mangaListData: updatedMangaListData };
        default:
          return {};
      }
    }),
  updateAnimeList: (
    animeId: number,
    currentStatus: ListStatus,
    data: Partial<IAnimeList>
  ) =>
    set((state) => {
      if (!state.animeListData) return {};

      switch (currentStatus.type) {
        case 'anime':
          const anime = state.animeListData[currentStatus.value].find(
            (item) => item.id === animeId
          );

          const updatedAnimeListData = updateAnimeListData({
            animeId,
            state: state.animeListData,
            status: currentStatus.value,
            data,
            isSingleUpdate: false
          });

          if (anime?.entryId === undefined) {
            return { animeListData: updatedAnimeListData };
          }

          AnimeListService.enqueueListUpdate({
            providerId: Provider.ANILIST,
            listType: 'anime',
            entryId: anime.entryId,
            ...data
          });

          return { animeListData: updatedAnimeListData };
        default:
          return {};
      }
    })
}));

export const tauriHandler = createTauriStore('anilist', useAniListStore, {
  autoStart: true,
  saveOnChange: true
});
