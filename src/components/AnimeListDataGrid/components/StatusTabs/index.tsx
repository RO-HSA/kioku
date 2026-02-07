import { Tab, Tabs } from '@mui/material';
import { FC } from 'react';

import { useAnimeListDataGridStore } from '@/stores/animeListDataGrid';
import useStatusTabs from './useStatusTabs';

interface StatusTabsProps {
  watchingCount?: number;
  completedCount?: number;
  onHoldCount?: number;
  droppedCount?: number;
  planToWatchCount?: number;
}

const tabClassNames = 'text-xs! pt-2! px-2!';

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

  const { tabsRootRef } = useStatusTabs();

  return (
    <div className="w-full overflow-x-hidden">
      <Tabs
        ref={tabsRootRef}
        className="w-full min-w-0"
        value={selectedStatus}
        variant="scrollable"
        scrollButtons={false}
        sx={{
          width: '100%',
          minWidth: 0,
          '& .MuiTabs-scroller': {
            cursor: 'grab',
            width: '100%',
            overflowX: 'auto',
            scrollbarWidth: 'none',
            touchAction: 'pan-x pan-y',
            WebkitOverflowScrolling: 'touch',
            '&::-webkit-scrollbar': { display: 'none' }
          },
          '& .MuiTabs-flexContainer': {
            flexWrap: 'nowrap'
          },
          '& .MuiTabs-scroller[data-dragging="true"]': {
            cursor: 'grabbing',
            userSelect: 'none'
          }
        }}
        onChange={(_, value) => setSelectedStatus(value)}>
        <Tab
          className={tabClassNames}
          label={`Watching (${watchingCount})`}
          aria-label="Watching list"
          value="watching"
        />
        <Tab
          className={tabClassNames}
          label={`Completed (${completedCount})`}
          aria-label="Completed list"
          value="completed"
        />
        <Tab
          className={tabClassNames}
          label={`On Hold (${onHoldCount})`}
          aria-label="On Hold list"
          value="onHold"
        />
        <Tab
          className={tabClassNames}
          label={`Dropped (${droppedCount})`}
          aria-label="Dropped list"
          value="dropped"
        />
        <Tab
          className={tabClassNames}
          label={`Plan To Watch (${planToWatchCount})`}
          aria-label="Plan To Watch list"
          value="planToWatch"
        />
      </Tabs>
    </div>
  );
};

export default StatusTabs;
