import { PlayerDetectionService } from '@/services/backend/PlayerDetection';
import { NotificationService } from '@/services/Notification';
import { useNowPlayingAliasesStore } from '@/stores/detection/nowPlayingAliases';
import { usePlayerDetectionStore } from '@/stores/detection/playerDetection';
import { useAniListStore } from '@/stores/providers/anilist';
import { useMyAnimeListStore } from '@/stores/providers/myanimelist';
import { useProviderStore } from '@/stores/providers/provider';
import { IAnimeList } from '@/types/AnimeList';
import { Provider } from '@/types/List';
import { getTodayAsYmd } from '@/utils/date';
import { UnlistenFn } from '@tauri-apps/api/event';
import { useEffect } from 'react';
import {
  calculatePlaybackMatches,
  flattenAnimeListData,
  resolveNextStatusFromDetectedEpisode
} from './utils';

const notification = new NotificationService();

const usePlaybackObserverEvents = () => {
  const setEpisodeDetected = usePlayerDetectionStore(
    (state) => state.setEpisodeDetected
  );
  const setEpisodeClosed = usePlayerDetectionStore(
    (state) => state.setEpisodeClosed
  );
  const setMatchingResult = usePlayerDetectionStore(
    (state) => state.setMatchingResult
  );

  useEffect(() => {
    let unlistenDetected: UnlistenFn | null = null;
    let unlistenClosed: UnlistenFn | null = null;

    const setupListeners = async () => {
      unlistenDetected = await PlayerDetectionService.listenEpisodeDetected(
        (detection) => {
          const myAnimeListAnimeData =
            useMyAnimeListStore.getState().animeListData;
          const aniListAnimeData = useAniListStore.getState().animeListData;

          switch (useProviderStore.getState().activeProvider) {
            case Provider.MY_ANIME_LIST:
              if (!myAnimeListAnimeData) {
                notification.sendNotification({
                  title: 'No anime list data',
                  body: 'Please synchronize your anime list to enable playback detection.'
                });
                return;
              }

              calculatePlaybackMatches({
                animeListData: myAnimeListAnimeData,
                animeTitle: detection.animeTitle,
                episodeNumber: detection.episode,
                aliasesByAnimeId: useNowPlayingAliasesStore
                  .getState()
                  .getAliasesByProvider(Provider.MY_ANIME_LIST),
                setMatchingResult
              });
              break;
            case Provider.ANILIST:
              if (!aniListAnimeData) {
                notification.sendNotification({
                  title: 'No anime list data',
                  body: 'Please synchronize your anime list to enable playback detection.'
                });
                return;
              }

              calculatePlaybackMatches({
                animeListData: aniListAnimeData,
                animeTitle: detection.animeTitle,
                episodeNumber: detection.episode,
                aliasesByAnimeId: useNowPlayingAliasesStore
                  .getState()
                  .getAliasesByProvider(Provider.ANILIST),
                setMatchingResult
              });
              break;
            default:
              notification.sendNotification({
                title: 'No provider selected',
                body: 'Please select an anime list provider and synchronize your anime list to enable playback detection.'
              });
              return;
          }

          setEpisodeDetected(detection);
        }
      );

      unlistenClosed = await PlayerDetectionService.listenEpisodeClosed(
        (detection) => {
          const { activeMatchedAnimeId } = usePlayerDetectionStore.getState();

          setEpisodeClosed(detection);

          if (!activeMatchedAnimeId || detection.episode === null) {
            return;
          }

          let aggregatedData: IAnimeList[] = [];

          const myAnimeListAnimeData =
            useMyAnimeListStore.getState().animeListData;
          const aniListAnimeData = useAniListStore.getState().animeListData;

          switch (useProviderStore.getState().activeProvider) {
            case Provider.MY_ANIME_LIST:
              if (myAnimeListAnimeData) {
                aggregatedData = flattenAnimeListData(myAnimeListAnimeData);
              }
              break;
            case Provider.ANILIST:
              if (aniListAnimeData) {
                aggregatedData = flattenAnimeListData(aniListAnimeData);
              }
              break;
            default:
              break;
          }

          const anime = aggregatedData.find(
            (item) => item.id === activeMatchedAnimeId
          );

          if (!anime) {
            return;
          }

          const rawNextEpisode = Math.max(
            anime.userEpisodesWatched,
            detection.episode
          );
          const nextEpisode =
            anime.totalEpisodes > 0
              ? Math.min(rawNextEpisode, anime.totalEpisodes)
              : rawNextEpisode;

          if (nextEpisode <= anime.userEpisodesWatched) {
            return;
          }

          const nextStatus = resolveNextStatusFromDetectedEpisode(
            anime,
            nextEpisode
          );

          const updatePayload: Partial<IAnimeList> = {
            userEpisodesWatched: nextEpisode
          };

          if (nextStatus !== anime.userStatus) {
            updatePayload.userStatus = nextStatus;
          }

          if (
            nextStatus === 'completed' &&
            anime.userStatus !== 'completed' &&
            !anime.userFinishDate
          ) {
            updatePayload.userFinishDate = getTodayAsYmd();
          }

          useMyAnimeListStore
            .getState()
            .updateAnimeList(anime.id, anime.userStatus, updatePayload);
        }
      );
    };

    setupListeners();

    return () => {
      if (unlistenDetected) {
        unlistenDetected();
      }

      if (unlistenClosed) {
        unlistenClosed();
      }
    };
  }, [setEpisodeClosed, setEpisodeDetected, setMatchingResult]);
};

export default usePlaybackObserverEvents;
