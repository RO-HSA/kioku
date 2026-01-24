import { createTauriStore } from '@tauri-store/zustand';
import { create } from 'zustand';

type SidebarStore = {
  isOpen: boolean;
  toggle: () => void;
  open: () => void;
  close: () => void;
  setOpen: (isOpen: boolean) => void;
};

export const useSidebarStore = create<SidebarStore>((set) => ({
  isOpen: false,
  toggle: () => set((state) => ({ isOpen: !state.isOpen })),
  open: () => set(() => ({ isOpen: true })),
  close: () => set(() => ({ isOpen: false })),
  setOpen: (isOpen) => set(() => ({ isOpen }))
}));

export const tauriHandler = createTauriStore('sidebar', useSidebarStore);
