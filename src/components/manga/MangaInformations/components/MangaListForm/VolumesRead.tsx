import { Control, useWatch } from 'react-hook-form';

import NumberField from '@/components/ui/NumberField';
import { FC } from 'react';
import { MangaListFormData } from './hooks/types';

interface VolumesReadProps {
  control: Control<MangaListFormData>;
  totalVolumes: number;
  onChange: (value: number) => void;
}

const VolumesRead: FC<VolumesReadProps> = ({
  control,
  totalVolumes,
  onChange
}) => {
  const userVolumes = useWatch({
    name: 'userVolumesRead',
    control
  });

  return (
    <>
      <NumberField
        label="Volumes read"
        min={0}
        max={totalVolumes || undefined}
        value={userVolumes}
        onValueChange={(value) => onChange(value || 0)}
      />
    </>
  );
};

export default VolumesRead;
