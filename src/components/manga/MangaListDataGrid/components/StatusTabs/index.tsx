import { Tab, Tabs } from '@mui/material';
import { FC, useMemo } from 'react';

import { useMangaListDataGridStore } from '@/stores/mangaListDataGrid';
import useStatusTabs from './useStatusTabs';

interface StatusTabsProps {
  readingCount?: number;
  completedCount?: number;
  onHoldCount?: number;
  droppedCount?: number;
  planToReadCount?: number;
}

const tabClassNames = 'text-xs! pt-2! px-2!';

const StatusTabs: FC<StatusTabsProps> = ({
  readingCount = 0,
  completedCount = 0,
  onHoldCount = 0,
  droppedCount = 0,
  planToReadCount = 0
}) => {
  const selectedStatus = useMangaListDataGridStore(
    (state) => state.selectedStatus
  );
  const setSelectedStatus = useMangaListDataGridStore(
    (state) => state.setSelectedStatus
  );

  const { tabsRootRef } = useStatusTabs();

  const tabs = useMemo(
    () => [
      {
        label: `Reading (${readingCount})`,
        ariaLabel: 'Reading list',
        value: 'reading'
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
        label: `Plan To Read (${planToReadCount})`,
        ariaLabel: 'Plan To Read list',
        value: 'planToRead'
      }
    ],
    [readingCount, completedCount, onHoldCount, droppedCount, planToReadCount]
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
            className={tabClassNames}
            label={tab.label}
            aria-label={tab.ariaLabel}
            value={tab.value}
          />
        ))}
      </Tabs>
    </div>
  );
};

export default StatusTabs;
