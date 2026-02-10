import { createTauriStore } from '@tauri-store/zustand';
import { create } from 'zustand';

import { IAnimeList } from '@/types/AnimeList';
import { createRef, RefObject } from 'react';

type AnimeDetailsStore = {
  isOpen: boolean;
  selectedAnime: IAnimeList | null;
  formRef: RefObject<HTMLFormElement | null>;
  setIsOpen: (isOpen: boolean) => void;
  setSelectedAnime: (selectedAnime: IAnimeList | null) => void;
  setFormRef: (formRef: RefObject<HTMLFormElement | null>) => void;
};

export const useAnimeDetailsStore = create<AnimeDetailsStore>((set) => ({
  isOpen: false,
  selectedAnime: null,
  formRef: createRef<HTMLFormElement | null>(),
  setIsOpen: (isOpen) => set(() => ({ isOpen })),
  setSelectedAnime: (selectedAnime) => set(() => ({ selectedAnime })),
  setFormRef: (formRef) => set(() => ({ formRef }))
}));

export const tauriHandler = createTauriStore(
  'animeDetails',
  useAnimeDetailsStore
);
