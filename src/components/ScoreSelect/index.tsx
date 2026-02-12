import { MenuItem, Select, SelectChangeEvent } from '@mui/material';
import { FC } from 'react';

interface ScoreSelectProps {
  score: number;
  labelId?: string;
  label?: string;
  shouldNotOverrideRenderValue?: boolean;
  fullWidth?: boolean;
  onChange: (event: SelectChangeEvent<number>) => void;
}

const options = [
  { value: 0, label: '(0) No Score' },
  { value: 1, label: '(1) Appalling' },
  { value: 2, label: '(2) Horrible' },
  { value: 3, label: '(3) Very Bad' },
  { value: 4, label: '(4) Bad' },
  { value: 5, label: '(5) Average' },
  { value: 6, label: '(6) Fine' },
  { value: 7, label: '(7) Good' },
  { value: 8, label: '(8) Very Good' },
  { value: 9, label: '(9) Great' },
  { value: 10, label: '(10) Masterpiece' }
];

const ScoreSelect: FC<ScoreSelectProps> = ({
  score,
  label,
  labelId,
  fullWidth = false,
  shouldNotOverrideRenderValue = false,
  onChange
}) => {
  const renderValue = (selected: number) => {
    if (shouldNotOverrideRenderValue) {
      const option = options.find((option) => option.value === selected);

      return option ? option.label : '';
    }

    if (selected === 0) {
      return '-';
    }

    return selected;
  };

  return (
    <Select
      onChange={onChange}
      value={score}
      fullWidth={fullWidth}
      labelId={labelId}
      label={label}
      size="small"
      renderValue={(selected) => renderValue(selected)}>
      {options.map((option) => (
        <MenuItem key={option.value} value={option.value}>
          {option.label}
        </MenuItem>
      ))}
    </Select>
  );
};

export default ScoreSelect;
