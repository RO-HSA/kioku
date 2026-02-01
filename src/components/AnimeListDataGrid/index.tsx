import { Box } from '@mui/material';
import { DataGrid } from '@mui/x-data-grid';
import { FC } from 'react';

import { SynchronizedAnimeList } from '@/services/backend/types';
import CustomToolbar from './components/CustomToolbar';
import useAnimeListDataGrid from './hooks/useAnimeListDataGrid';

interface AnimeListDataGridProps {
  listData: SynchronizedAnimeList | null;
}

const AnimeListDataGrid: FC<AnimeListDataGridProps> = ({ listData }) => {
  const { columns, rows } = useAnimeListDataGrid({ listData });

  const toolbar = () => {
    return (
      <CustomToolbar
        watchingCount={listData?.watching.length}
        completedCount={listData?.completed.length}
        onHoldCount={listData?.onHold.length}
        droppedCount={listData?.dropped.length}
        planToWatchCount={listData?.planToWatch.length}
      />
    );
  };

  return (
    <Box sx={{ height: '100%', width: '100%' }}>
      <DataGrid
        columns={columns}
        rows={rows}
        hideFooter={true}
        slots={{ toolbar }}
        showToolbar
      />
    </Box>
  );
};

export default AnimeListDataGrid;
