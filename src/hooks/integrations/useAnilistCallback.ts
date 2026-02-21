import { listen, UnlistenFn } from '@tauri-apps/api/event';
import { useEffect } from 'react';

import { AniListService } from '@/services/backend/AniList';
import { useAniListStore } from '@/stores/providers/anilist';
import { useProviderStore } from '@/stores/providers/provider';
import { Provider } from '@/types/List';

const useAnilistCallback = () => {
  const setIsAuthenticating = useAniListStore(
    (state) => state.setIsAuthenticating
  );
  const setIsAuthenticated = useAniListStore(
    (state) => state.setIsAuthenticated
  );
  const setIsReauthenticating = useAniListStore(
    (state) => state.setIsReauthenticating
  );
  const setId = useAniListStore((state) => state.setId);
  const setUsername = useAniListStore((state) => state.setUsername);
  const setProfilePictureUrl = useAniListStore(
    (state) => state.setProfilePictureUrl
  );
  const setStatistics = useAniListStore((state) => state.setStatistics);

  useEffect(() => {
    let unlistenSuccessfull: UnlistenFn | null = null;
    let unlistenFailed: UnlistenFn | null = null;

    const setupSuccessfullListener = async () => {
      unlistenSuccessfull = await listen('anilist-auth-callback', () => {
        setIsAuthenticated(true);
        setIsAuthenticating(false);
        setIsReauthenticating(false);

        AniListService.fetchUserInfo().then((userInfo) => {
          setId(userInfo.id);
          setUsername(userInfo.name);
          setProfilePictureUrl(userInfo.picture);
          setStatistics(userInfo.statistics);
        });

        const activeProvider = useProviderStore.getState().activeProvider;

        if (activeProvider === null) {
          useProviderStore.setState({
            activeProvider: Provider.ANILIST
          });
        }
      });
    };

    const setupFailedListener = async () => {
      unlistenFailed = await listen('anilist-auth-failed', () => {
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

export default useAnilistCallback;
