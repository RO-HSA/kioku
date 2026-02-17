import { createTauriStore } from '@tauri-store/zustand';
import { create } from 'zustand';

import { AnimeListService } from '@/services/backend/AnimeList';
import { SynchronizedAnimeList } from '@/services/backend/types';
import { AnimeListUserStatus, IAnimeList } from '@/types/AnimeList';
import { Provider } from '@/types/List';
import { updateAnimeListData } from './utils';

type AniListStore = {
  username: string | null;
  isAuthenticating: boolean;
  isAuthenticated: boolean;
  isReauthenticating: boolean;
  animeListData: SynchronizedAnimeList | null;
  setUsername: (username: string | null) => void;
  setIsAuthenticating: (isAuthenticating: boolean) => void;
  setIsAuthenticated: (isAuthenticated: boolean) => void;
  setIsReauthenticating: (isReauthenticating: boolean) => void;
  setAnimeListData: (animeListData: SynchronizedAnimeList | null) => void;
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
  username: null,
  isAuthenticating: false,
  isAuthenticated: false,
  isReauthenticating: false,
  animeListData: null,
  setIsAuthenticated: (isAuthenticated) => set(() => ({ isAuthenticated })),
  setUsername: (username) => set(() => ({ username })),
  setIsAuthenticating: (isAuthenticating) => set(() => ({ isAuthenticating })),
  setIsReauthenticating: (isReauthenticating) =>
    set(() => ({ isReauthenticating })),
  setAnimeListData: (animeListData) => set(() => ({ animeListData })),
  setProgress: (animeId, status, newProgress) =>
    set((state) => {
      if (!state.animeListData) return {};

      const updatedAnimeListData = updateAnimeListData({
        animeId,
        status,
        state: state.animeListData,
        data: { userEpisodesWatched: newProgress }
      });

      AnimeListService.enqueueListUpdate({
        providerId: Provider.ANILIST,
        entryId: animeId,
        userEpisodesWatched: newProgress
      });

      return { animeListData: updatedAnimeListData };
    }),
  setScore: (animeId, status, newScore) =>
    set((state) => {
      if (!state.animeListData) return {};

      const updatedAnimeListData = updateAnimeListData({
        animeId,
        status,
        state: state.animeListData,
        data: { userScore: newScore }
      });

      AnimeListService.enqueueListUpdate({
        providerId: Provider.ANILIST,
        entryId: animeId,
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

      const updatedAnimeListData = updateAnimeListData({
        animeId,
        state: state.animeListData,
        status: currentStatus,
        data,
        isSingleUpdate: false
      });

      AnimeListService.enqueueListUpdate({
        providerId: Provider.ANILIST,
        entryId: animeId,
        ...data
      });

      return { animeListData: updatedAnimeListData };
    })
}));

export const tauriHandler = createTauriStore('anilist', useAniListStore, {
  autoStart: true,
  saveOnChange: true
});
