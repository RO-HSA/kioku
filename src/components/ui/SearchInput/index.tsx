import { TextField } from '@mui/material';
import debounce from 'debounce';
import { ChangeEvent, FC, useEffect, useMemo, useState } from 'react';

interface SearchInputProps {
  onDebounceEnd: (value: string) => void;
}

const SearchInput: FC<SearchInputProps> = ({ onDebounceEnd }) => {
  const [internalValue, setInternalValue] = useState('');

  const debouncedOnChange = useMemo(
    () => debounce((value: string) => onDebounceEnd(value), 200),
    [onDebounceEnd]
  );

  useEffect(() => {
    return () => debouncedOnChange.clear();
  }, [debouncedOnChange]);

  const handleChange = (event: ChangeEvent<HTMLInputElement>) => {
    const value = event.target.value;

    setInternalValue(value);
    debouncedOnChange(value);
  };

  return (
    <TextField
      label="Search"
      variant="outlined"
      value={internalValue}
      onChange={handleChange}
      fullWidth
      size="small"
    />
  );
};

export default SearchInput;
