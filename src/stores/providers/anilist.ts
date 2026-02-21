import { createTauriStore } from '@tauri-store/zustand';
import { create } from 'zustand';

import { AnimeListService } from '@/services/backend/AnimeList';
import { SynchronizedAnimeList } from '@/services/backend/types';
import { AnimeListUserStatus, IAnimeList } from '@/types/AnimeList';
import { Provider } from '@/types/List';
import { Statistics } from '@/types/User';
import { updateAnimeListData } from './utils';

type AniListStore = {
  id: number | null;
  username: string | null;
  profilePictureUrl: string | null;
  statistics: Statistics | null;
  isAuthenticating: boolean;
  isAuthenticated: boolean;
  isReauthenticating: boolean;
  animeListData: SynchronizedAnimeList | null;
  setId: (id: number | null) => void;
  setUsername: (username: string | null) => void;
  setProfilePictureUrl: (url: string | null) => void;
  setStatistics: (statistics: Statistics | null) => void;
  setIsAuthenticating: (isAuthenticating: boolean) => void;
  setIsAuthenticated: (isAuthenticated: boolean) => void;
  setIsReauthenticating: (isReauthenticating: boolean) => void;
  setAnimeListData: (animeListData: SynchronizedAnimeList | null) => void;
  signOut: () => void;
  setProgress: (
    animeId: number,
    status: AnimeListUserStatus,
    newProgress: number
  ) => void;
  setScore: (
    animeId: number,
    status: AnimeListUserStatus,
    newScore: number
  ) => void;
  updateAnimeList: (
    animeId: number,
    status: AnimeListUserStatus,
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
  setId: (id) => set(() => ({ id })),
  setUsername: (username) => set(() => ({ username })),
  setProfilePictureUrl: (url) => set(() => ({ profilePictureUrl: url })),
  setStatistics: (statistics) => set(() => ({ statistics })),
  setIsAuthenticated: (isAuthenticated) => set(() => ({ isAuthenticated })),
  setIsAuthenticating: (isAuthenticating) => set(() => ({ isAuthenticating })),
  setIsReauthenticating: (isReauthenticating) =>
    set(() => ({ isReauthenticating })),
  setAnimeListData: (animeListData) => set(() => ({ animeListData })),
  signOut: () =>
    set(() => ({
      id: null,
      username: null,
      profilePictureUrl: null,
      statistics: null,
      isAuthenticated: false,
      isAuthenticating: false,
      isReauthenticating: false,
      animeListData: null
    })),
  setProgress: (animeId, status, newProgress) =>
    set((state) => {
      if (!state.animeListData) return {};

      const anime = state.animeListData[status].find(
        (item) => item.id === animeId
      );

      const updatedAnimeListData = updateAnimeListData({
        animeId,
        status,
        state: state.animeListData,
        data: { userEpisodesWatched: newProgress }
      });

      if (anime?.entryId === undefined) {
        return { animeListData: updatedAnimeListData };
      }

      AnimeListService.enqueueListUpdate({
        providerId: Provider.ANILIST,
        entryId: anime.entryId,
        userEpisodesWatched: newProgress
      });

      return { animeListData: updatedAnimeListData };
    }),
  setScore: (animeId, status, newScore) =>
    set((state) => {
      if (!state.animeListData) return {};

      const anime = state.animeListData[status].find(
        (item) => item.id === animeId
      );

      const updatedAnimeListData = updateAnimeListData({
        animeId,
        status,
        state: state.animeListData,
        data: { userScore: newScore }
      });

      if (anime?.entryId === undefined) {
        return { animeListData: updatedAnimeListData };
      }

      AnimeListService.enqueueListUpdate({
        providerId: Provider.ANILIST,
        entryId: anime.entryId,
        userScore: newScore
      });

      return { animeListData: updatedAnimeListData };
    }),
  updateAnimeList: (
    animeId: number,
    currentStatus: AnimeListUserStatus,
    data: Partial<IAnimeList>
  ) =>
    set((state) => {
      if (!state.animeListData) return {};

      const anime = state.animeListData[currentStatus].find(
        (item) => item.id === animeId
      );

      const updatedAnimeListData = updateAnimeListData({
        animeId,
        state: state.animeListData,
        status: currentStatus,
        data,
        isSingleUpdate: false
      });

      if (anime?.entryId === undefined) {
        return { animeListData: updatedAnimeListData };
      }

      AnimeListService.enqueueListUpdate({
        providerId: Provider.ANILIST,
        entryId: anime.entryId,
        ...data
      });

      return { animeListData: updatedAnimeListData };
    })
}));

export const tauriHandler = createTauriStore('anilist', useAniListStore, {
  autoStart: true,
  saveOnChange: true
});
