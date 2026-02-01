import { IconButton, Tab, Tabs, Tooltip } from '@mui/material';
import { ColumnsPanelTrigger, Toolbar, ToolbarButton } from '@mui/x-data-grid';
import { Columns4, LoaderCircle, RefreshCw } from 'lucide-react';
import { FC, useState } from 'react';

import { MyAnimeListService } from '@/services/backend/MyAnimeList';
import { useAnimeListDataGridStore } from '@/stores/animeListDataGrid';
import { useMyAnimeListStore } from '@/stores/config/providers/myanimelist';

interface CustomToolbarProps {
  watchingCount?: number;
  completedCount?: number;
  onHoldCount?: number;
  droppedCount?: number;
  planToWatchCount?: number;
}

const CustomToolbar: FC<CustomToolbarProps> = ({
  watchingCount = 0,
  completedCount = 0,
  onHoldCount = 0,
  droppedCount = 0,
  planToWatchCount = 0
}) => {
  const [isRefreshing, setIsRefreshing] = useState(false);

  const selectedStatus = useAnimeListDataGridStore(
    (state) => state.selectedStatus
  );
  const setSelectedStatus = useAnimeListDataGridStore(
    (state) => state.setSelectedStatus
  );

  const setAnimeListData = useMyAnimeListStore(
    (state) => state.setAnimeListData
  );

  const handleRefresh = async () => {
    setIsRefreshing(true);
    try {
      const result = await MyAnimeListService.synchronizeList();
      setAnimeListData(result);
    } finally {
      setIsRefreshing(false);
    }
  };

  return (
    <Toolbar>
      <div className="w-full overflow-hidden">
        <Tabs
          className="justify-self-start"
          value={selectedStatus}
          variant="scrollable"
          onChange={(_, value) => setSelectedStatus(value)}>
          <Tab
            className=""
            label={`Watching (${watchingCount})`}
            value="watching"
          />
          <Tab label={`Completed (${completedCount})`} value="completed" />
          <Tab label={`On Hold (${onHoldCount})`} value="onHold" />
          <Tab label={`Dropped (${droppedCount})`} value="dropped" />
          <Tab
            label={`Plan To Watch (${planToWatchCount})`}
            value="planToWatch"
          />
        </Tabs>
      </div>

      <Tooltip title="Refresh List">
        <IconButton disabled={isRefreshing} onClick={handleRefresh}>
          <span>
            {!isRefreshing ? (
              <RefreshCw className="text-primary" />
            ) : (
              <LoaderCircle className="text-primary animate-spin" />
            )}
          </span>
        </IconButton>
      </Tooltip>

      <Tooltip title="Columns">
        <ColumnsPanelTrigger render={<ToolbarButton />}>
          <Columns4 />
        </ColumnsPanelTrigger>
      </Tooltip>
    </Toolbar>
  );
};

export default CustomToolbar;
