import { SidebarNavigationStep } from '@/types/Navigation';
import { createTauriStore } from '@tauri-store/zustand';
import { create } from 'zustand';

type SidebarNavigationStore = {
  navigationStep: SidebarNavigationStep;
  setNavigationStep: (step: SidebarNavigationStep) => void;
};

export const useSidebarNavigationStore = create<SidebarNavigationStore>(
  (set) => ({
    navigationStep: SidebarNavigationStep.ANIME_LIST,
    setNavigationStep: (step: SidebarNavigationStep) =>
      set(() => ({ navigationStep: step }))
  })
);

export const tauriHandler = createTauriStore(
  'sidebarNavigation',
  useSidebarNavigationStore
);
