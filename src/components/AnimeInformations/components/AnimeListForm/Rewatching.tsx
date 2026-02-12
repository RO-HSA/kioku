import { FormControlLabel, FormGroup, Switch } from '@mui/material';
import { FC } from 'react';
import { Control, useWatch } from 'react-hook-form';

import { AnimeListFormData } from './hooks/types';

interface RewatchingProps {
  control: Control<AnimeListFormData>;
  onChange: (isRewatching: boolean) => void;
}

const Rewatching: FC<RewatchingProps> = ({ control, onChange }) => {
  const isRewatching = useWatch({ name: 'isRewatching', control });

  return (
    <FormGroup>
      <FormControlLabel
        control={
          <Switch
            checked={isRewatching}
            onChange={(event) => onChange(event.target.checked)}
          />
        }
        label="Rewatching"
      />
    </FormGroup>
  );
};

export default Rewatching;
