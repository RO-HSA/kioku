import { Typography } from '@mui/material';
import { FC } from 'react';

interface DetailsProps {
  mediaType: string;
  status: string;
  totalChapters: number;
  totalVolumes: number;
  genres: string;
  authors: string;
  score: number;
}

const Details: FC<DetailsProps> = ({
  mediaType,
  status,
  totalChapters,
  totalVolumes,
  genres,
  authors,
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
        <Typography variant="body2">Chapters:</Typography>
        <Typography variant="body2">
          {totalChapters !== 0 ? totalChapters : 'Unknown'}
        </Typography>
      </div>

      <div className="grid grid-cols-2">
        <Typography variant="body2">Volumes:</Typography>
        <Typography variant="body2">
          {totalVolumes !== 0 ? totalVolumes : 'Unknown'}
        </Typography>
      </div>

      <div className="grid grid-cols-2">
        <Typography variant="body2">Genres:</Typography>
        <Typography variant="body2">{genres}</Typography>
      </div>

      <div className="grid grid-cols-2">
        <Typography variant="body2">Authors:</Typography>
        <Typography variant="body2">{authors}</Typography>
      </div>

      <div className="grid grid-cols-2">
        <Typography variant="body2">Score:</Typography>
        <Typography variant="body2">{score}</Typography>
      </div>
    </div>
  );
};

export default Details;
