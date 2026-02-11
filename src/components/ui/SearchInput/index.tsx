import { TextField } from '@mui/material';
import { ChangeEvent, FC } from 'react';

interface SearchInputProps {
  value: string;
  onChange: (value: string) => void;
}

const SearchInput: FC<SearchInputProps> = ({ value, onChange }) => {
  const handleChange = (event: ChangeEvent<HTMLInputElement>) => {
    const value = event.target.value;

    onChange(value);
  };

  return (
    <TextField
      label="Search"
      variant="outlined"
      value={value}
      onChange={handleChange}
      fullWidth
      size="small"
    />
  );
};

export default SearchInput;
