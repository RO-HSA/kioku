import { MangaListUserStatus } from '@/types/MangaList';

export type MangaListFormData = {
  userStatus: MangaListUserStatus;
  userScore: number;
  userChaptersRead: number;
  userVolumesRead: number;
  isRereading: boolean;
  userComments: string;
  userNumTimesReread: number;
  userStartDate?: string;
  userFinishDate?: string;
};
