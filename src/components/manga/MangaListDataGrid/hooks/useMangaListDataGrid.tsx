import { Box, SelectChangeEvent, Tooltip } from '@mui/material';
import { SquareCheck, SquarePlay, SquareStop } from 'lucide-react';
import { MRT_ColumnDef, useMaterialReactTable } from 'material-react-table';
import { useCallback, useEffect, useMemo } from 'react';

import GroupedExpandCell from '@/components/ui/GroupedExpandCell';
import useMaterialTableTheme from '@/hooks/useMaterialTableTheme';
import { PathName } from '@/routes';
import { SynchronizedMangaList } from '@/services/backend/types';
import { useMangaDetailsStore } from '@/stores/mangaDetails';
import { useMangaListDataGridStore } from '@/stores/mangaListDataGrid';
import { useAniListStore } from '@/stores/providers/anilist';
import { useMyAnimeListStore } from '@/stores/providers/myanimelist';
import { useProviderStore } from '@/stores/providers/provider';
import { Provider } from '@/types/List';
import {
  IMangaList,
  MangaListStatus,
  MangaListUserStatus
} from '@/types/MangaList';
import { useLocation } from 'react-router';
import ScoreSelect from '../../../ScoreSelect';
import CustomTopToolbar from '../components/CustomTopToolbar';
import MediaType from '../components/MediaType';
import ProgressStatus from '../components/ProgressStatus';

interface UseMangaListDataGridProps {
  listData: SynchronizedMangaList | null;
}

