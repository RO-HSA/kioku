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

          if (response.anime_statistics) {
            setStatistics({
              numItems: response.anime_statistics.num_items,
              numItemsCompleted: response.anime_statistics.num_items_completed,
              numItemsDropped: response.anime_statistics.num_items_dropped,
              numItemsOnHold: response.anime_statistics.num_items_on_hold,
              numItemsPlanToWatch:
                response.anime_statistics.num_items_plan_to_watch,
              numItemsWatching: response.anime_statistics.num_items_watching,
              numDays: response.anime_statistics.num_days,
              numDaysCompleted: response.anime_statistics.num_days_completed,
              numDaysDropped: response.anime_statistics.num_days_dropped,
              numDaysOnHold: response.anime_statistics.num_days_on_hold,
              numDaysWatching: response.anime_statistics.num_days_watching,
              meanScore: response.anime_statistics.mean_score,
              numEpisodes: response.anime_statistics.num_episodes,
              numTimesRewatched: response.anime_statistics.num_times_rewatched,
              numDaysWatched: response.anime_statistics?.num_days_watched
            });
          }
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
