export type MangaListStatus =
  | 'Finished'
  | 'Currently Publishing'
  | 'Not Yet Published'
  | 'On Hiatus'
  | 'Discontinued';

export type MangaListUserStatus =
  | 'reading'
  | 'completed'
  | 'onHold'
  | 'dropped'
  | 'planToRead';

export type MangaListMediaType =
  | 'Manga'
  | 'Novel'
  | 'Light Novel'
  | 'One-shot'
  | 'Doujinshi'
  | 'Manhwa'
  | 'Manhua'
  | 'OEL'
  | 'Unknown';

export interface IManga {
  id: number;
  title: string;
  imageUrl: string;
  synopsis: string;
  alternativeTitles: string;
  score: number;
  status: MangaListStatus;
  totalVolumes: number;
  totalChapters: number;
  genres: string;
  startDate: string | null;
  endDate: string | null;
  authors: string;
  serialization: string;
  mediaType: MangaListMediaType;
}

export interface IMangaUserList {
  entryId?: number;
  userStatus: MangaListUserStatus;
  userScore: number;
  userVolumesRead: number;
  userChaptersRead: number;
  isRereading: boolean;
  userComments: string;
  userNumTimesReread: number;
  userStartDate?: string;
  userFinishDate?: string;
  updatedAt?: string;
}

export interface IMangaList extends IManga, IMangaUserList {}
