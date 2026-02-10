import { Box, SelectChangeEvent, Tooltip } from '@mui/material';
import { SquareCheck, SquarePlay, SquareStop } from 'lucide-react';
import { MRT_ColumnDef, useMaterialReactTable } from 'material-react-table';
import { useCallback, useEffect, useMemo, useState } from 'react';

import ProgressStatus from '@/components/AnimeListDataGrid/components/ProgressStatus';
import { SynchronizedAnimeList } from '@/services/backend/types';
import { useAnimeDetailsStore } from '@/stores/animeDetails';
import { useAnimeListDataGridStore } from '@/stores/animeListDataGrid';
import { useMyAnimeListStore } from '@/stores/providers/myanimelist';
import {
  AnimeListStatus,
  AnimeListUserStatus,
  IAnimeList
} from '@/types/AnimeList';
import ScoreSelect from '../../ScoreSelect';
import CustomTopToolbar from '../components/CustomTopToolbar';
import MediaType from '../components/MediaType';
import StartSeason from '../components/StartSeason';
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

  const selectedUserStatus = useAnimeListDataGridStore(
    (state) => state.selectedStatus
  );
  const searchValue = useAnimeListDataGridStore((state) => state.searchValue);
  const openAnimeDetails = useAnimeDetailsStore((state) => state.setIsOpen);
  const setSelectedAnime = useAnimeDetailsStore(
    (state) => state.setSelectedAnime
  );

  const setScore = useMyAnimeListStore((state) => state.setScore);

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
              status={row.original.userStatus}
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
      density: 'compact'
    },
    mrtTheme,
    muiTablePaperProps,
    muiTableContainerProps,
    muiTableHeadCellProps,
    muiTableBodyCellProps: ({ cell }) => ({
      ...muiTableBodyCellProps,
      onDoubleClick: () => handleOpenAnimeDetails(cell.row.original)
    }),
    muiTableBodyRowProps,
    muiTopToolbarProps,
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
    state: { isLoading, globalFilter: searchValue },
    rowVirtualizerOptions: { overscan: 5 }
  });

  return { table };
};

export default useAnimeListDataGrid;
