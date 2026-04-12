import { type OnChangeFn } from '@tanstack/react-table';
import { createTauriStore } from '@tauri-store/zustand';
import { create } from 'zustand';

import { MangaListUserStatus } from '@/types/MangaList';
import {
  MRT_ColumnSizingState,
  MRT_SortingState,
  MRT_VisibilityState
} from 'material-react-table';

type MangaListDataGridStore = {
  localSearchValue: string;
  remoteSearchValue: string;
  isLoading: boolean;
  selectedStatus: MangaListUserStatus;
  sorting: MRT_SortingState;
  columnVisibility: MRT_VisibilityState;
  columnSizing: MRT_ColumnSizingState;
  setSelectedStatus: (selectedStatus: MangaListUserStatus) => void;
  setLocalSearchValue: (searchValue: string) => void;
  setRemoteSearchValue: (searchValue: string) => void;
  setIsloading: (isLoading: boolean) => void;
  onSortingChange: OnChangeFn<MRT_SortingState>;
  onColumnVisibilityChange: OnChangeFn<MRT_VisibilityState>;
  onColumnSizingChange: OnChangeFn<MRT_ColumnSizingState>;
};

export const useMangaListDataGridStore = create<MangaListDataGridStore>(
  (set) => ({
    localSearchValue: '',
    remoteSearchValue: '',
    isLoading: true,
    selectedStatus: 'reading',
    sorting: [],
    columnVisibility: { userStatus: false, genres: false },
    columnSizing: {},
    setLocalSearchValue: (searchValue) =>
      set(() => ({ localSearchValue: searchValue })),
    setRemoteSearchValue: (searchValue) =>
      set(() => ({ remoteSearchValue: searchValue })),
    setSelectedStatus: (selectedStatus) => set(() => ({ selectedStatus })),
    setIsloading: (isLoading) => set(() => ({ isLoading })),
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
  'mangalistDataGrid',
  useMangaListDataGridStore,
  {
    autoStart: true,
    saveOnChange: true,
    filterKeys: ['localSearchValue', 'remoteSearchValue', 'isLoading'],
    filterKeysStrategy: 'omit'
  }
);
