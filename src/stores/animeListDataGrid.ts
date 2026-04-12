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
  localSearchValue: string;
  remoteSearchValue: string;
  isLoading: boolean;
  selectedStatus: AnimeListUserStatus;
  sorting: MRT_SortingState;
  columnVisibility: MRT_VisibilityState;
  columnSizing: MRT_ColumnSizingState;
  setSelectedStatus: (selectedStatus: AnimeListUserStatus) => void;
  setLocalSearchValue: (searchValue: string) => void;
  setRemoteSearchValue: (searchValue: string) => void;
  setIsLoading: (isLoading: boolean) => void;
  onSortingChange: OnChangeFn<MRT_SortingState>;
  onColumnVisibilityChange: OnChangeFn<MRT_VisibilityState>;
  onColumnSizingChange: OnChangeFn<MRT_ColumnSizingState>;
};

export const useAnimeListDataGridStore = create<AnimeListDataGridStore>(
  (set) => ({
    localSearchValue: '',
    remoteSearchValue: '',
    isLoading: true,
    selectedStatus: 'watching',
    sorting: [],
    columnVisibility: { userStatus: false, genres: false },
    columnSizing: {},
    setLocalSearchValue: (searchValue) =>
      set(() => ({ localSearchValue: searchValue })),
    setRemoteSearchValue: (searchValue) =>
      set(() => ({ remoteSearchValue: searchValue })),
    setSelectedStatus: (selectedStatus) => set(() => ({ selectedStatus })),
    setIsLoading: (isLoading) => set(() => ({ isLoading })),
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
    filterKeys: ['localSearchValue', 'remoteSearchValue', 'isLoading'],
    filterKeysStrategy: 'omit'
  }
);
