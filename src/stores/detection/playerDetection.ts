import { AnimePlaybackDetection } from '@/services/backend/types';
import { Provider } from '@/types/List';
import { create } from 'zustand';

type PlayerDetectionStore = {
  activeEpisode: AnimePlaybackDetection | null;
  lastClosedEpisode: AnimePlaybackDetection | null;
  activeMatchedAnimeId: number | null;
  activeMatchedProvider: Provider | null;
  suggestedAnimeIds: number[];
  setEpisodeDetected: (detection: AnimePlaybackDetection) => void;
  setEpisodeClosed: (detection: AnimePlaybackDetection) => void;
  setMatchingResult: (
    provider: Provider,
    matchedAnimeId: number | null,
    suggestedAnimeIds: number[]
  ) => void;
  resolveActiveAnime: (provider: Provider, animeId: number) => void;
};

export const usePlayerDetectionStore = create<PlayerDetectionStore>((set) => ({
  activeEpisode: null,
  lastClosedEpisode: null,
  activeMatchedAnimeId: null,
  activeMatchedProvider: null,
  suggestedAnimeIds: [],
  setEpisodeDetected: (detection) =>
    set(() => ({
      activeEpisode: detection,
      lastClosedEpisode: null
    })),
  setEpisodeClosed: (detection) =>
    set(() => ({
      activeEpisode: null,
      lastClosedEpisode: detection,
      activeMatchedAnimeId: null,
      activeMatchedProvider: null,
      suggestedAnimeIds: []
    })),
  setMatchingResult: (provider, matchedAnimeId, suggestedAnimeIds) =>
    set(() => ({
      activeMatchedAnimeId: matchedAnimeId,
      activeMatchedProvider: provider,
      suggestedAnimeIds
    })),
  resolveActiveAnime: (provider, animeId) =>
    set(() => ({
      activeMatchedAnimeId: animeId,
      activeMatchedProvider: provider,
      suggestedAnimeIds: []
    }))
}));
