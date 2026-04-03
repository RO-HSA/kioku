import { FormControl, InputLabel, MenuItem, Select } from '@mui/material';
import { FC } from 'react';
import { Control, useWatch } from 'react-hook-form';

import { AnimeListUserStatus } from '@/types/AnimeList';
import { AnimeListFormData } from './hooks/types';

interface StatusSelectProps {
  control: Control<AnimeListFormData>;
  onChange: (status: AnimeListUserStatus) => void;
}

const StatusSelect: FC<StatusSelectProps> = ({ control, onChange }) => {
  const status = useWatch({ name: 'userStatus', control });

  return (
    <FormControl className="max-w-56.75 w-full">
      <InputLabel id="status">Status</InputLabel>
      <Select
        labelId="status"
        label="Status"
        size="small"
        value={status}
        onChange={(e) => onChange(e.target.value as AnimeListUserStatus)}>
        <MenuItem value="watching">Watching</MenuItem>
        <MenuItem value="completed">Completed</MenuItem>
        <MenuItem value="onHold">On Hold</MenuItem>
        <MenuItem value="dropped">Dropped</MenuItem>
        <MenuItem value="planToWatch">Plan to Watch</MenuItem>
      </Select>
    </FormControl>
  );
};

export default StatusSelect;
