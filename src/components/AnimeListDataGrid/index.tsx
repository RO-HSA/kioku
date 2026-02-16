import { Box } from '@mui/material';
import { MaterialReactTable } from 'material-react-table';
import { useMemo } from 'react';

import { useAniListStore } from '@/stores/providers/anilist';
import { useMyAnimeListStore } from '@/stores/providers/myanimelist';
import { useProviderStore } from '@/stores/providers/provider';
import { Provider } from '@/types/List';
import useAnimeListDataGrid from './hooks/useAnimeListDataGrid';

const AnimeListDataGrid = () => {
  const myAnimeListData = useMyAnimeListStore((state) => state.animeListData);
  const aniListData = useAniListStore((state) => state.animeListData);
  const activeProvider = useProviderStore((state) => state.activeProvider);

  const animeListDataByProvider = useMemo(() => {
    switch (activeProvider) {
      case Provider.MY_ANIME_LIST:
        return myAnimeListData;
      case Provider.ANILIST:
        return aniListData;
      default:
        return null;
    }
  }, [activeProvider, myAnimeListData, aniListData]);

  const { table } = useAnimeListDataGrid({
    listData: animeListDataByProvider
  });

  return (
    <Box sx={{ height: '100%', width: '100%', minHeight: 0 }}>
      <MaterialReactTable table={table} />
    </Box>
  );
};

export default AnimeListDataGrid;
