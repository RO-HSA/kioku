import { createTauriStore } from '@tauri-store/zustand';
import { create } from 'zustand';

type AliasesByAnimeId = Record<string, string[]>;

type NowPlayingAliasesStore = {
  aliasesByAnimeId: AliasesByAnimeId;
  addAlias: (animeId: number, title: string) => void;
};

export const useNowPlayingAliasesStore = create<NowPlayingAliasesStore>(
  (set) => ({
    aliasesByAnimeId: {},
    addAlias: (animeId: number, title: string) =>
      set((state) => {
        const trimmedTitle = title.trim();
        if (!trimmedTitle) {
          return {};
        }

        const key = animeId.toString();
        const existingAliases = state.aliasesByAnimeId[key] || [];
        const alreadyExists = existingAliases.some(
          (alias) => alias.toLowerCase().trim() === trimmedTitle.toLowerCase()
        );

        if (alreadyExists) {
          return {};
        }

        return {
          aliasesByAnimeId: {
            ...state.aliasesByAnimeId,
            [key]: [...existingAliases, trimmedTitle]
          }
        };
      })
  })
);

export const tauriHandler = createTauriStore(
  'nowPlayingAliases',
  useNowPlayingAliasesStore,
  {
    autoStart: true,
    saveOnChange: true,
    filterKeys: ['aliasesByAnimeId'],
    filterKeysStrategy: 'pick'
  }
);
