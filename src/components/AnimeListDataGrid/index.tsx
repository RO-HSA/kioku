import { Box } from '@mui/material';
import { MaterialReactTable } from 'material-react-table';

import { useMyAnimeListStore } from '@/stores/providers/myanimelist';
import { AnimeListUserStatus } from '@/types/AnimeList';
import useAnimeListDataGrid from './hooks/useAnimeListDataGrid';

const AnimeListDataGrid = () => {
  const animeListData = useMyAnimeListStore((state) => state.animeListData);
  const setProgress = useMyAnimeListStore((state) => state.setProgress);

  const handleProgressChange = (
    animeId: number,
    status: AnimeListUserStatus,
    newProgress: number
  ) => {
    setProgress(animeId, status, newProgress);
  };

  const { table } = useAnimeListDataGrid({
    listData: animeListData,
    onProgressChange: handleProgressChange
  });

  return (
    <Box sx={{ height: '100%', width: '100%', minHeight: 0 }}>
      <MaterialReactTable table={table} />
    </Box>
  );
};

export default AnimeListDataGrid;
