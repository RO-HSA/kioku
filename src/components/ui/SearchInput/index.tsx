import { InputAdornment, TextField } from '@mui/material';
import { X } from 'lucide-react';
import { ChangeEvent, FC, KeyboardEvent } from 'react';

interface SearchInputProps {
  value: string;
  onChange: (value: string) => void;
}

const SearchInput: FC<SearchInputProps> = ({ value, onChange }) => {
  const handleChange = (event: ChangeEvent<HTMLInputElement>) => {
    const value = event.target.value;

    onChange(value);
  };

  const handleKeyDown = (event: KeyboardEvent<HTMLInputElement>) => {
    if (event.key !== 'Escape') {
      event.stopPropagation();
    }
  };

  return (
    <TextField
      label="Search"
      variant="outlined"
      value={value}
      onChange={handleChange}
      onKeyDown={handleKeyDown}
      fullWidth
      size="small"
      sx={{
        '& .MuiInputAdornment-root': {
          visibility: value.length > 0 ? 'visible' : 'hidden'
        }
      }}
      slotProps={{
        input: {
          endAdornment: (
            <InputAdornment position="end" sx={{}}>
              <X size={14} cursor="pointer" />
            </InputAdornment>
          )
        }
      }}
    />
  );
};

export default SearchInput;
