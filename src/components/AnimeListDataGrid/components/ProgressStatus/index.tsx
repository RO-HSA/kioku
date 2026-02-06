import { Box, LinearProgress } from '@mui/material';
import { FC, useState } from 'react';

import { AnimeListBroadcast } from '@/services/backend/types';
import ProgressControls from './components/ProgressControls';
import ProgressNumber from './components/ProgressNumber';
import { getProgressValues } from './utils';

interface ProgressStatusProps {
  progress: number;
  total: number;
  startDate: string | null;
  broadcast: AnimeListBroadcast;
  onProgressChange: (newProgress: number) => void;
}

const ProgressStatus: FC<ProgressStatusProps> = ({
  progress,
  total,
  startDate,
  broadcast,
  onProgressChange
}) => {
  const [isHovered, setIsHovered] = useState(false);
  const { progressValue, bufferPercent } = getProgressValues(
    progress,
    total,
    startDate,
    broadcast
  );

  return (
    <Box display="flex" width="100%" height="100%" gap={1} alignItems="center">
      <Box
        position="relative"
        justifyContent="space-between"
        width="100%"
        onMouseEnter={() => setIsHovered(true)}
        onMouseLeave={() => setIsHovered(false)}>
        <LinearProgress
          variant="buffer"
          value={progressValue}
          sx={{
            height: '14px',
            width: '100%',
            '&& .MuiLinearProgress-dashed': {
              animation: 'none',
              backgroundImage: 'none',
              backgroundColor: (theme) => theme.palette.grey.A200
            }
          }}
          valueBuffer={bufferPercent}
        />

        {isHovered && (
          <ProgressControls
            progress={progress}
            total={total}
            onProgressChange={onProgressChange}
          />
        )}
      </Box>

      <ProgressNumber progress={progress} total={total} />
    </Box>
  );
};

export default ProgressStatus;
