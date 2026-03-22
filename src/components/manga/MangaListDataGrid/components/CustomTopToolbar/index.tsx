import { Box } from '@mui/material';
import {
  MRT_ShowHideColumnsButton,
  MRT_TableInstance
} from 'material-react-table';
import { FC } from 'react';

import { SynchronizedMangaList } from '@/services/backend/types';
import { IMangaList } from '@/types/MangaList';
import RefreshListButton from '../RefreshListButton';
import SearchButton from '../SearchButton';
import StatusTabs from '../StatusTabs';

interface CustomTopToolbarProps {
  listData: SynchronizedMangaList | null;
  table: MRT_TableInstance<IMangaList>;
}

const CustomTopToolbar: FC<CustomTopToolbarProps> = ({ listData, table }) => {
  return (
    <Box className="flex justify-between px-3 w-full items-center overflow-hidden">
      <StatusTabs
        readingCount={listData?.reading.length || 0}
        completedCount={listData?.completed.length || 0}
        onHoldCount={listData?.onHold.length || 0}
        droppedCount={listData?.dropped.length || 0}
        planToReadCount={listData?.planToRead.length || 0}
      />
      <SearchButton />
      <RefreshListButton />
      <MRT_ShowHideColumnsButton table={table} />
    </Box>
  );
};

export default CustomTopToolbar;
