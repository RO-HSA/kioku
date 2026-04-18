import { createTauriStore } from '@tauri-store/zustand';
import { create } from 'zustand';

import { IAnimeList } from '@/types/AnimeList';
import { IMangaList } from '@/types/MangaList';

type ContextMenuPosition = {
  top: number;
  left: number;
};

type ContextMenuStore = {
  popoverPosition: ContextMenuPosition | null;
  state: IAnimeList | IMangaList | null;
  openContextMenu: (
    position: ContextMenuPosition,
    state: IAnimeList | IMangaList | null
  ) => void;
  closeContextMenu: () => void;
};

export const useContextMenuStore = create<ContextMenuStore>((set) => ({
  popoverPosition: null,
  state: null,
  openContextMenu: (position, state) => {
    set({ popoverPosition: position });
    set({ state });
  },
  closeContextMenu: () => set(() => ({ popoverPosition: null, state: null }))
}));

export const tauriHandler = createTauriStore(
  'contextMenu',
  useContextMenuStore
);
