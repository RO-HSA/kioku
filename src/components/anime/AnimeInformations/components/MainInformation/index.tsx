import { Typography } from '@mui/material';
import { FC } from 'react';

import SafeRichText from '@/components/ui/SafeRichText';
import { IAnimeList } from '@/types/AnimeList';
import Details from '../Details';
import InfoHeader from '../InfoHeader';

interface MainInformationProps {
  anime: IAnimeList;
}

const MainInformation: FC<MainInformationProps> = ({ anime }) => {
  const {
    alternativeTitles,
    mediaType,
    status,
    totalEpisodes,
    startSeason,
    genres,
    studios,
    score,
    synopsis
  } = anime;

  return (
    <div>
      <InfoHeader label="Alternative titles" />

      <Typography className="py-2!" variant="body2">
        {alternativeTitles}
      </Typography>

      <InfoHeader label="Details" />

      <Details
        mediaType={mediaType}
        status={status}
        totalEpisodes={totalEpisodes}
        startSeason={startSeason}
        genres={genres}
        studios={studios}
        score={score}
      />

      <InfoHeader label="Synopsis" />

      <SafeRichText className="py-2!" content={synopsis} />
    </div>
  );
};

export default MainInformation;
