import { IconButton, Tooltip } from '@mui/material';
import {
  LoaderCircle,
  RefreshCw,
  SquareCheck,
  SquarePlay,
  SquareStop
} from 'lucide-react';
import {
  MRT_ColumnDef,
  MRT_ShowHideColumnsButton,
  useMaterialReactTable
} from 'material-react-table';
import { useCallback, useEffect, useMemo, useState } from 'react';

import ProgressStatus from '@/components/AnimeListDataGrid/components/ProgressStatus';
import { MyAnimeListService } from '@/services/backend/MyAnimeList';
import {
  AnimeListStatus,
  AnimeListUserStatus,
  IAnimeList,
  SynchronizedAnimeList
} from '@/services/backend/types';
import { useAnimeListDataGridStore } from '@/stores/animeListDataGrid';
import { useMyAnimeListStore } from '@/stores/providers/myanimelist';
import MediaType from '../components/MediaType';
import ScoreSelect from '../components/ScoreSelect';
import StartSeason from '../components/StartSeason';
import StatusTabs from '../components/StatusTabs';
import useMaterialTableTheme from './useMaterialTableTheme';

interface UseAnimeListDataGridProps {
  listData: SynchronizedAnimeList | null;
  onProgressChange: (
    animeId: number,
    status: AnimeListUserStatus,
    newProgress: number
  ) => void;
}

const useAnimeListDataGrid = ({
  listData,
  onProgressChange
}: UseAnimeListDataGridProps) => {
  const [isLoading, setIsLoading] = useState(true);
  const [isRefreshing, setIsRefreshing] = useState(false);

  const selectedUserStatus = useAnimeListDataGridStore(
    (state) => state.selectedStatus
  );

  const setAnimeListData = useMyAnimeListStore(
    (state) => state.setAnimeListData
  );

  const {
    mrtTheme,
    muiTablePaperProps,
    muiTableContainerProps,
    muiTableHeadCellProps,
    muiTableBodyCellProps,
    muiTableBodyRowProps,
    muiTopToolbarProps
  } = useMaterialTableTheme();

  const getStatusIcon = (status: AnimeListStatus) => {
    switch (status) {
      case 'Currently Airing':
        return <SquarePlay className="text-green-500" />;
      case 'Finished Airing':
        return <SquareCheck className="text-blue-500" />;
      case 'Not Yet Aired':
        return <SquareStop className="text-red-500" />;
      default:
        return <SquarePlay className="text-green-500" />;
    }
  };

  const columns = useMemo<MRT_ColumnDef<IAnimeList>[]>(
    () => [
      {
        accessorKey: 'status',
        header: '',
        size: 40,
        enableResizing: false,
        enableHiding: false,
        sortingFn: (rowA, rowB, columnId) => {
          const order = [
            'Currently Airing',
            'Finished Airing',
            'Not Yet Aired'
          ];

          const v1 = rowA.getValue<AnimeListStatus>(columnId);
          const v2 = rowB.getValue<AnimeListStatus>(columnId);

          return order.indexOf(v1) - order.indexOf(v2);
        },
        Cell: ({ cell }) => {
          return (
            <div className="flex justify-center items-center h-full w-full">
              <Tooltip title={cell.getValue<AnimeListStatus>() || ''}>
                {getStatusIcon(cell.getValue<AnimeListStatus>())}
              </Tooltip>
            </div>
          );
        }
      },
      {
        accessorKey: 'title',
        header: 'Title',
        size: 270,
        Cell: ({ cell }) => {
          return (
            <Tooltip title={cell.getValue<string>() || ''}>
              <span className="truncate text-ellipsis">
                {cell.getValue<string>()}
              </span>
            </Tooltip>
          );
        }
      },
      {
        accessorKey: 'userEpisodesWatched',
        header: 'Progress',
        size: 200,
        Cell: ({ cell, row }) => {
          const watched = cell.getValue<number>();
          return (
            <ProgressStatus
              progress={watched}
              total={row.original.totalEpisodes}
              startDate={row.original.startDate}
              broadcast={row.original.broadcast}
              onProgressChange={(newProgress) => {
                onProgressChange(
                  row.original.id,
                  row.original.userStatus,
                  newProgress
                );
              }}
            />
          );
        }
      },
      {
        accessorKey: 'userScore',
        header: 'Score',
        size: 85,
        Cell: ({ cell, row }) => {
          const score = cell.getValue<number>();

          return (
            <ScoreSelect
              score={score}
              animeId={row.original.id}
              status={row.original.userStatus}
            />
          );
        }
      },
      {
        accessorKey: 'mediaType',
        header: 'Type',
        size: 70,
        Cell: ({ cell }) => {
          const value = cell.getValue<string>();
          return <MediaType mediaType={value} />;
        }
      },
      {
        accessorKey: 'startSeason',
        header: 'Season',
        size: 100,
        Cell: ({ cell }) => {
          const value = cell.getValue<string>();

          return <StartSeason startSeason={value} />;
        }
      }
    ],
    [onProgressChange]
  );

  const data = useMemo(() => {
    switch (selectedUserStatus) {
      case 'watching':
        return listData?.watching || [];
      case 'completed':
        return listData?.completed || [];
      case 'onHold':
        return listData?.onHold || [];
      case 'dropped':
        return listData?.dropped || [];
      case 'planToWatch':
        return listData?.planToWatch || [];
      default:
        return [];
    }
  }, [listData, selectedUserStatus]);

  useEffect(() => {
    if (typeof window !== 'undefined') {
      setIsLoading(false);
    }
  }, []);

  const handleRefresh = useCallback(async () => {
    setIsRefreshing(true);
    try {
      const result = await MyAnimeListService.synchronizeList();
      setAnimeListData(result);
    } finally {
      setIsRefreshing(false);
    }
  }, [setAnimeListData, setIsRefreshing]);

  const table = useMaterialReactTable({
    columns,
    data,
    initialState: {
      density: 'compact'
    },
    mrtTheme,
    muiTablePaperProps,
    muiTableContainerProps,
    muiTableHeadCellProps,
    muiTableBodyCellProps,
    muiTableBodyRowProps,
    muiTopToolbarProps,
    renderTopToolbarCustomActions: () => (
      <StatusTabs
        watchingCount={listData?.watching.length || 0}
        completedCount={listData?.completed.length || 0}
        onHoldCount={listData?.onHold.length || 0}
        droppedCount={listData?.dropped.length || 0}
        planToWatchCount={listData?.planToWatch.length || 0}
      />
    ),
    renderToolbarInternalActions: ({ table }) => (
      <>
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
        <MRT_ShowHideColumnsButton table={table} />
      </>
    ),
    enableStickyHeader: true,
    enableDensityToggle: false,
    enableColumnResizing: true,
    enableColumnFilters: false,
    enableGlobalFilter: true,
    enableFullScreenToggle: false,
    enablePagination: false,
    enableBottomToolbar: false,
    enableRowVirtualization: true,
    enableColumnActions: false,
    state: { isLoading },
    rowVirtualizerOptions: { overscan: 5 }
  });

  return { table, handleRefresh };
};

export default useAnimeListDataGrid;
