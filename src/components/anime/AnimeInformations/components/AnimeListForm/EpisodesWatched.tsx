import { Control, useWatch } from 'react-hook-form';

import NumberField from '@/components/ui/NumberField';
import { FC } from 'react';
import { AnimeListFormData } from './hooks/types';

interface EpisodesWatchedProps {
  control: Control<AnimeListFormData>;
  totalEpisodes: number;
  onChange: (value: number) => void;
}

const EpisodesWatched: FC<EpisodesWatchedProps> = ({
  control,
  totalEpisodes,
  onChange
}) => {
  const userEpisodes = useWatch({
    name: 'userEpisodesWatched',
    control
  });

  return (
    <>
      <NumberField
        label="Episodes watched"
        min={0}
        max={totalEpisodes || undefined}
        value={userEpisodes}
        onValueChange={(value) => onChange(value || 0)}
      />
    </>
  );
};

export default EpisodesWatched;
