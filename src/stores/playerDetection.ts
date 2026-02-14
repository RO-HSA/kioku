import { AnimePlaybackDetection } from '@/services/backend/types';
import { create } from 'zustand';

type PlayerDetectionStore = {
  activeEpisode: AnimePlaybackDetection | null;
  lastClosedEpisode: AnimePlaybackDetection | null;
  activeMatchedAnimeId: number | null;
  suggestedAnimeIds: number[];
  setEpisodeDetected: (detection: AnimePlaybackDetection) => void;
  setEpisodeClosed: (detection: AnimePlaybackDetection) => void;
  setMatchingResult: (
    matchedAnimeId: number | null,
    suggestedAnimeIds: number[]
  ) => void;
  resolveActiveAnime: (animeId: number) => void;
};

export const usePlayerDetectionStore = create<PlayerDetectionStore>((set) => ({
  activeEpisode: null,
  lastClosedEpisode: null,
  activeMatchedAnimeId: null,
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
      suggestedAnimeIds: []
    })),
  setMatchingResult: (matchedAnimeId, suggestedAnimeIds) =>
    set(() => ({ activeMatchedAnimeId: matchedAnimeId, suggestedAnimeIds })),
  resolveActiveAnime: (animeId) =>
    set(() => ({ activeMatchedAnimeId: animeId, suggestedAnimeIds: [] }))
}));
