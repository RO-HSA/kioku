import { MyAnimeListService } from '@/services/backend/MyAnimeList';
import { useMyAnimeListStore } from '@/stores/config/providers/myanimelist';
import { listen, UnlistenFn } from '@tauri-apps/api/event';
import { useEffect } from 'react';

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
  const setListData = useMyAnimeListStore((state) => state.setListData);

  useEffect(() => {
    let unlistenSuccessfull: UnlistenFn | null = null;
    let unlistenFailed: UnlistenFn | null = null;

    const setupSuccessfullListener = async () => {
      unlistenSuccessfull = await listen('myanimelist-auth-callback', () => {
        setIsAuthenticated(true);
        setIsAuthenticating(false);
        setIsReauthenticating(false);
        MyAnimeListService.synchronizeList().then((response) => {
          setListData(response);
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
  }, [setIsAuthenticated, setIsAuthenticating, setListData]);
};

export default useMyanimelistCallback;
