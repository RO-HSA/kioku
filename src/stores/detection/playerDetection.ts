import { AnimePlaybackDetection } from '@/services/backend/types';
import { IAnimeList } from '@/types/AnimeList';
import { Provider } from '@/types/List';
import { create } from 'zustand';

type PlayerDetectionStore = {
  activeEpisode: AnimePlaybackDetection | null;
  lastClosedEpisode: AnimePlaybackDetection | null;
  activeMatchedAnimeId: number | null;
  activeMatchedProvider: Provider | null;
  suggestedAnimeIds: number[];
  remoteAnimeCandidates: IAnimeList[];
  setEpisodeDetected: (detection: AnimePlaybackDetection) => void;
  setEpisodeClosed: (detection: AnimePlaybackDetection) => void;
  setMatchingResult: (
    provider: Provider,
    matchedAnimeId: number | null,
    suggestedAnimeIds: number[],
    remoteAnimeCandidates?: IAnimeList[]
  ) => void;
  resolveActiveAnime: (provider: Provider, animeId: number) => void;
};

export const usePlayerDetectionStore = create<PlayerDetectionStore>((set) => ({
  activeEpisode: null,
  lastClosedEpisode: null,
  activeMatchedAnimeId: null,
  activeMatchedProvider: null,
  suggestedAnimeIds: [],
  remoteAnimeCandidates: [],
  setEpisodeDetected: (detection) =>
    set(() => ({
      activeEpisode: detection,
      lastClosedEpisode: null,
      activeMatchedAnimeId: null,
      activeMatchedProvider: null,
      suggestedAnimeIds: [],
      remoteAnimeCandidates: []
    })),
  setEpisodeClosed: (detection) =>
    set(() => ({
      activeEpisode: null,
      lastClosedEpisode: detection,
      activeMatchedAnimeId: null,
      activeMatchedProvider: null,
      suggestedAnimeIds: [],
      remoteAnimeCandidates: []
    })),
  setMatchingResult: (
    provider,
    matchedAnimeId,
    suggestedAnimeIds,
    remoteAnimeCandidates = []
  ) =>
    set(() => ({
      activeMatchedAnimeId: matchedAnimeId,
      activeMatchedProvider: provider,
      suggestedAnimeIds,
      remoteAnimeCandidates
    })),
  resolveActiveAnime: (provider, animeId) =>
    set(() => ({
      activeMatchedAnimeId: animeId,
      activeMatchedProvider: provider,
      suggestedAnimeIds: []
    }))
}));
