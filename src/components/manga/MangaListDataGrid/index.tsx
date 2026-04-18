import { Box } from '@mui/material';
import { MaterialReactTable } from 'material-react-table';
import { useMemo } from 'react';

import ContextMenu from '@/components/ContextMenu';
import { useAniListStore } from '@/stores/providers/anilist';
import { useMyAnimeListStore } from '@/stores/providers/myanimelist';
import { useProviderStore } from '@/stores/providers/provider';
import { Provider } from '@/types/List';
import useMangaListDataGrid from './hooks/useMangaListDataGrid';

const MangaListDataGrid = () => {
  const myAnimeListData = useMyAnimeListStore((state) => state.mangaListData);
  const aniListData = useAniListStore((state) => state.mangaListData);

  const activeProvider = useProviderStore((state) => state.activeProvider);

  const mangaListDataByProvider = useMemo(() => {
    switch (activeProvider) {
      case Provider.MY_ANIME_LIST:
        return myAnimeListData;
      case Provider.ANILIST:
        return aniListData;
      default:
        return null;
    }
  }, [activeProvider, myAnimeListData, aniListData]);

  const { table } = useMangaListDataGrid({
    listData: mangaListDataByProvider
  });

  return (
    <Box sx={{ height: '100%', width: '100%', minHeight: 0 }}>
      <MaterialReactTable table={table} />

      <ContextMenu />
    </Box>
  );
};

export default MangaListDataGrid;
