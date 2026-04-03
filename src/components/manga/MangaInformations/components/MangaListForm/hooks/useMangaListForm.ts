import { useCallback } from 'react';
import { useForm } from 'react-hook-form';

import { useAnimeDetailsStore } from '@/stores/animeDetails';
import { useMangaDetailsStore } from '@/stores/mangaDetails';
import { useAniListStore } from '@/stores/providers/anilist';
import { useMyAnimeListStore } from '@/stores/providers/myanimelist';
import { useProviderStore } from '@/stores/providers/provider';
import { Provider } from '@/types/List';
import { IMangaList } from '@/types/MangaList';
import { MangaListFormData } from './types';

interface UseMangaListFormProps {
  selectedManga: IMangaList;
}
1;

const useMangaListForm = ({ selectedManga }: UseMangaListFormProps) => {
  const setIsOpen = useAnimeDetailsStore((state) => state.setIsOpen);
  const setSelectedManga = useMangaDetailsStore(
    (state) => state.setSelectedManga
  );

  const activeProvider = useProviderStore((state) => state.activeProvider);

  const updateMyAnimeListMangaList = useMyAnimeListStore(
    (state) => state.updateMangaList
  );
  const updateAniListMangaList = useAniListStore(
    (state) => state.updateMangaList
  );

  const {
    userStatus,
    userChaptersRead,
    userVolumesRead,
    userScore,
    isRereading,
    userFinishDate,
    userStartDate,
    userComments,
    userNumTimesReread
  } = selectedManga;

  const { control, formState, setValue, handleSubmit, getValues } =
    useForm<MangaListFormData>({
      defaultValues: {
        userChaptersRead,
        userVolumesRead,
        userScore,
        userStatus,
        isRereading,
        userStartDate,
        userFinishDate,
        userComments,
        userNumTimesReread
      }
    });

  const getDirtyPayload = useCallback(() => {
    const payload: Partial<MangaListFormData> = {};
    const dirtyFields = formState.dirtyFields as Partial<
      Record<keyof MangaListFormData, boolean>
    >;
    const values = getValues();

    const setDirtyValue = <K extends keyof MangaListFormData>(key: K) => {
      if (!dirtyFields[key]) return;

      payload[key] = values[key] as MangaListFormData[K];
    };

    (Object.keys(dirtyFields) as Array<keyof MangaListFormData>).forEach(
      setDirtyValue
    );

    return payload;
  }, [formState.dirtyFields, getValues]);

  const onSubmit = useCallback(() => {
    const payload = getDirtyPayload();

    if (Object.keys(payload).length === 0) return;

    switch (activeProvider) {
      case Provider.MY_ANIME_LIST:
        updateMyAnimeListMangaList(
          selectedManga.id,
          selectedManga.userStatus,
          payload
        );
        break;
      case Provider.ANILIST:
        updateAniListMangaList(
          selectedManga.id,
          selectedManga.userStatus,
          payload
        );
        break;
      default:
        break;
    }

    setIsOpen(false);
    setSelectedManga(null);
  }, [
    selectedManga,
    activeProvider,
    getDirtyPayload,
    setIsOpen,
    setSelectedManga,
    updateMyAnimeListMangaList,
    updateAniListMangaList
  ]);

  return {
    control,
    formState,
    setValue,
    handleSubmit,
    onSubmit
  };
};

export default useMangaListForm;
