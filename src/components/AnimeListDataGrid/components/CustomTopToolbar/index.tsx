import {
  MRT_ShowHideColumnsButton,
  MRT_TableInstance
} from 'material-react-table';
import { FC } from 'react';

import { IAnimeList, SynchronizedAnimeList } from '@/services/backend/types';
import { Box } from '@mui/material';
import RefreshListButton from '../RefreshListButton';
import SearchButton from '../SearchButton';
import StatusTabs from '../StatusTabs';

interface CustomTopToolbarProps {
  listData: SynchronizedAnimeList | null;
  table: MRT_TableInstance<IAnimeList>;
}

const CustomTopToolbar: FC<CustomTopToolbarProps> = ({ listData, table }) => {
  return (
    <Box className="flex justify-between px-3 w-full items-center overflow-hidden">
      <StatusTabs
        watchingCount={listData?.watching.length || 0}
        completedCount={listData?.completed.length || 0}
        onHoldCount={listData?.onHold.length || 0}
        droppedCount={listData?.dropped.length || 0}
        planToWatchCount={listData?.planToWatch.length || 0}
      />
      <SearchButton />
      <RefreshListButton />
      <MRT_ShowHideColumnsButton table={table} />
    </Box>
  );
};

export default CustomTopToolbar;
