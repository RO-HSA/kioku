import { Box } from '@mui/material';
import { MaterialReactTable } from 'material-react-table';

import {
  AnimeListUserStatus,
  SynchronizedAnimeList
} from '@/services/backend/types';
import { useMyAnimeListStore } from '@/stores/providers/myanimelist';
import useAnimeListDataGrid from './hooks/useAnimeListDataGrid';

interface AnimeListDataGridProps {
  listData: SynchronizedAnimeList | null;
}

const AnimeListDataGrid = ({ listData }: AnimeListDataGridProps) => {
  const setProgress = useMyAnimeListStore((state) => state.setProgress);

  const handleProgressChange = (
    animeId: number,
    status: AnimeListUserStatus,
    newProgress: number
  ) => {
    setProgress(animeId, status, newProgress);
  };

  const { table } = useAnimeListDataGrid({
    listData,
    onProgressChange: handleProgressChange
  });

  return (
    <Box sx={{ height: '100%', width: '100%', minHeight: 0 }}>
      <MaterialReactTable table={table} />
    </Box>
  );
};

export default AnimeListDataGrid;
