import { ConfigMenuStep } from '@/types/Navigation';
import { createTauriStore } from '@tauri-store/zustand';
import { create } from 'zustand';

type ConfigMenuStore = {
  isOpen: boolean;
  step: ConfigMenuStep;
  openConfigMenu: () => void;
  closeConfigMenu: () => void;
  setStep: (step: ConfigMenuStep) => void;
};

export const useConfigMenuStore = create<ConfigMenuStore>((set) => ({
  isOpen: false,
  step: ConfigMenuStep.INTEGRATIONS,
  openConfigMenu: () => set(() => ({ isOpen: true })),
  closeConfigMenu: () => set(() => ({ isOpen: false })),
  setStep: (step: ConfigMenuStep) => set(() => ({ step }))
}));

export const tauriHandler = createTauriStore('configMenu', useConfigMenuStore);
