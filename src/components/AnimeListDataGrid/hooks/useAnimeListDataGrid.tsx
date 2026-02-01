import { GridColDef } from '@mui/x-data-grid';
import { SquareCheck, SquarePlay, SquareStop } from 'lucide-react';
import { useMemo } from 'react';

import {
  AnimeListStatus,
  SynchronizedAnimeList
} from '@/services/backend/types';
import { useAnimeListDataGridStore } from '@/stores/animeListDataGrid';
import { Tooltip } from '@mui/material';

interface UseAnimeListDataGridProps {
  listData: SynchronizedAnimeList | null;
}

const useAnimeListDataGrid = ({ listData }: UseAnimeListDataGridProps) => {
  const selectedUserStatus = useAnimeListDataGridStore(
    (state) => state.selectedStatus
  );

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

  const columns: GridColDef[] = [
    {
      field: 'status',
      headerName: '',
      disableColumnMenu: true,
      width: 50,
      hideable: false,
      type: 'custom',
      getSortComparator(sortDirection) {
        const modifier = sortDirection === 'desc' ? -1 : 1;

        return (v1, v2) => {
          const order = [
            'Currently Airing',
            'Finished Airing',
            'Not Yet Aired'
          ];

          return (
            (order.indexOf(v1 as AnimeListStatus) -
              order.indexOf(v2 as AnimeListStatus)) *
            modifier
          );
        };
      },
      renderCell: (params) => {
        return (
          <div className="flex justify-center items-center h-full w-full">
            <Tooltip title={params.value as AnimeListStatus}>
              {getStatusIcon(params.value as AnimeListStatus)}
            </Tooltip>
          </div>
        );
      }
    },
    { field: 'title', headerName: 'Title', width: 250 },
    { field: 'progress', headerName: 'Progress', width: 150 },
    {
      field: 'userScore',
      headerName: 'Score',
      type: 'singleSelect',
      valueOptions: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
      valueFormatter: (value) => value || '-'
    },
    { field: 'mediaType', headerName: 'Type' },
    {
      field: 'startSeason',
      headerName: 'Season',
      valueFormatter: (value: string) =>
        value.charAt(0).toUpperCase() + value.slice(1)
    }
  ];

  const rows = useMemo(() => {
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

  return { columns, rows };
};

export default useAnimeListDataGrid;
