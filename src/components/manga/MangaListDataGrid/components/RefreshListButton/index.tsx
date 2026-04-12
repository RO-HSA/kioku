import { IconButton, Tooltip } from '@mui/material';
import { LoaderCircle, RefreshCw } from 'lucide-react';
import { useCallback, useState } from 'react';

import { AniListService } from '@/services/backend/AniList';
import { MyAnimeListService } from '@/services/backend/MyAnimeList';
import { useMangaListDataGridStore } from '@/stores/mangaListDataGrid';
import { useAniListStore } from '@/stores/providers/anilist';
import { useMyAnimeListStore } from '@/stores/providers/myanimelist';
import { useProviderStore } from '@/stores/providers/provider';
import { Provider } from '@/types/List';

const RefreshListButton = () => {
  const [isRefreshing, setIsRefreshing] = useState(false);

  const activeProvider = useProviderStore((state) => state.activeProvider);

  const setIsLoading = useMangaListDataGridStore((state) => state.setIsloading);

  const setMyAnimeListData = useMyAnimeListStore(
    (state) => state.setMangaListData
  );

  const setAniListData = useAniListStore((state) => state.setMangaListData);

  const handleRefresh = useCallback(async () => {
    setIsRefreshing(true);

    switch (activeProvider) {
      case Provider.MY_ANIME_LIST:
        try {
          setIsLoading(true);
          const result = await MyAnimeListService.synchronizeList('manga');
          setMyAnimeListData(result);
        } finally {
          setIsRefreshing(false);
          setIsLoading(false);
        }
        break;
      case Provider.ANILIST:
        try {
          const result = await AniListService.synchronizeList('manga');
          setAniListData(result);
        } finally {
          setIsRefreshing(false);
          setIsLoading(false);
        }
        break;
      default:
        setIsRefreshing(false);
        setIsLoading(false);
    }
  }, [activeProvider, setMyAnimeListData, setAniListData, setIsRefreshing]);

  return (
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
    </>
  );
};

export default RefreshListButton;
