import {
  FormControl,
  FormControlLabel,
  FormGroup,
  InputLabel,
  Switch
} from '@mui/material';
import { useWatch } from 'react-hook-form';

import ScoreSelect from '@/components/ScoreSelect';
import NumberField from '@/components/ui/NumberField';
import { useAnimeDetailsStore } from '@/stores/animeDetails';
import { formatDateValue, parseDateValue } from '@/utils/date';
import DatePicker from './DatePIcker';
import StatusSelect from './StatusSelect';
import useAnimeListForm from './hooks/useAnimeListForm';

const AnimeListForm = () => {
  const formRef = useAnimeDetailsStore((state) => state.formRef);
  const selectedAnime = useAnimeDetailsStore((state) => state.selectedAnime);

  if (!selectedAnime) return null;

  const { control, setValue, handleSubmit, onSubmit } = useAnimeListForm({
    selectedAnime
  });

  const totalEpisodes = useWatch({
    name: 'userEpisodesWatched',
    control: control
  });

  const userNumTimesRewatched = useWatch({
    name: 'userNumTimesRewatched',
    control: control
  });

  const isRewatching = useWatch({
    name: 'isRewatching',
    control: control
  });

  const userEpisodes = useWatch({
    name: 'userEpisodesWatched',
    control: control
  });

  const score = useWatch({
    name: 'userScore',
    control: control
  });

  const startDate = useWatch({
    name: 'userStartDate',
    control: control
  });

  const finishDate = useWatch({
    name: 'userFinishDate',
    control: control
  });

  const userStatus = useWatch({
    name: 'userStatus',
    control: control
  });

  return (
    <form
      ref={formRef}
      className="flex flex-col gap-5"
      onSubmit={handleSubmit(onSubmit)}>
      <div className="flex flex-wrap gap-3">
        <NumberField
          label="Episodes watched"
          min={0}
          value={userEpisodes}
          defaultValue={totalEpisodes}
          onValueChange={(value) =>
            setValue('userEpisodesWatched', value || 0, { shouldDirty: true })
          }
        />

        <NumberField
          label="Times watched"
          min={0}
          max={5}
          value={userNumTimesRewatched}
          onValueChange={(value) =>
            setValue('userNumTimesRewatched', value || 0, { shouldDirty: true })
          }
        />

        <FormGroup>
          <FormControlLabel
            control={
              <Switch
                value={isRewatching}
                onChange={(event) =>
                  setValue('isRewatching', event.target.checked)
                }
              />
            }
            label="Rewatching"
          />
        </FormGroup>
      </div>

      <div className="flex flex-wrap gap-3">
        <StatusSelect
          status={userStatus}
          onChange={(value) =>
            setValue('userStatus', value, { shouldDirty: true })
          }
        />

        <FormControl className="max-w-56.75 w-full">
          <InputLabel id="score-label">Score</InputLabel>

          <ScoreSelect
            labelId="score-label"
            label="Score"
            score={score}
            shouldNotOverrideRenderValue
            onChange={(event) =>
              setValue('userScore', event.target.value || 0, {
                shouldDirty: true
              })
            }
          />
        </FormControl>

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
      </div>
    </form>
  );
};

export default AnimeListForm;
