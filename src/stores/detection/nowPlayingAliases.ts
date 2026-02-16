import { createTauriStore } from '@tauri-store/zustand';
import { create } from 'zustand';

import { Provider } from '@/types/List';

type AliasesByAnimeId = Record<string, string[]>;
type AliasesByProvider = Partial<Record<Provider, AliasesByAnimeId>>;
type PersistedAliases = AliasesByProvider | AliasesByAnimeId;

const hasProviderBuckets = (
  aliases: PersistedAliases
): aliases is AliasesByProvider => {
  return (
    Object.prototype.hasOwnProperty.call(aliases, Provider.MY_ANIME_LIST) ||
    Object.prototype.hasOwnProperty.call(aliases, Provider.ANILIST)
  );
};

const toProviderAliases = (aliases: PersistedAliases): AliasesByProvider => {
  if (hasProviderBuckets(aliases)) {
    return aliases;
  }

  if (Object.keys(aliases).length === 0) {
    return {};
  }

  return {
    [Provider.MY_ANIME_LIST]: aliases as AliasesByAnimeId
  };
};

const getAliasesForProvider = (
  aliases: PersistedAliases,
  provider: Provider
): AliasesByAnimeId => {
  if (hasProviderBuckets(aliases)) {
    return aliases[provider] || {};
  }

  if (provider === Provider.MY_ANIME_LIST) {
    return aliases as AliasesByAnimeId;
  }

  return {};
};

type NowPlayingAliasesStore = {
  aliasesByAnimeId: PersistedAliases;
  getAliasesByProvider: (provider: Provider) => AliasesByAnimeId;
  addAlias: (provider: Provider, animeId: number, title: string) => void;
};

export const useNowPlayingAliasesStore = create<NowPlayingAliasesStore>(
  (set, get) => ({
    aliasesByAnimeId: {},
    getAliasesByProvider: (provider) =>
      getAliasesForProvider(get().aliasesByAnimeId, provider),
    addAlias: (provider: Provider, animeId: number, title: string) =>
      set((state) => {
        const trimmedTitle = title.trim();
        if (!trimmedTitle) {
          return {};
        }

        const aliasesByProvider = toProviderAliases(state.aliasesByAnimeId);
        const providerAliases = aliasesByProvider[provider] || {};
        const key = animeId.toString();
        const existingAliases = providerAliases[key] || [];
        const alreadyExists = existingAliases.some(
          (alias) => alias.toLowerCase().trim() === trimmedTitle.toLowerCase()
        );

        if (alreadyExists) {
          return {};
        }

        return {
          aliasesByAnimeId: {
            ...aliasesByProvider,
            [provider]: {
              ...providerAliases,
              [key]: [...existingAliases, trimmedTitle]
            }
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
