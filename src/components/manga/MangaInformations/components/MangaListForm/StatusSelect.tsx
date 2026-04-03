import { FormControl, InputLabel, MenuItem, Select } from '@mui/material';
import { FC } from 'react';
import { Control, useWatch } from 'react-hook-form';

import { MangaListUserStatus } from '@/types/MangaList';
import { MangaListFormData } from './hooks/types';

interface StatusSelectProps {
  control: Control<MangaListFormData>;
  onChange: (status: MangaListUserStatus) => void;
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
        onChange={(e) => onChange(e.target.value as MangaListUserStatus)}>
        <MenuItem value="reading">Reading</MenuItem>
        <MenuItem value="completed">Completed</MenuItem>
        <MenuItem value="onHold">On Hold</MenuItem>
        <MenuItem value="dropped">Dropped</MenuItem>
        <MenuItem value="planToRead">Plan to Read</MenuItem>
      </Select>
    </FormControl>
  );
};

export default StatusSelect;