const useMangaListDataGrid = ({ listData }: UseMangaListDataGridProps) => {
  const location = useLocation();

  const isSearchPage = location.pathname === PathName.SEARCH;

  const selectedUserStatus = useMangaListDataGridStore(
    (state) => state.selectedStatus
  );
  const localSearchValue = useMangaListDataGridStore(
    (state) => state.localSearchValue
  );
  const sorting = useMangaListDataGridStore((state) => state.sorting);
  const columnVisibility = useMangaListDataGridStore(
    (state) => state.columnVisibility
  );
  const columnSizing = useMangaListDataGridStore((state) => state.columnSizing);
  const isLoading = useMangaListDataGridStore((state) => state.isLoading);
  const setIsLoading = useMangaListDataGridStore((state) => state.setIsloading);
  const onSortingChange = useMangaListDataGridStore(
    (state) => state.onSortingChange
  );
  const onColumnVisibilityChange = useMangaListDataGridStore(
    (state) => state.onColumnVisibilityChange
  );
  const onColumnSizingChange = useMangaListDataGridStore(
    (state) => state.onColumnSizingChange
  );

  const openMangaDetails = useMangaDetailsStore((state) => state.setIsOpen);
  const setSelectedManga = useMangaDetailsStore(
    (state) => state.setSelectedManga
  );

  const activeProvider = useProviderStore((state) => state.activeProvider);

  const mangaSearchResults = useMyAnimeListStore(
    (state) => state.mangaSearchResults
  );
  const aniListMangaSearchResults = useAniListStore(
    (state) => state.mangaSearchResults
  );
  const setMyAnimeListScore = useMyAnimeListStore((state) => state.setScore);
  const setMyAnimeListProgress = useMyAnimeListStore(
    (state) => state.setProgress
  );

  const setAniListScore = useAniListStore((state) => state.setScore);
  const setAniListProgress = useAniListStore((state) => state.setProgress);

  const setScore = useCallback(
    (entryId: number, status: MangaListUserStatus, newScore: number) => {
      switch (activeProvider) {
        case Provider.MY_ANIME_LIST:
          return setMyAnimeListScore(
            entryId,
            { type: 'manga', value: status },
            newScore
          );
        case Provider.ANILIST:
          return setAniListScore(
            entryId,
            { type: 'manga', value: status },
            newScore
          );
        default:
          return () => {};
      }
    },
    [activeProvider, setAniListScore, setMyAnimeListScore]
  );

  const handleChaptersProgressChange = useCallback(
    (entryId: number, status: MangaListUserStatus, newProgress: number) => {
      switch (activeProvider) {
        case Provider.MY_ANIME_LIST:
          return setMyAnimeListProgress(
            entryId,
            { type: 'manga', value: status },
            newProgress,
            'chapters'
          );
        case Provider.ANILIST:
          return setAniListProgress(
            entryId,
            { type: 'manga', value: status },
            newProgress,
            'chapters'
          );
        default:
          return () => {};
      }
    },
    [activeProvider, setMyAnimeListProgress, setAniListProgress]
  );

  const handleVolumesProgressChange = useCallback(
    (entryId: number, status: MangaListUserStatus, newProgress: number) => {
      switch (activeProvider) {
        case Provider.MY_ANIME_LIST:
          return setMyAnimeListProgress(
            entryId,
            { type: 'manga', value: status },
            newProgress,
            'volumes'
          );
        case Provider.ANILIST:
          return setAniListProgress(
            entryId,
            { type: 'manga', value: status },
            newProgress,
            'volumes'
          );
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

  const getStatusIcon = (status: MangaListStatus) => {
    switch (status) {
      case 'Currently Publishing':
        return <SquarePlay className="text-green-500" />;
      case 'Finished':
        return <SquareCheck className="text-blue-500" />;
      case 'Not Yet Published':
        return <SquareStop className="text-red-500" />;
      default:
        return <SquarePlay className="text-green-500" />;
    }
  };

  const getUserStatusLabel = (status: MangaListUserStatus) => {
    switch (status) {
      case 'reading':
        return 'Reading';
      case 'completed':
        return 'Completed';
      case 'onHold':
        return 'On Hold';
      case 'dropped':
        return 'Dropped';
      case 'planToRead':
        return 'Plan To Read';
      default:
        return status;
    }
  };

  const allData = useMemo(() => {
    if (!listData) {
      return [];
    }

    return [
      ...listData.reading,
      ...listData.completed,
      ...listData.onHold,
      ...listData.dropped,
      ...listData.planToRead
    ];
  }, [listData]);

  const mangaListDataById = useMemo(
    () => new Map(allData.map((manga) => [manga.id, manga])),
    [allData]
  );

  const getSearchMembershipLabel = useCallback(
    (manga: IMangaList) =>
      mangaListDataById.has(manga.id) ? 'In list' : 'Not in list',
    [mangaListDataById]
  );

  const columns = useMemo<MRT_ColumnDef<IMangaList>[]>(
    () => [
      {
        accessorKey: 'userStatus',
        header: 'List',
        size: 60,
        enableSorting: false,
        enableGlobalFilter: false,
        visibleInShowHideMenu: false,
        getGroupingValue: (row) =>
          isSearchPage
            ? getSearchMembershipLabel(row)
            : getUserStatusLabel(row.userStatus),
        Cell: ({ cell }) => {
          const value = cell.getValue<MangaListUserStatus>();
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
            'Currently Publishing',
            'Finished',
            'Not Yet Published'
          ];

          const v1 = rowA.getValue<MangaListStatus>(columnId);
          const v2 = rowB.getValue<MangaListStatus>(columnId);

          return order.indexOf(v1) - order.indexOf(v2);
        },
        Cell: ({ cell }) => {
          return (
            <div className="flex justify-center items-center h-full w-full">
              <Tooltip title={cell.getValue<MangaListStatus>() || ''}>
                {getStatusIcon(cell.getValue<MangaListStatus>())}
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
        accessorKey: 'userChaptersRead',
        header: 'Chapters',
        size: 150,
        enableGlobalFilter: false,
        visibleInShowHideMenu: !isSearchPage,
        Cell: ({ cell, row }) => {
          const read = cell.getValue<number>();
          return (
            <ProgressStatus
              progress={read}
              total={row.original.totalChapters}
              status={row.original.userStatus}
              onProgressChange={(newProgress) => {
                handleChaptersProgressChange(
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
        accessorKey: 'userVolumesRead',
        header: 'Volumes',
        size: 150,
        enableGlobalFilter: false,
        visibleInShowHideMenu: !isSearchPage,
        Cell: ({ cell, row }) => {
          const read = cell.getValue<number>();
          return (
            <ProgressStatus
              progress={read}
              total={row.original.totalVolumes}
              status={row.original.userStatus}
              onProgressChange={(newProgress) => {
                handleVolumesProgressChange(
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
        accessorKey: 'score',
        header: 'Score',
        size: 85,
        enableGlobalFilter: false,
        visibleInShowHideMenu: isSearchPage,
        Cell: ({ cell }) => {
          const score = cell.getValue<number>();

          return (
            <Box display="flex" justifyContent="center" width="100%">
              {score === 0 ? (
                <span className="text-gray-500">{'N/A'}</span>
              ) : (
                score
              )}
            </Box>
          );
        }
      },
      {
        accessorKey: 'userScore',
        header: 'Score',
        size: 85,
        enableGlobalFilter: false,
        visibleInShowHideMenu: !isSearchPage,
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
        accessorKey: 'genres',
        header: 'Genres',
        size: 200,
        Cell: ({ cell }) => {
          const value = cell.getValue<string[]>();
          return (
            <Tooltip title={value}>
              <span className="truncate text-ellipsis">{value}</span>
            </Tooltip>
          );
        }
      }
    ],
    [
      getSearchMembershipLabel,
      isSearchPage,
      handleVolumesProgressChange,
      handleChaptersProgressChange,
      setScore
    ]
  );

  const shouldGroupByStatus = localSearchValue.trim().length > 0;

  const data = useMemo(() => {
    if (shouldGroupByStatus) {
      return allData;
    }

    switch (selectedUserStatus) {
      case 'reading':
        return listData?.reading || [];
      case 'completed':
        return listData?.completed || [];
      case 'onHold':
        return listData?.onHold || [];
      case 'dropped':
        return listData?.dropped || [];
      case 'planToRead':
        return listData?.planToRead || [];
      default:
        return [];
    }
  }, [allData, listData, selectedUserStatus, shouldGroupByStatus]);

  const searchResults = useMemo(() => {
    const results = (() => {
      switch (activeProvider) {
        case Provider.MY_ANIME_LIST:
          return mangaSearchResults || [];

        case Provider.ANILIST:
          return aniListMangaSearchResults || [];
        default:
          return [];
      }
    })();

    return results
      .map((manga) => mangaListDataById.get(manga.id) || manga)
      .sort(
        (a, b) =>
          Number(mangaListDataById.has(a.id)) -
          Number(mangaListDataById.has(b.id))
      );
  }, [
    activeProvider,
    aniListMangaSearchResults,
    mangaListDataById,
    mangaSearchResults
  ]);

  const grouping = useMemo(
    () =>
      isSearchPage ? ['userStatus'] : shouldGroupByStatus ? ['userStatus'] : [],
    [isSearchPage, shouldGroupByStatus]
  );

  const handleOpenMangaDetails = useCallback(
    (manga: IMangaList) => {
      setSelectedManga(manga);
      openMangaDetails(true);
    },
    [openMangaDetails, setSelectedManga]
  );

  useEffect(() => {
    if (typeof window !== 'undefined') {
      setIsLoading(false);
    }
  }, []);

  const table = useMaterialReactTable({
    columns,
    data: isSearchPage ? searchResults : data,
    initialState: {
      density: 'compact',
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

        handleOpenMangaDetails(cell.row.original);
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
    displayColumnDefOptions: {
      'mrt-row-expand': {
        grow: false,
        size: 140,
        Cell: ({ row, staticRowIndex, table }) => (
          <GroupedExpandCell
            row={row}
            staticRowIndex={staticRowIndex}
            table={table}
          />
        )
      }
    },
    state: {
      isLoading,
      globalFilter: localSearchValue,
      grouping,
      sorting,
      columnVisibility: {
        ...columnVisibility,
        ...(isSearchPage
          ? {
              userChaptersRead: false,
              userVolumesRead: false,
              userScore: false
            }
          : { score: false })
      },
      columnSizing
    },
    onSortingChange,
    onColumnVisibilityChange,
    onColumnSizingChange,
    rowVirtualizerOptions: { overscan: 5 }
  });

  return { table };
};

export default useMangaListDataGrid;
