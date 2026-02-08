import {
  FormControl,
  FormControlLabel,
  FormGroup,
  InputLabel,
  Switch
} from '@mui/material';
import { useForm } from 'react-hook-form';

import ScoreSelect from '@/components/ScoreSelect';
import NumberField from '@/components/ui/NumberField';
import { useAnimeDetailsStore } from '@/stores/animeDetails';
import StatusSelect from './StatusSelect';
import { AnimeListFormData } from './types';

const AnimeListForm = () => {
  const selectedAnime = useAnimeDetailsStore((state) => state.selectedAnime);

  if (!selectedAnime) return null;

  const {
    totalEpisodes,
    userStatus,
    userEpisodesWatched,
    userScore,
    isRewatching,
    userFinishDate,
    userStartDate
  } = selectedAnime;

  const {} = useForm<AnimeListFormData>({
    defaultValues: {
      userEpisodesWatched,
      userScore,
      userStatus,
      isRewatching,
      userStartDate,
      userFinishDate
    }
  });

  return (
    <form className="flex flex-col gap-5">
      <div className="flex flex-wrap gap-3">
        <NumberField
          label="Episodes watched"
          min={0}
          max={totalEpisodes || undefined}
          value={userEpisodesWatched}
        />

        <NumberField label="Times watched" />

        <FormGroup>
          <FormControlLabel control={<Switch />} label="Rewatching" />
        </FormGroup>
      </div>

      <div className="flex flex-wrap gap-3">
        <StatusSelect status={userStatus} />

        <FormControl className="max-w-56.75 w-full">
          <InputLabel id="score-label">Score</InputLabel>

          <ScoreSelect
            labelId="score-label"
            label="Score"
            score={userScore}
            shouldNotOverrideRenderValue
            onChange={() => {}}
          />
        </FormControl>
      </div>
    </form>
  );
};

export default AnimeListForm;
