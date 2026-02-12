import { createTauriStore } from '@tauri-store/zustand';
import { create } from 'zustand';

import { AnimeListUserStatus } from '@/types/AnimeList';

type AnimeListDataGridStore = {
  searchValue: string;
  selectedStatus: AnimeListUserStatus;
  setSelectedStatus: (selectedStatus: AnimeListUserStatus) => void;
  setSearchValue: (searchValue: string) => void;
};

export const useAnimeListDataGridStore = create<AnimeListDataGridStore>(
  (set) => ({
    searchValue: '',
    selectedStatus: 'watching',
    setSearchValue: (searchValue) => set(() => ({ searchValue })),
    setSelectedStatus: (selectedStatus) => set(() => ({ selectedStatus }))
  })
);

export const tauriHandler = createTauriStore(
  'animelistDataGrid',
  useAnimeListDataGridStore
);
