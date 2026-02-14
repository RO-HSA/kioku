import { AnimePlaybackDetection } from '@/services/backend/types';
import { create } from 'zustand';

type PlayerDetectionStore = {
  activeEpisode: AnimePlaybackDetection | null;
  lastClosedEpisode: AnimePlaybackDetection | null;
  setEpisodeDetected: (detection: AnimePlaybackDetection) => void;
  setEpisodeClosed: (detection: AnimePlaybackDetection) => void;
};

export const usePlayerDetectionStore = create<PlayerDetectionStore>((set) => ({
  activeEpisode: null,
  lastClosedEpisode: null,
  setEpisodeDetected: (detection) => set(() => ({ activeEpisode: detection })),
  setEpisodeClosed: (detection) =>
    set(() => ({ activeEpisode: null, lastClosedEpisode: detection }))
}));
