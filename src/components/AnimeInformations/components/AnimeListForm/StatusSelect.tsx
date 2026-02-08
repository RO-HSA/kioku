import { AnimeListUserStatus } from '@/services/backend/types';
import { FormControl, InputLabel, MenuItem, Select } from '@mui/material';
import { FC } from 'react';

interface StatusSelectProps {
  status: AnimeListUserStatus;
}

const StatusSelect: FC<StatusSelectProps> = ({ status }) => {
  return (
    <FormControl className="max-w-56.75 w-full">
      <InputLabel id="status">Status</InputLabel>
      <Select labelId="status" label="Status" size="small" value={status}>
        <MenuItem value="watching">Watching</MenuItem>
        <MenuItem value="completed">Completed</MenuItem>
        <MenuItem value="onHold">Paused</MenuItem>
        <MenuItem value="dropped">Dropped</MenuItem>
        <MenuItem value="planToWatch">Plan to Watch</MenuItem>
      </Select>
    </FormControl>
  );
};

export default StatusSelect;
