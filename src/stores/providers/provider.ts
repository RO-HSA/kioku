import { createTauriStore } from '@tauri-store/zustand';
import { create } from 'zustand';

import { Provider } from '@/types/List';

type ProviderStore = {
  activeProvider: Provider | null;
  setActiveProvider: (provider: Provider | null) => void;
};

export const useProviderStore = create<ProviderStore>((set) => ({
  activeProvider: null,
  setActiveProvider: (provider) => set(() => ({ activeProvider: provider }))
}));

export const tauriHandler = createTauriStore('provider', useProviderStore, {
  autoStart: true,
  saveOnChange: true
});
