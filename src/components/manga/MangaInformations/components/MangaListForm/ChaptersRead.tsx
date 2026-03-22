import { FC } from 'react';
import { Control, useWatch } from 'react-hook-form';

import NumberField from '@/components/ui/NumberField';
import { MangaListFormData } from './hooks/types';

interface ChaptersReadProps {
  control: Control<MangaListFormData>;
  totalChapters: number;
  onChange: (value: number) => void;
}

const ChaptersRead: FC<ChaptersReadProps> = ({
  control,
  totalChapters,
  onChange
}) => {
  const chaptersRead = useWatch({
    name: 'userChaptersRead',
    control
  });

  return (
    <>
      <NumberField
        label="Chapters read"
        min={0}
        max={totalChapters || undefined}
        value={chaptersRead}
        onValueChange={(value) => onChange(value || 0)}
      />
    </>
  );
};

export default ChaptersRead;
