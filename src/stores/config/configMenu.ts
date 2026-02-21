import { PlayerDetectionService } from '@/services/backend/PlayerDetection';
import { ConfigurationState } from '@/types/Configuration';
import { ConfigMenuStep } from '@/types/Navigation';
import { createTauriStore } from '@tauri-store/zustand';
import { create } from 'zustand';

type ConfigMenuStore = {
  isOpen: boolean;
  step: ConfigMenuStep;
  selectedTab: number;
  configuration: ConfigurationState;
  openConfigMenu: () => void;
  closeConfigMenu: () => void;
  setStep: (step: ConfigMenuStep) => void;
  setSelectedTab: (tab: number) => void;
  setConfiguration: (configuration: ConfigurationState) => void;
};

const defaultConfiguration: ConfigurationState = {
  detection: {
    playerDetectionEnabled: false,
    enabledPlayers: []
  },
  application: {
    enableAutoStartup: false,
    startMinimized: false
  }
};

export const useConfigMenuStore = create<ConfigMenuStore>((set) => ({
  isOpen: false,
  step: ConfigMenuStep.INTEGRATIONS,
  selectedTab: 0,
  configuration: defaultConfiguration,
  openConfigMenu: () => set(() => ({ isOpen: true })),
  closeConfigMenu: () => set(() => ({ isOpen: false })),
  setStep: (step: ConfigMenuStep) => set(() => ({ step })),
  setSelectedTab: (selectedTab: number) => set(() => ({ selectedTab })),
  setConfiguration: (configuration: ConfigurationState) =>
    set(() => {
      PlayerDetectionService.configurePlaybackObserver({
        enabled: configuration.detection.playerDetectionEnabled,
        players: configuration.detection.enabledPlayers
      });
      return { configuration };
    })
}));

export const tauriHandler = createTauriStore('configMenu', useConfigMenuStore, {
  autoStart: true,
  saveOnChange: true,
  filterKeys: ['configuration'],
  filterKeysStrategy: 'pick'
});
