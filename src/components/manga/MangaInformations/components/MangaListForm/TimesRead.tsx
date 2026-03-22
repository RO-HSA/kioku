import { FC } from 'react';
import { Control, useWatch } from 'react-hook-form';

import NumberField from '@/components/ui/NumberField';
import { MangaListFormData } from './hooks/types';

interface TimesReadProps {
  control: Control<MangaListFormData>;
  onChange: (value: number) => void;
}

const TimesRead: FC<TimesReadProps> = ({ control, onChange }) => {
  const userNumTimesReread = useWatch({
    name: 'userNumTimesReread',
    control
  });

  return (
    <>
      <NumberField
        label="Times read"
        min={0}
        max={5}
        value={userNumTimesReread}
        onValueChange={(value) => onChange(value || 0)}
      />
    </>
  );
};

export default TimesRead;
