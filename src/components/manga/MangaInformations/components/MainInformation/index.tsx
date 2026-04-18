import { Typography } from '@mui/material';
import { FC } from 'react';

import SafeRichText from '@/components/ui/SafeRichText';
import { IMangaList } from '@/types/MangaList';
import Details from '../Details';
import InfoHeader from '../InfoHeader';

interface MainInformationProps {
  manga: IMangaList;
}

const MainInformation: FC<MainInformationProps> = ({ manga }) => {
  const {
    alternativeTitles,
    mediaType,
    status,
    totalChapters,
    totalVolumes,
    genres,
    score,
    authors,
    synopsis
  } = manga;

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
        totalChapters={totalChapters}
        totalVolumes={totalVolumes}
        genres={genres}
        authors={authors}
        score={score}
      />

      <InfoHeader label="Synopsis" />

      <SafeRichText className="py-2!" content={synopsis} />
    </div>
  );
};

export default MainInformation;
