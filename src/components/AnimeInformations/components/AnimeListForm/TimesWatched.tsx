import { Control, useWatch } from 'react-hook-form';

import NumberField from '@/components/ui/NumberField';
import { FC } from 'react';
import { AnimeListFormData } from './hooks/types';

interface TimesWatchedProps {
  control: Control<AnimeListFormData>;
  onChange: (value: number) => void;
}

const TimesWatched: FC<TimesWatchedProps> = ({ control, onChange }) => {
  const userNumTimesRewatched = useWatch({
    name: 'userNumTimesRewatched',
    control
  });

  return (
    <>
      <NumberField
        label="Times watched"
        min={0}
        max={5}
        value={userNumTimesRewatched}
        onValueChange={(value) => onChange(value || 0)}
      />
    </>
  );
};

export default TimesWatched;
