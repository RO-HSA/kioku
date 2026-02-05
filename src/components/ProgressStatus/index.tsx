import { Box, LinearProgress } from '@mui/material';
import { FC } from 'react';

import { AnimeListBroadcast } from '@/services/backend/types';
import { getProgressValues } from './utils';

interface ProgressStatusProps {
  progress: number;
  total: number;
  startDate: string | null;
  broadcast: AnimeListBroadcast;
}

const ProgressStatus: FC<ProgressStatusProps> = ({
  progress,
  total,
  startDate,
  broadcast
}) => {
  const { progressValue, bufferPercent } = getProgressValues(
    progress,
    total,
    startDate,
    broadcast
  );

  return (
    <Box width="100%">
      <LinearProgress
        variant="buffer"
        value={progressValue}
        sx={{
          '&& .MuiLinearProgress-dashed': {
            animation: 'none',
            backgroundImage: 'none',
            backgroundColor: (theme) => theme.palette.grey.A200
          }
        }}
        valueBuffer={bufferPercent}
      />
    </Box>
  );
};

export default ProgressStatus;
