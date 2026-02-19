import { useEffect } from 'react';

import {
  tauriHandler as myAnimeListTauriHandler,
  useMyAnimeListStore
} from '@/stores/providers/myanimelist';
import {
  tauriHandler as providerTauriHandler,
  useProviderStore
} from '@/stores/providers/provider';
import { Provider } from '@/types/List';

const useProviderMigration = () => {
  useEffect(() => {
    let cancelled = false;

    const migrateToProviderMyAnimeList = async () => {
      await Promise.all([
        providerTauriHandler.start(),
        myAnimeListTauriHandler.start()
      ]);

      if (cancelled) {
        return;
      }

      const { activeProvider } = useProviderStore.getState();
      const { isAuthenticated } = useMyAnimeListStore.getState();

      if (activeProvider === null && isAuthenticated) {
        useProviderStore.getState().setActiveProvider(Provider.MY_ANIME_LIST);
      }
    };

    void migrateToProviderMyAnimeList().catch(() => undefined);

    return () => {
      cancelled = true;
    };
  }, []);
};

export default useProviderMigration;
