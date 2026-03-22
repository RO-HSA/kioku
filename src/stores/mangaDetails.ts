import { createTauriStore } from '@tauri-store/zustand';
import { create } from 'zustand';

import { IMangaList } from '@/types/MangaList';
import { createRef, RefObject } from 'react';

type MangaDetailsStore = {
  isOpen: boolean;
  selectedManga: IMangaList | null;
  formRef: RefObject<HTMLFormElement | null>;
  setIsOpen: (isOpen: boolean) => void;
  setSelectedManga: (selectedManga: IMangaList | null) => void;
  setFormRef: (formRef: RefObject<HTMLFormElement | null>) => void;
};

export const useMangaDetailsStore = create<MangaDetailsStore>((set) => ({
  isOpen: false,
  selectedManga: null,
  formRef: createRef<HTMLFormElement | null>(),
  setIsOpen: (isOpen) => set(() => ({ isOpen })),
  setSelectedManga: (selectedManga) => set(() => ({ selectedManga })),
  setFormRef: (formRef) => set(() => ({ formRef }))
}));

export const tauriHandler = createTauriStore(
  'mangaDetails',
  useMangaDetailsStore
);
