import { TextField } from '@mui/material';
import { FC } from 'react';
import { Control, useWatch } from 'react-hook-form';

import { AnimeListFormData } from './hooks/types';

interface UserNoteProps {
  control: Control<AnimeListFormData>;
  onChange: (value: string) => void;
}

const UserNote: FC<UserNoteProps> = ({ control, onChange }) => {
  const userComments = useWatch({ name: 'userComments', control });

  return (
    <>
      <TextField
        className="max-w-116.5 w-full"
        label="Notes"
        value={userComments}
        multiline
        onChange={(event) => onChange(event.target.value)}
      />
    </>
  );
};

export default UserNote;
