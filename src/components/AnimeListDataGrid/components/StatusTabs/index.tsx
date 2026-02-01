import { Tab, Tabs } from '@mui/material';
import { FC } from 'react';

import { useAnimeListDataGridStore } from '@/stores/animeListDataGrid';

interface StatusTabsProps {
  watchingCount?: number;
  completedCount?: number;
  onHoldCount?: number;
  droppedCount?: number;
  planToWatchCount?: number;
}

const StatusTabs: FC<StatusTabsProps> = ({
  watchingCount = 0,
  completedCount = 0,
  onHoldCount = 0,
  droppedCount = 0,
  planToWatchCount = 0
}) => {
  const selectedStatus = useAnimeListDataGridStore(
    (state) => state.selectedStatus
  );
  const setSelectedStatus = useAnimeListDataGridStore(
    (state) => state.setSelectedStatus
  );

  return (
    <div className="w-full overflow-hidden">
      <Tabs
        className="justify-self-start"
        value={selectedStatus}
        variant="fullWidth"
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
  );
};

export default StatusTabs;
