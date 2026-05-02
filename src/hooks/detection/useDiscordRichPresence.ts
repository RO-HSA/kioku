import { useEffect, useMemo, useRef, useState } from 'react';

import { DiscordRpcService } from '@/services/backend/DiscordRpc';
import { DiscordPresenceRequest } from '@/services/backend/types';
import { useConfigMenuStore } from '@/stores/config/configMenu';
import { usePlayerDetectionStore } from '@/stores/detection/playerDetection';
import { useAniListStore } from '@/stores/providers/anilist';
import { useMyAnimeListStore } from '@/stores/providers/myanimelist';
import { useProviderStore } from '@/stores/providers/provider';
import { Provider } from '@/types/List';
import { buildEntityUrl } from '@/utils/url';
import { flattenAnimeListData } from './utils';

const DISCORD_PRESENCE_REFRESH_MS = 30_000;

const useDiscordRichPresence = () => {
  const activeEpisode = usePlayerDetectionStore((state) => state.activeEpisode);
  const activeMatchedAnimeId = usePlayerDetectionStore(
    (state) => state.activeMatchedAnimeId
  );
  const activeMatchedProvider = usePlayerDetectionStore(
    (state) => state.activeMatchedProvider
  );

  const activeProvider = useProviderStore((state) => state.activeProvider);
  const myAnimeListAnimeData = useMyAnimeListStore(
    (state) => state.animeListData
  );
  const aniListAnimeData = useAniListStore((state) => state.animeListData);
  const enableRichPresence = useConfigMenuStore(
    (state) => state.configuration.sharing.enableRichPresence
  );
  const displayUsernameInPresence = useConfigMenuStore(
    (state) => state.configuration.sharing.displayUsernameInPresence
  );
  const displayTimeElapsedInPresence = useConfigMenuStore(
    (state) => state.configuration.sharing.displayTimeElapsedInPresence
  );
  const preferAnimeTitleInPresence = useConfigMenuStore(
    (state) => state.configuration.sharing.preferAnimeTitleInPresence
  );

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
    if (!activeMatchedAnimeId || activeMatchedProvider !== activeProvider) {
      return undefined;
    }

    return aggregatedData.find((anime) => anime.id === activeMatchedAnimeId);
  }, [
    activeMatchedAnimeId,
    activeMatchedProvider,
    activeProvider,
    aggregatedData
  ]);

  const [currentSession, setCurrentSession] = useState<{
    episodeKey: string | null;
    startTimestamp: number;
  }>({
    episodeKey: null,
    startTimestamp: 0
  });
  const lastPayloadRef = useRef<string>('clear');

  useEffect(() => {
    if (!activeEpisode) {
      setCurrentSession((current) =>
        current.episodeKey === null && current.startTimestamp === 0
          ? current
          : { episodeKey: null, startTimestamp: 0 }
      );
      return;
    }

    const episodeKey = [
      activeEpisode.player,
      activeEpisode.animeTitle,
      String(activeEpisode.episode ?? '')
    ].join('|');

    setCurrentSession((current) =>
      current.episodeKey === episodeKey
        ? current
        : {
            episodeKey,
            startTimestamp: Math.floor(Date.now() / 1000)
          }
    );
  }, [activeEpisode]);

  const presenceRequest = useMemo<DiscordPresenceRequest | null>(() => {
    if (!activeEpisode || !enableRichPresence) {
      return null;
    }

    const request: DiscordPresenceRequest = {
      details: matchedAnime?.title ?? activeEpisode.animeTitle,
      state: `Episode ${activeEpisode.episode ?? '?'}`,
      startTimestamp: currentSession.startTimestamp,
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
      request.endTimestamp = currentSession.startTimestamp;
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

    return request;
  }, [
    activeEpisode,
    activeProvider,
    displayTimeElapsedInPresence,
    displayUsernameInPresence,
    enableRichPresence,
    matchedAnime,
    preferAnimeTitleInPresence,
    currentSession.startTimestamp
  ]);

  useEffect(() => {
    if (!presenceRequest) {
      if (lastPayloadRef.current === 'clear') {
        return;
      }

      lastPayloadRef.current = 'clear';
      DiscordRpcService.clearPresence().catch(() => {
        lastPayloadRef.current = '';
      });
      return;
    }

    const payloadSignature = JSON.stringify(presenceRequest);

    const pushPresence = () => {
      lastPayloadRef.current = payloadSignature;
      DiscordRpcService.setPresence(presenceRequest).catch(() => {
        lastPayloadRef.current = '';
      });
    };

    if (payloadSignature !== lastPayloadRef.current) {
      pushPresence();
    }

    const refreshInterval = window.setInterval(() => {
      pushPresence();
    }, DISCORD_PRESENCE_REFRESH_MS);

    return () => {
      window.clearInterval(refreshInterval);
    };
  }, [presenceRequest]);

  useEffect(() => {
    return () => {
      DiscordRpcService.clearPresence().catch(() => undefined);
    };
  }, []);
};

export default useDiscordRichPresence;
