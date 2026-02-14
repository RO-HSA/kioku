import { UnlistenFn } from '@tauri-apps/api/event';
import { useEffect } from 'react';

import {
  findExactAnimeMatch,
  findSuggestedAnimeMatches
} from '@/components/NowPlaying/utils';
import { PlayerDetectionService } from '@/services/backend/PlayerDetection';
import { SynchronizedAnimeList } from '@/services/backend/types';
import { NotificationService } from '@/services/Notification';
import { useNowPlayingAliasesStore } from '@/stores/nowPlayingAliases';
import { usePlayerDetectionStore } from '@/stores/playerDetection';
import { useMyAnimeListStore } from '@/stores/providers/myanimelist';
import { AnimeListUserStatus, IAnimeList } from '@/types/AnimeList';
import { getTodayAsYmd } from '@/utils/date';

const notification = new NotificationService();

const flattenAnimeListData = (
  animeListData: SynchronizedAnimeList | null
): IAnimeList[] => {
  if (!animeListData) {
    return [];
  }

  return [
    ...animeListData.watching,
    ...animeListData.completed,
    ...animeListData.onHold,
    ...animeListData.dropped,
    ...animeListData.planToWatch
  ];
};

const resolveNextStatusFromDetectedEpisode = (
  anime: IAnimeList,
  nextEpisode: number
): AnimeListUserStatus => {
  if (anime.totalEpisodes > 0 && nextEpisode >= anime.totalEpisodes) {
    return 'completed';
  }

  if (
    anime.userStatus === 'planToWatch' ||
    anime.userStatus === 'onHold' ||
    anime.userStatus === 'dropped'
  ) {
    return 'watching';
  }

  return anime.userStatus;
};

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
          const animeListData = useMyAnimeListStore.getState().animeListData;
          const aliasesByAnimeId =
            useNowPlayingAliasesStore.getState().aliasesByAnimeId;
          const aggregatedData = flattenAnimeListData(animeListData);

          const exactMatch = findExactAnimeMatch(
            aggregatedData,
            detection.animeTitle,
            aliasesByAnimeId
          );

          if (exactMatch) {
            notification.sendNotification({
              title: 'Now Playing',
              body: `${exactMatch.title}\nEpisode ${detection.episode ?? '?'}`
            });
            setMatchingResult(exactMatch.id, []);
          } else {
            notification.sendNotification({
              title: 'Media not recognized',
              body: `${detection.animeTitle}\nEpisode ${detection.episode ?? '?'}\nTry selecting the correct anime from the suggestions on Now Playing menu.`
            });

            const suggestions = findSuggestedAnimeMatches(
              aggregatedData,
              detection.animeTitle,
              aliasesByAnimeId
            );

            setMatchingResult(
              null,
              suggestions.map((anime) => anime.id)
            );
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

          const animeListData = useMyAnimeListStore.getState().animeListData;
          const aggregatedData = flattenAnimeListData(animeListData);
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
