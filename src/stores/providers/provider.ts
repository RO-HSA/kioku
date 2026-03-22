import { createTauriStore } from '@tauri-store/zustand';
import { create } from 'zustand';

import { ListType, Provider } from '@/types/List';

type ProviderStore = {
  activeProvider: Provider | null;
  selectedListType: ListType;
  setActiveProvider: (provider: Provider | null) => void;
  setSelectedListType: (listType: ListType) => void;
};

export const useProviderStore = create<ProviderStore>((set) => ({
  activeProvider: null,
  selectedListType: 'anime',
  setActiveProvider: (provider) => set(() => ({ activeProvider: provider })),
  setSelectedListType: (listType) => set(() => ({ selectedListType: listType }))
}));

export const tauriHandler = createTauriStore('provider', useProviderStore, {
  autoStart: true,
  saveOnChange: true
});
