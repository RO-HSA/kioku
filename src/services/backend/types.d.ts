import {
  AnimeListUserStatus,
  IAnimeList,
  IAnimeUserList
} from '@/types/AnimeList';
import { Provider } from '@/types/List';

export type SynchronizedAnimeList = Record<AnimeListUserStatus, IAnimeList[]>;

export interface AnimeListUpdateRequest extends Partial<IAnimeUserList> {
  providerId: Provider;
  entryId?: number;
  mediaId?: number;
}

export type SupportedPlayer = 'mpv' | 'mpc-hc' | 'mpc-be';

export interface DetectPlayingAnimeRequest {
  players?: SupportedPlayer[];
}

export interface AnimePlaybackDetection {
  player: SupportedPlayer;
  animeTitle: string;
  episode: number | null;
}

export interface ConfigurePlaybackObserverRequest {
  enabled?: boolean;
  players?: SupportedPlayer[];
  pollIntervalMs?: number;
}

export interface PlaybackObserverSnapshot {
  active: AnimePlaybackDetection | null;
  lastObserved: AnimePlaybackDetection | null;
  observedProcessId: number | null;
  observedPlayer: SupportedPlayer | null;
  selectedPlayers: SupportedPlayer[];
  enabled: boolean;
  pollIntervalMs: number;
  lastError: string | null;
}

export interface DiscordPresenceButton {
  label: string;
  url: string;
}

export interface DiscordPresenceRequest {
  details?: string;
  state?: string;
  type?: number;
  statusDisplayType?: number;
  largeImage?: string;
  largeText?: string;
  largeUrl?: string;
  smallImage?: string;
  smallText?: string;
  startTimestamp?: number;
  endTimestamp?: number;
  buttons?: DiscordPresenceButton[];
}
