import { IAnimeList } from '@/services/backend/types';
import { createTauriStore } from '@tauri-store/zustand';
import { create } from 'zustand';

type AnimeDetailsStore = {
  isOpen: boolean;
  selectedAnime: IAnimeList | null;
  setIsOpen: (isOpen: boolean) => void;
  setSelectedAnime: (selectedAnime: IAnimeList | null) => void;
};

export const useAnimeDetailsStore = create<AnimeDetailsStore>((set) => ({
  isOpen: false,
  selectedAnime: null,
  setIsOpen: (isOpen) => set(() => ({ isOpen })),
  setSelectedAnime: (selectedAnime) => set(() => ({ selectedAnime }))
}));

export const tauriHandler = createTauriStore(
  'animeDetails',
  useAnimeDetailsStore
);
