import { Box, SelectChangeEvent, Tooltip } from '@mui/material';
import { SquareCheck, SquarePlay, SquareStop } from 'lucide-react';
import { MRT_ColumnDef, useMaterialReactTable } from 'material-react-table';
import { useCallback, useEffect, useMemo, useState } from 'react';

import ProgressStatus from '@/components/AnimeListDataGrid/components/ProgressStatus';
import { SynchronizedAnimeList } from '@/services/backend/types';
import { useAnimeDetailsStore } from '@/stores/animeDetails';
import { useAnimeListDataGridStore } from '@/stores/animeListDataGrid';
import { useAniListStore } from '@/stores/providers/anilist';
import { useMyAnimeListStore } from '@/stores/providers/myanimelist';
import { useProviderStore } from '@/stores/providers/provider';
import {
  AnimeListStatus,
  AnimeListUserStatus,
  IAnimeList
} from '@/types/AnimeList';
import { Provider } from '@/types/List';
import ScoreSelect from '../../ScoreSelect';
import CustomTopToolbar from '../components/CustomTopToolbar';
import MediaType from '../components/MediaType';
import StartSeason from '../components/StartSeason';
import useMaterialTableTheme from './useMaterialTableTheme';

interface UseAnimeListDataGridProps {
  listData: SynchronizedAnimeList | null;
}

const useAnimeListDataGrid = ({ listData }: UseAnimeListDataGridProps) => {
  const [isLoading, setIsLoading] = useState(true);

  const selectedUserStatus = useAnimeListDataGridStore(
    (state) => state.selectedStatus
  );
  const activeProvider = useProviderStore((state) => state.activeProvider);
  const searchValue = useAnimeListDataGridStore((state) => state.searchValue);
  const openAnimeDetails = useAnimeDetailsStore((state) => state.setIsOpen);
  const setSelectedAnime = useAnimeDetailsStore(
    (state) => state.setSelectedAnime
  );

  const setMyAnimeListScore = useMyAnimeListStore((state) => state.setScore);
  const setMyAnimeListProgress = useMyAnimeListStore(
    (state) => state.setProgress
  );
  const setAniListScore = useAniListStore((state) => state.setScore);
  const setAniListProgress = useAniListStore((state) => state.setProgress);

  const setScore = useCallback(
    (animeId: number, status: AnimeListUserStatus, newScore: number) => {
      switch (activeProvider) {
        case Provider.MY_ANIME_LIST:
          return setMyAnimeListScore(animeId, status, newScore);
        case Provider.ANILIST:
          return setAniListScore(animeId, status, newScore);
        default:
          return () => {};
      }
    },
    [activeProvider, setAniListScore, setMyAnimeListScore]
  );

  const handleProgressChange = useCallback(
    (animeId: number, status: AnimeListUserStatus, newProgress: number) => {
      switch (activeProvider) {
        case Provider.MY_ANIME_LIST:
          return setMyAnimeListProgress(animeId, status, newProgress);
        case Provider.ANILIST:
          return setAniListProgress(animeId, status, newProgress);
        default:
          return () => {};
      }
    },
    [activeProvider, setMyAnimeListProgress, setAniListProgress]
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

  const getUserStatusLabel = (status: AnimeListUserStatus) => {
    switch (status) {
      case 'watching':
        return 'Watching';
      case 'completed':
        return 'Completed';
      case 'onHold':
        return 'On Hold';
      case 'dropped':
        return 'Dropped';
      case 'planToWatch':
        return 'Plan To Watch';
      default:
        return status;
    }
  };

  const columns = useMemo<MRT_ColumnDef<IAnimeList>[]>(
    () => [
      {
        accessorKey: 'userStatus',
        header: 'List',
        size: 60,
        enableSorting: false,
        enableGlobalFilter: false,
        getGroupingValue: (row) => getUserStatusLabel(row.userStatus),
        Cell: ({ cell }) => {
          const value = cell.getValue<AnimeListUserStatus>();
          return value ? getUserStatusLabel(value) : '';
        }
      },
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
        enableGlobalFilter: false,
        Cell: ({ cell, row }) => {
          const watched = cell.getValue<number>();
          return (
            <ProgressStatus
              progress={watched}
              total={row.original.totalEpisodes}
              startDate={row.original.startDate}
              broadcast={row.original.broadcast}
              status={row.original.userStatus}
              onProgressChange={(newProgress) => {
                handleProgressChange(
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
        enableGlobalFilter: false,
        Cell: ({ cell, row }) => {
          const score = cell.getValue<number>();

          const handleChange = (event: SelectChangeEvent<number>) => {
            const { value } = event.target;

            setScore(row.original.id, row.original.userStatus, value);
          };

          return (
            <Box width="100%">
              <ScoreSelect fullWidth score={score} onChange={handleChange} />
            </Box>
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
    [handleProgressChange, setScore]
  );

  const allData = useMemo(() => {
    if (!listData) {
      return [];
    }

    return [
      ...listData.watching,
      ...listData.completed,
      ...listData.onHold,
      ...listData.dropped,
      ...listData.planToWatch
    ];
  }, [listData]);

  const shouldGroupByStatus = searchValue.trim().length > 0;

  const data = useMemo(() => {
    if (shouldGroupByStatus) {
      return allData;
    }

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
  }, [allData, listData, selectedUserStatus, shouldGroupByStatus]);

  const grouping = useMemo(
    () => (shouldGroupByStatus ? ['userStatus'] : []),
    [shouldGroupByStatus]
  );

  const handleOpenAnimeDetails = useCallback(
    (anime: IAnimeList) => {
      setSelectedAnime(anime);
      openAnimeDetails(true);
    },
    [openAnimeDetails, setSelectedAnime]
  );

  useEffect(() => {
    if (typeof window !== 'undefined') {
      setIsLoading(false);
    }
  }, []);

  const table = useMaterialReactTable({
    columns,
    data,
    initialState: {
      density: 'compact',
      columnVisibility: {
        userStatus: false
      },
      expanded: true
    },
    mrtTheme,
    muiTablePaperProps,
    muiTableContainerProps,
    muiTableHeadCellProps,
    muiTableBodyRowProps,
    muiTopToolbarProps,
    muiTableBodyCellProps: ({ cell }) => ({
      ...muiTableBodyCellProps,
      onDoubleClick: () => {
        if (cell.row.getIsGrouped()) {
          return;
        }

        handleOpenAnimeDetails(cell.row.original);
      }
    }),
    renderTopToolbar: () => (
      <CustomTopToolbar listData={listData} table={table} />
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
    enableGrouping: true,
    enableColumnOrdering: false,
    enableColumnPinning: false,
    enableColumnDragging: false,
    globalFilterFn: 'includesString',
    groupedColumnMode: 'remove',
    state: {
      isLoading,
      globalFilter: searchValue,
      grouping
    },
    rowVirtualizerOptions: { overscan: 5 }
  });

  return { table };
};

export default useAnimeListDataGrid;
