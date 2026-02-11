import { FC } from 'react';
import { Control, useWatch } from 'react-hook-form';

import { AnimeListUserStatus } from '@/types/AnimeList';
import { AnimeListFormData } from './hooks/types';
import StatusSelect from './StatusSelect';

interface StatusSelectorProps {
  control: Control<AnimeListFormData>;
  onChange: (value: AnimeListUserStatus) => void;
}

const StatusSelector: FC<StatusSelectorProps> = ({ control, onChange }) => {
  const userStatus = useWatch({ name: 'userStatus', control });

  return (
    <>
      <StatusSelect status={userStatus} onChange={(value) => onChange(value)} />
    </>
  );
};

export default StatusSelector;
