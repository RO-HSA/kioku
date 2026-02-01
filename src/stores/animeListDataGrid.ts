import { AnimeListUserStatus } from '@/services/backend/types';
import { createTauriStore } from '@tauri-store/zustand';
import { create } from 'zustand';

type AnimeListDataGridStore = {
  selectedStatus: AnimeListUserStatus;
  setSelectedStatus: (selectedStatus: AnimeListUserStatus) => void;
};

export const useAnimeListDataGridStore = create<AnimeListDataGridStore>(
  (set) => ({
    selectedStatus: 'watching',
    setSelectedStatus: (selectedStatus) => set(() => ({ selectedStatus }))
  })
);

export const tauriHandler = createTauriStore(
  'animelistDataGrid',
  useAnimeListDataGridStore
);
