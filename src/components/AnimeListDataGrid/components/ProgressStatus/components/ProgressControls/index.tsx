import { Box } from '@mui/material';
import { Minus, Plus } from 'lucide-react';
import { FC } from 'react';

import { AnimeListUserStatus } from '@/types/AnimeList';

export interface ProgressControlsProps {
  progress: number;
  total: number;
  status: AnimeListUserStatus;
  onProgressChange: (newProgress: number) => void;
}

const ProgressControls: FC<ProgressControlsProps> = ({
  progress,
  total,
  status,
  onProgressChange
}) => {
  const shouldShowMinus = progress > 0 && status !== 'completed';
  const shouldShowPlus =
    (progress < total || total === 0) && status !== 'completed';

  return (
    <Box
      position="absolute"
      top={0}
      left={0}
      display="flex"
      justifyContent="space-between"
      width="100%">
      {shouldShowMinus && (
        <div className="bg-error-dark cursor-pointer">
          <Minus
            size={14}
            className="text-white"
            onClick={() => onProgressChange(progress - 1)}
          />
        </div>
      )}

      {shouldShowPlus && (
        <div className="bg-success-dark cursor-pointer">
          <Plus
            size={14}
            className="text-white"
            onClick={() => onProgressChange(progress + 1)}
          />
        </div>
      )}
    </Box>
  );
};

export default ProgressControls;
