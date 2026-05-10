import { useNowPlayingAliasesStore } from '@/stores/detection/nowPlayingAliases';
import { usePlayerDetectionStore } from '@/stores/detection/playerDetection';
import { useAniListStore } from '@/stores/providers/anilist';
import { useMyAnimeListStore } from '@/stores/providers/myanimelist';
import { useProviderStore } from '@/stores/providers/provider';
import { Provider } from '@/types/List';
import { calculatePlaybackMatches, searchRemoteAnimeCandidates } from './utils';

export const refreshActivePlaybackMatch = async (
  provider = useProviderStore.getState().activeProvider
) => {
  if (!provider) {
    return;
  }

  const { activeEpisode, setMatchingResult } =
    usePlayerDetectionStore.getState();
  if (!activeEpisode) {
    return;
  }

  const animeListData =
    provider === Provider.MY_ANIME_LIST
      ? useMyAnimeListStore.getState().animeListData
      : provider === Provider.ANILIST
        ? useAniListStore.getState().animeListData
        : null;

  if (!animeListData) {
    return;
  }

  const remoteAnimeCandidates = await searchRemoteAnimeCandidates(
    provider,
    activeEpisode.animeTitle
  );

  if (
    useProviderStore.getState().activeProvider !== provider ||
    usePlayerDetectionStore.getState().activeEpisode !== activeEpisode
  ) {
    return;
  }

  calculatePlaybackMatches({
    provider,
    animeListData,
    animeTitle: activeEpisode.animeTitle,
    episodeNumber: activeEpisode.episode,
    aliasesByAnimeId: useNowPlayingAliasesStore
      .getState()
      .getAliasesByProvider(provider),
    remoteAnimeCandidates,
    shouldNotify: false,
    setMatchingResult
  });
};
