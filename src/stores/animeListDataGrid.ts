import { type OnChangeFn } from '@tanstack/react-table';
import { createTauriStore } from '@tauri-store/zustand';
import { create } from 'zustand';

import { AnimeListUserStatus } from '@/types/AnimeList';
import {
  MRT_ColumnSizingState,
  MRT_SortingState,
  MRT_VisibilityState
} from 'material-react-table';

type AnimeListDataGridStore = {
  searchValue: string;
  selectedStatus: AnimeListUserStatus;
  sorting: MRT_SortingState;
  columnVisibility: MRT_VisibilityState;
  columnSizing: MRT_ColumnSizingState;
  setSelectedStatus: (selectedStatus: AnimeListUserStatus) => void;
  setSearchValue: (searchValue: string) => void;
  onSortingChange: OnChangeFn<MRT_SortingState>;
  onColumnVisibilityChange: OnChangeFn<MRT_VisibilityState>;
  onColumnSizingChange: OnChangeFn<MRT_ColumnSizingState>;
};

export const useAnimeListDataGridStore = create<AnimeListDataGridStore>(
  (set) => ({
    searchValue: '',
    selectedStatus: 'watching',
    sorting: [],
    columnVisibility: { userStatus: false, genres: false },
    columnSizing: {},
    setSearchValue: (searchValue) => set(() => ({ searchValue })),
    setSelectedStatus: (selectedStatus) => set(() => ({ selectedStatus })),
    onSortingChange: (updaterOrValue) =>
      set((state) => {
        const sorting =
          typeof updaterOrValue === 'function'
            ? updaterOrValue(state.sorting)
            : updaterOrValue;

        return { sorting };
      }),
    onColumnVisibilityChange: (updaterOrValue) =>
      set((state) => {
        const columnVisibility =
          typeof updaterOrValue === 'function'
            ? updaterOrValue(state.columnVisibility)
            : updaterOrValue;

        return { columnVisibility };
      }),
    onColumnSizingChange: (updaterOrValue) =>
      set((state) => {
        const columnSizing =
          typeof updaterOrValue === 'function'
            ? updaterOrValue(state.columnSizing)
            : updaterOrValue;

        return { columnSizing };
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
