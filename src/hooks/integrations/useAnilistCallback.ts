import { listen, UnlistenFn } from '@tauri-apps/api/event';
import { useEffect } from 'react';

import { AniListService } from '@/services/backend/AniList';
import { useAniListStore } from '@/stores/providers/anilist';

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
  const setAnimeListData = useAniListStore((state) => state.setAnimeListData);

  useEffect(() => {
    let unlistenSuccessfull: UnlistenFn | null = null;
    let unlistenFailed: UnlistenFn | null = null;

    const setupSuccessfullListener = async () => {
      unlistenSuccessfull = await listen('anilist-auth-callback', () => {
        setIsAuthenticated(true);
        setIsAuthenticating(false);
        setIsReauthenticating(false);
        AniListService.synchronizeList().then((response) => {
          setAnimeListData(response);
        });
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
  }, [setIsAuthenticated, setIsAuthenticating, setAnimeListData]);
};

export default useAnilistCallback;
