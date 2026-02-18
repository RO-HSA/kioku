import { Typography } from '@mui/material';
import { FC } from 'react';

interface DetailsProps {
  mediaType: string;
  status: string;
  totalEpisodes: number;
  startSeason: string;
  genres: string;
  studios: string;
  score: number;
}

const Details: FC<DetailsProps> = ({
  mediaType,
  status,
  totalEpisodes,
  startSeason,
  genres,
  studios,
  score
}) => {
  return (
    <div className="flex flex-col gap-1 py-1 max-w-64">
      <div className="grid grid-cols-2">
        <Typography variant="body2">Type:</Typography>
        <Typography variant="body2">{mediaType}</Typography>
      </div>

      <div className="grid grid-cols-2">
        <Typography variant="body2">Status:</Typography>
        <Typography variant="body2">{status}</Typography>
      </div>

      <div className="grid grid-cols-2">
        <Typography variant="body2">Episodes:</Typography>
        <Typography variant="body2">
          {totalEpisodes !== 0 ? totalEpisodes : 'Unknown'}
        </Typography>
      </div>

      <div className="grid grid-cols-2">
        <Typography variant="body2">Season:</Typography>
        <Typography variant="body2">
          {startSeason.charAt(0).toUpperCase() + startSeason.slice(1)}
        </Typography>
      </div>

      <div className="grid grid-cols-2">
        <Typography variant="body2">Genres:</Typography>
        <Typography variant="body2">{genres}</Typography>
      </div>

      <div className="grid grid-cols-2">
        <Typography variant="body2">Producers:</Typography>
        <Typography variant="body2">{studios}</Typography>
      </div>

      <div className="grid grid-cols-2">
        <Typography variant="body2">Score:</Typography>
        <Typography variant="body2">
          {score > 10 ? score / 10 : score}
        </Typography>
      </div>
    </div>
  );
};

export default Details;
