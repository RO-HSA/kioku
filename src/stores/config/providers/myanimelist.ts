import { MyAnimeListService } from '@/services/backend/MyAnimeList';
import {
  AnimeListUserStatus,
  SynchronizedAnimeList
} from '@/services/backend/types';
import { createTauriStore } from '@tauri-store/zustand';
import { create } from 'zustand';

type MyAnimeListStore = {
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
};

export const useMyAnimeListStore = create<MyAnimeListStore>((set) => ({
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

      const updatedAnimeListData = {
        ...state.animeListData,
        [status]: state.animeListData[status].map((anime) => {
          if (anime.id === animeId) {
            return { ...anime, userEpisodesWatched: newProgress };
          }
          return anime;
        })
      };

      MyAnimeListService.enqueueListUpdate({
        providerId: 'myanimelist',
        entryId: animeId,
        userEpisodesWatched: newProgress
      });

      return { animeListData: updatedAnimeListData };
    })
}));

export const tauriHandler = createTauriStore(
  'myanimelist',
  useMyAnimeListStore,
  { autoStart: true }
);
