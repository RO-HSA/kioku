import { useCallback } from 'react';
import { useForm } from 'react-hook-form';

import { useAnimeDetailsStore } from '@/stores/animeDetails';
import { useMyAnimeListStore } from '@/stores/providers/myanimelist';
import { IAnimeList } from '@/types/AnimeList';
import { AnimeListFormData } from './types';

interface UseAnimeListFormProps {
  selectedAnime: IAnimeList;
}

const useAnimeListForm = ({ selectedAnime }: UseAnimeListFormProps) => {
  const setIsOpen = useAnimeDetailsStore((state) => state.setIsOpen);
  const setSelectedAnime = useAnimeDetailsStore(
    (state) => state.setSelectedAnime
  );
  const updateAnimeList = useMyAnimeListStore((state) => state.updateAnimeList);

  const {
    userStatus,
    userEpisodesWatched,
    userScore,
    isRewatching,
    userFinishDate,
    userStartDate,
    userComments,
    userNumTimesRewatched
  } = selectedAnime;

  const { control, formState, setValue, handleSubmit, getValues } =
    useForm<AnimeListFormData>({
      defaultValues: {
        userEpisodesWatched,
        userScore,
        userStatus,
        isRewatching,
        userStartDate,
        userFinishDate,
        userComments,
        userNumTimesRewatched
      }
    });

  const getDirtyPayload = useCallback(() => {
    const payload: Partial<AnimeListFormData> = {};
    const dirtyFields = formState.dirtyFields as Partial<
      Record<keyof AnimeListFormData, boolean>
    >;
    const values = getValues();

    const setDirtyValue = <K extends keyof AnimeListFormData>(key: K) => {
      if (!dirtyFields[key]) return;

      payload[key] = values[key] as AnimeListFormData[K];
    };

    (Object.keys(dirtyFields) as Array<keyof AnimeListFormData>).forEach(
      setDirtyValue
    );

    return payload;
  }, [formState.dirtyFields, getValues]);

  const onSubmit = useCallback(() => {
    const payload = getDirtyPayload();

    if (Object.keys(payload).length === 0) return;

    updateAnimeList(selectedAnime.id, selectedAnime.userStatus, payload);

    setIsOpen(false);
    setSelectedAnime(null);
  }, [getDirtyPayload, setIsOpen, setSelectedAnime, updateAnimeList]);

  return {
    control,
    formState,
    setValue,
    handleSubmit,
    onSubmit
  };
};

export default useAnimeListForm;
