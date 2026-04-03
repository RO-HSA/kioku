import { FormControlLabel, FormGroup, Switch } from '@mui/material';
import { FC } from 'react';
import { Control, useWatch } from 'react-hook-form';

import { MangaListFormData } from './hooks/types';

interface RereadingProps {
  control: Control<MangaListFormData>;
  onChange: (isRereading: boolean) => void;
}

const Rereading: FC<RereadingProps> = ({ control, onChange }) => {
  const isRereading = useWatch({ name: 'isRereading', control });

  return (
    <FormGroup>
      <FormControlLabel
        control={
          <Switch
            checked={isRereading}
            onChange={(event) => onChange(event.target.checked)}
          />
        }
        label="Rereading"
      />
    </FormGroup>
  );
};

export default Rereading;
