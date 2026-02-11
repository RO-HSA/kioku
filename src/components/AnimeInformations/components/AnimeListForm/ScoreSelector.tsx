import { FormControl, InputLabel } from '@mui/material';
import { FC } from 'react';
import { Control, useWatch } from 'react-hook-form';

import ScoreSelect from '@/components/ScoreSelect';
import { AnimeListFormData } from './hooks/types';

interface ScoreSelectorProps {
  control: Control<AnimeListFormData>;
  onChange: (value: number) => void;
}

const ScoreSelector: FC<ScoreSelectorProps> = ({ control, onChange }) => {
  const score = useWatch({ name: 'userScore', control });

  return (
    <FormControl className="max-w-56.75 w-full">
      <InputLabel id="score-label">Score</InputLabel>

      <ScoreSelect
        labelId="score-label"
        label="Score"
        score={score}
        shouldNotOverrideRenderValue
        onChange={(value) => onChange(value.target.value)}
      />
    </FormControl>
  );
};

export default ScoreSelector;
