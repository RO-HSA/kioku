import { type OnChangeFn } from '@tanstack/react-table';
import { createTauriStore } from '@tauri-store/zustand';
import { create } from 'zustand';

import { AnimeListUserStatus } from '@/types/AnimeList';
import { MRT_SortingState } from 'material-react-table';

type AnimeListDataGridStore = {
  searchValue: string;
  selectedStatus: AnimeListUserStatus;
  sorting: MRT_SortingState;
  setSelectedStatus: (selectedStatus: AnimeListUserStatus) => void;
  setSearchValue: (searchValue: string) => void;
  onSortingChange: OnChangeFn<MRT_SortingState>;
};

export const useAnimeListDataGridStore = create<AnimeListDataGridStore>(
  (set) => ({
    searchValue: '',
    selectedStatus: 'watching',
    sorting: [],
    setSearchValue: (searchValue) => set(() => ({ searchValue })),
    setSelectedStatus: (selectedStatus) => set(() => ({ selectedStatus })),
    onSortingChange: (updaterOrValue) =>
      set((state) => {
        const sorting =
          typeof updaterOrValue === 'function'
            ? updaterOrValue(state.sorting)
            : updaterOrValue;

        return { sorting };
      })
  })
);

export const tauriHandler = createTauriStore(
  'animelistDataGrid',
  useAnimeListDataGridStore,
  {
    autoStart: true,
    saveOnChange: true,
    filterKeys: ['searchValue'],
    filterKeysStrategy: 'omit'
  }
);
