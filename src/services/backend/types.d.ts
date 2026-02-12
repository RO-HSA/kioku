import {
  AnimeListUserStatus,
  IAnimeList,
  IAnimeUserList
} from '@/types/AnimeList';
import { Provider } from '@/types/List';

export type SynchronizedAnimeList = Record<AnimeListUserStatus, IAnimeList[]>;

export interface AnimeListUpdateRequest extends Partial<IAnimeUserList> {
  providerId: Provider;
  entryId: number;
}
