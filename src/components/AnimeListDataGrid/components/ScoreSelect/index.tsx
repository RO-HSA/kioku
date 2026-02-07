import { AnimeListUserStatus } from '@/services/backend/types';
import { useMyAnimeListStore } from '@/stores/providers/myanimelist';
import { Box, MenuItem, Select, SelectChangeEvent } from '@mui/material';
import { FC } from 'react';

interface ScoreSelectProps {
  animeId: number;
  status: AnimeListUserStatus;
  score: number;
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

const ScoreSelect: FC<ScoreSelectProps> = ({ animeId, status, score }) => {
  const setScore = useMyAnimeListStore((state) => state.setScore);

  const handleChange = (event: SelectChangeEvent<number>) => {
    const { value } = event.target;

    setScore(animeId, status, value);
  };

  return (
    <Box width="100%">
      <Select
        onChange={handleChange}
        value={score}
        fullWidth
        size="small"
        renderValue={(selected) => {
          if (selected === 0) {
            return '-';
          }

          return selected;
        }}>
        {options.map((option) => (
          <MenuItem key={option.value} value={option.value}>
            {option.label}
          </MenuItem>
        ))}
      </Select>
    </Box>
  );
};

export default ScoreSelect;
