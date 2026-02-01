import { Box } from '@mui/material';
import { MaterialReactTable } from 'material-react-table';

import { SynchronizedAnimeList } from '@/services/backend/types';
import useAnimeListDataGrid from './hooks/useAnimeListDataGrid';

interface AnimeListDataGridProps {
  listData: SynchronizedAnimeList | null;
}

const AnimeListDataGrid = ({ listData }: AnimeListDataGridProps) => {
  const { table } = useAnimeListDataGrid({ listData });

  return (
    <Box sx={{ height: '100%', width: '100%' }}>
      <MaterialReactTable table={table} />
    </Box>
  );
};

export default AnimeListDataGrid;
