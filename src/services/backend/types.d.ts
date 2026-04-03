import {
  AnimeListUserStatus,
  IAnimeList,
  IAnimeUserList
} from '@/types/AnimeList';
import { Provider } from '@/types/List';
import {
  IMangaList,
  IMangaUserList,
  MangaListUserStatus
} from '@/types/MangaList';

export type SynchronizedAnimeList = Record<AnimeListUserStatus, IAnimeList[]>;
export type SynchronizedMangaList = Record<MangaListUserStatus, IMangaList[]>;

interface BaseListUpdateRequest {
  providerId: Provider;
  listType: 'anime' | 'manga';
  entryId?: number;
  mediaId?: number;
}

export interface AnimeListUpdateRequest
  extends BaseListUpdateRequest, Partial<IAnimeUserList> {
  listType: 'anime';
}

export interface MangaListUpdateRequest
  extends BaseListUpdateRequest, Partial<IMangaUserList> {
  listType: 'manga';
}

export type ListUpdateRequest = AnimeListUpdateRequest | MangaListUpdateRequest;

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
