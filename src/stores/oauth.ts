import { createTauriStore } from '@tauri-store/zustand';
import { create } from 'zustand';

type OAuthStore = {
  codeVerifier: string | null;
  username: string | null;
  setCodeVerifier: (codeVerifier: string | null) => void;
  setUsername: (username: string | null) => void;
};

export const useOAuthStore = create<OAuthStore>((set) => ({
  codeVerifier: null,
  username: null,
  setCodeVerifier: (codeVerifier) => set(() => ({ codeVerifier })),
  setUsername: (username) => set(() => ({ username }))
}));

export const tauriHandler = createTauriStore('oauth', useOAuthStore, {
  autoStart: true,
  saveOnChange: true
});
