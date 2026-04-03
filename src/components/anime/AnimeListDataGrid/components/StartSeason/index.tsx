import { Tooltip } from '@mui/material';
import { FC } from 'react';

interface StartSeasonProps {
  startSeason: string;
}

const StartSeason: FC<StartSeasonProps> = ({ startSeason }) => {
  const transformedValue =
    startSeason.charAt(0).toUpperCase() + startSeason.slice(1);

  return (
    <Tooltip title={transformedValue}>
      <div className="w-full">
        {transformedValue !== 'Unknown' ? (
          transformedValue
        ) : (
          <span className="text-gray-400">Unknown</span>
        )}
      </div>
    </Tooltip>
  );
};

export default StartSeason;
