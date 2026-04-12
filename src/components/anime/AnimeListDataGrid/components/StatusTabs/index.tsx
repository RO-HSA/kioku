import { Tab, Tabs } from '@mui/material';
import { FC, useMemo } from 'react';

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

  const isLoading = useAnimeListDataGridStore((state) => state.isLoading);

  const { tabsRootRef } = useStatusTabs();

  const tabs = useMemo(
    () => [
      {
        label: `Watching (${watchingCount})`,
        ariaLabel: 'Watching list',
        value: 'watching'
      },
      {
        label: `Completed (${completedCount})`,
        ariaLabel: 'Completed list',
        value: 'completed'
      },
      {
        label: `On Hold (${onHoldCount})`,
        ariaLabel: 'On Hold list',
        value: 'onHold'
      },
      {
        label: `Dropped (${droppedCount})`,
        ariaLabel: 'Dropped list',
        value: 'dropped'
      },
      {
        label: `Plan To Watch (${planToWatchCount})`,
        ariaLabel: 'Plan To Watch list',
        value: 'planToWatch'
      }
    ],
    [watchingCount, completedCount, onHoldCount, droppedCount, planToWatchCount]
  );

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
        {tabs.map((tab) => (
          <Tab
            key={tab.value}
            className={tabClassNames}
            label={tab.label}
            aria-label={tab.ariaLabel}
            value={tab.value}
            disabled={isLoading}
          />
        ))}
      </Tabs>
    </div>
  );
};

export default StatusTabs;
