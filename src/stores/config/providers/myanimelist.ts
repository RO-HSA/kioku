import { SynchronizedAnimeList } from '@/services/backend/types';
import { createTauriStore } from '@tauri-store/zustand';
import { create } from 'zustand';

type MyAnimeListStore = {
  username: string | null;
  isAuthenticating: boolean;
  isAuthenticated: boolean;
  isReauthenticating: boolean;
  listData: SynchronizedAnimeList | null;
  setUsername: (username: string | null) => void;
  setIsAuthenticating: (isAuthenticating: boolean) => void;
  setIsAuthenticated: (isAuthenticated: boolean) => void;
  setIsReauthenticating: (isReauthenticating: boolean) => void;
  setListData: (listData: SynchronizedAnimeList | null) => void;
};

export const useMyAnimeListStore = create<MyAnimeListStore>((set) => ({
  username: null,
  isAuthenticating: false,
  isAuthenticated: false,
  isReauthenticating: false,
  listData: null,
  setIsAuthenticated: (isAuthenticated) => set(() => ({ isAuthenticated })),
  setUsername: (username) => set(() => ({ username })),
  setIsAuthenticating: (isAuthenticating) => set(() => ({ isAuthenticating })),
  setIsReauthenticating: (isReauthenticating) =>
    set(() => ({ isReauthenticating })),
  setListData: (listData) => set(() => ({ listData }))
}));

export const tauriHandler = createTauriStore(
  'myanimelist',
  useMyAnimeListStore,
  { autoStart: true }
);
