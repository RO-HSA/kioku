import { useWatch } from 'react-hook-form';

import { useAnimeDetailsStore } from '@/stores/animeDetails';
import { formatDateValue, parseDateValue } from '@/utils/date';
import DatePicker from './DatePIcker';
import EpisodesWatched from './EpisodesWatched';
import useAnimeListForm from './hooks/useAnimeListForm';
import Rewatching from './Rewatching';
import ScoreSelector from './ScoreSelector';
import StatusSelector from './StatusSelector';
import TimesWatched from './TimesWatched';
import UserNote from './UserNote';

const AnimeListForm = () => {
  const formRef = useAnimeDetailsStore((state) => state.formRef);
  const selectedAnime = useAnimeDetailsStore((state) => state.selectedAnime);

  if (!selectedAnime) return null;

  const { control, setValue, handleSubmit, onSubmit } = useAnimeListForm({
    selectedAnime
  });

  const startDate = useWatch({
    name: 'userStartDate',
    control
  });

  const finishDate = useWatch({
    name: 'userFinishDate',
    control
  });

  return (
    <form
      ref={formRef}
      className="flex flex-col gap-5"
      onSubmit={handleSubmit(onSubmit)}>
      <div className="flex flex-wrap gap-3">
        <EpisodesWatched
          control={control}
          totalEpisodes={selectedAnime.totalEpisodes}
          onChange={(value) =>
            setValue('userEpisodesWatched', value, { shouldDirty: true })
          }
        />

        <TimesWatched
          control={control}
          onChange={(value) =>
            setValue('userNumTimesRewatched', value, { shouldDirty: true })
          }
        />

        <Rewatching
          control={control}
          onChange={(value) =>
            setValue('isRewatching', value, { shouldDirty: true })
          }
        />
      </div>

      <div className="flex flex-wrap gap-3">
        <StatusSelector
          control={control}
          onChange={(value) =>
            setValue('userStatus', value, { shouldDirty: true })
          }
        />

        <ScoreSelector
          control={control}
          onChange={(value) =>
            setValue('userScore', value || 0, {
              shouldDirty: true
            })
          }
        />

        <div className="flex flex-wrap gap-3 min-w-0">
          <DatePicker
            label="Date started"
            value={parseDateValue(startDate)}
            onChange={(date) =>
              setValue('userStartDate', formatDateValue(date), {
                shouldDirty: true
              })
            }
          />

          <DatePicker
            label="Date completed"
            value={parseDateValue(finishDate)}
            onChange={(date) =>
              setValue('userFinishDate', formatDateValue(date), {
                shouldDirty: true
              })
            }
          />
        </div>

        <UserNote
          control={control}
          onChange={(value) =>
            setValue('userComments', value, { shouldDirty: true })
          }
        />
      </div>
    </form>
  );
};

export default AnimeListForm;
