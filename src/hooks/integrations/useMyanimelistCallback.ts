import { listen, UnlistenFn } from '@tauri-apps/api/event';
import { useEffect } from 'react';

import { MyAnimeListService } from '@/services/backend/MyAnimeList';
import { useMyAnimeListStore } from '@/stores/providers/myanimelist';
import { useProviderStore } from '@/stores/providers/provider';
import { Provider } from '@/types/List';

const useMyanimelistCallback = () => {
  const setIsAuthenticating = useMyAnimeListStore(
    (state) => state.setIsAuthenticating
  );
  const setIsAuthenticated = useMyAnimeListStore(
    (state) => state.setIsAuthenticated
  );
  const setIsReauthenticating = useMyAnimeListStore(
    (state) => state.setIsReauthenticating
  );
  const setId = useMyAnimeListStore((state) => state.setId);
  const setStatistics = useMyAnimeListStore((state) => state.setStatistics);
  const setUsername = useMyAnimeListStore((state) => state.setUsername);
  const setProfilePictureUrl = useMyAnimeListStore(
    (state) => state.setProfilePictureUrl
  );

  useEffect(() => {
    let unlistenSuccessfull: UnlistenFn | null = null;
    let unlistenFailed: UnlistenFn | null = null;

    const setupSuccessfullListener = async () => {
      unlistenSuccessfull = await listen('myanimelist-auth-callback', () => {
        setIsAuthenticated(true);
        setIsAuthenticating(false);
        setIsReauthenticating(false);
        MyAnimeListService.fetchUserInfo().then((response) => {
          setId(response.id);
          setUsername(response.name);
          setProfilePictureUrl(response.picture);
          setStatistics(response.statistics);
          const activeProvider = useProviderStore.getState().activeProvider;

          if (activeProvider === null) {
            useProviderStore.setState({
              activeProvider: Provider.MY_ANIME_LIST
            });
          }
        });
      });
    };

    const setupFailedListener = async () => {
      unlistenFailed = await listen('myanimelist-auth-failed', () => {
        setIsAuthenticating(false);
        setIsReauthenticating(false);
      });
    };

    setupSuccessfullListener();
    setupFailedListener();

    return () => {
      if (unlistenSuccessfull) {
        unlistenSuccessfull();
      }

      if (unlistenFailed) {
        unlistenFailed();
      }

      setIsAuthenticating(false);
      setIsReauthenticating(false);
    };
  }, [
    setIsAuthenticated,
    setIsAuthenticating,
    setIsReauthenticating,
    setId,
    setProfilePictureUrl,
    setStatistics,
    setUsername
  ]);
};

export default useMyanimelistCallback;
