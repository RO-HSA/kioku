import { useEffect, useMemo, useRef } from 'react';

import { DiscordRpcService } from '@/services/backend/DiscordRpc';
import { DiscordPresenceRequest } from '@/services/backend/types';
import {
  defaultConfiguration,
  useConfigMenuStore
} from '@/stores/config/configMenu';
import { usePlayerDetectionStore } from '@/stores/detection/playerDetection';
import { useAniListStore } from '@/stores/providers/anilist';
import { useMyAnimeListStore } from '@/stores/providers/myanimelist';
import { useProviderStore } from '@/stores/providers/provider';
import { Provider } from '@/types/List';
import { buildEntityUrl } from '@/utils/url';
import { flattenAnimeListData } from './utils';

const useDiscordRichPresence = () => {
  const activeEpisode = usePlayerDetectionStore((state) => state.activeEpisode);
  const activeMatchedAnimeId = usePlayerDetectionStore(
    (state) => state.activeMatchedAnimeId
  );

  const activeProvider = useProviderStore((state) => state.activeProvider);
  const myAnimeListAnimeData = useMyAnimeListStore(
    (state) => state.animeListData
  );
  const aniListAnimeData = useAniListStore((state) => state.animeListData);

  const aggregatedData = useMemo(() => {
    switch (activeProvider) {
      case Provider.MY_ANIME_LIST:
        return flattenAnimeListData(myAnimeListAnimeData);
      case Provider.ANILIST:
        return flattenAnimeListData(aniListAnimeData);
      default:
        return [];
    }
  }, [activeProvider, aniListAnimeData, myAnimeListAnimeData]);

  const matchedAnime = useMemo(() => {
    if (!activeMatchedAnimeId) {
      return undefined;
    }

    return aggregatedData.find((anime) => anime.id === activeMatchedAnimeId);
  }, [activeMatchedAnimeId, aggregatedData]);

  const currentSessionRef = useRef<{
    episodeKey: string | null;
    startTimestamp: number;
  }>({
    episodeKey: null,
    startTimestamp: 0
  });
  const lastPayloadRef = useRef<string>('clear');

  useEffect(() => {
    if (!activeEpisode) {
      currentSessionRef.current = { episodeKey: null, startTimestamp: 0 };

      if (lastPayloadRef.current === 'clear') {
        return;
      }

      lastPayloadRef.current = 'clear';
      DiscordRpcService.clearPresence().catch(() => undefined);
      return;
    }

    const { configuration } = useConfigMenuStore.getState();

    const enableRichPresence =
      configuration?.sharing?.enableRichPresence ??
      defaultConfiguration.sharing.enableRichPresence;

    if (!enableRichPresence) return;

    const displayUsernameInPresence =
      configuration?.sharing?.displayUsernameInPresence ??
      defaultConfiguration.sharing.displayUsernameInPresence;

    const displayTimeElapsedInPresence =
      configuration?.sharing?.displayTimeElapsedInPresence ??
      defaultConfiguration.sharing.displayTimeElapsedInPresence;

    const preferAnimeTitleInPresence =
      configuration?.sharing?.preferAnimeTitleInPresence ??
      defaultConfiguration.sharing.preferAnimeTitleInPresence;

    const episodeKey = [
      activeEpisode.player,
      activeEpisode.animeTitle,
      String(activeEpisode.episode ?? '')
    ].join('|');

    if (currentSessionRef.current.episodeKey !== episodeKey) {
      currentSessionRef.current = {
        episodeKey,
        startTimestamp: Math.floor(Date.now() / 1000)
      };
    }

    const request: DiscordPresenceRequest = {
      details: matchedAnime?.title ?? activeEpisode.animeTitle,
      state: `Episode ${activeEpisode.episode ?? '?'}`,
      startTimestamp: currentSessionRef.current.startTimestamp,
      type: 3,
      statusDisplayType: 0,
      buttons: [
        {
          label: 'Download Kioku',
          url: 'https://github.com/RO-HSA/kioku/releases'
        }
      ]
    };

    if (!displayTimeElapsedInPresence) {
      request.startTimestamp = undefined;
      request.endTimestamp = currentSessionRef.current.startTimestamp;
    }

    let providerText = '';
    let username = '';

    switch (activeProvider) {
      case Provider.MY_ANIME_LIST:
        providerText = 'MyAnimeList';
        username = useMyAnimeListStore.getState().username ?? '';
        break;
      case Provider.ANILIST:
        providerText = 'AniList';
        username = useAniListStore.getState().username ?? '';
        break;
      default:
        break;
    }

    if (matchedAnime && activeProvider) {
      request.smallImage = activeProvider.toLowerCase();
      request.smallText = displayUsernameInPresence
        ? `${username} - ${providerText}`
        : providerText;

      if (matchedAnime.imageUrl) {
        request.largeImage = matchedAnime.imageUrl;
        request.largeText = matchedAnime.title;
        request.largeUrl = buildEntityUrl(
          activeProvider,
          'anime',
          matchedAnime.id
        );
      }

      if (preferAnimeTitleInPresence) {
        request.statusDisplayType = 2;
      }
    }

    const payloadSignature = JSON.stringify(request);
    if (payloadSignature === lastPayloadRef.current) {
      return;
    }

    lastPayloadRef.current = payloadSignature;
    DiscordRpcService.setPresence(request).catch(() => {
      lastPayloadRef.current = '';
    });
  }, [activeEpisode, activeProvider, matchedAnime]);

  useEffect(() => {
    return () => {
      DiscordRpcService.clearPresence().catch(() => undefined);
    };
  }, []);
};

export default useDiscordRichPresence;
