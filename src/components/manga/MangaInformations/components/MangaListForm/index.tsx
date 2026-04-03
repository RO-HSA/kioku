import { useWatch } from 'react-hook-form';

import DatePicker from '@/components/DatePicker';
import { useMangaDetailsStore } from '@/stores/mangaDetails';
import { formatDateValue, parseDateValue } from '@/utils/date';
import ChaptersRead from './ChaptersRead';
import useMangaListForm from './hooks/useMangaListForm';
import Rereading from './Rereading';
import ScoreSelector from './ScoreSelector';
import StatusSelect from './StatusSelect';
import TimesRead from './TimesRead';
import UserNote from './UserNote';
import VolumesRead from './VolumesRead';

const MangaListForm = () => {
  const formRef = useMangaDetailsStore((state) => state.formRef);
  const selectedManga = useMangaDetailsStore((state) => state.selectedManga);

  if (!selectedManga) return null;

  const { control, setValue, handleSubmit, onSubmit } = useMangaListForm({
    selectedManga
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
        <ChaptersRead
          control={control}
          totalChapters={selectedManga.totalChapters}
          onChange={(value) =>
            setValue('userChaptersRead', value, { shouldDirty: true })
          }
        />

        <VolumesRead
          control={control}
          totalVolumes={selectedManga.totalVolumes}
          onChange={(value) =>
            setValue('userVolumesRead', value, { shouldDirty: true })
          }
        />
      </div>

      <div className="flex flex-wrap gap-3">
        <TimesRead
          control={control}
          onChange={(value) =>
            setValue('userNumTimesReread', value, { shouldDirty: true })
          }
        />

        <Rereading
          control={control}
          onChange={(value) =>
            setValue('isRereading', value, { shouldDirty: true })
          }
        />
      </div>

      <div className="flex flex-wrap gap-3">
        <StatusSelect
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

export default MangaListForm;
