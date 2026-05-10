import { IconButton, Tooltip } from '@mui/material';
import { LoaderCircle, RefreshCw } from 'lucide-react';
import { useCallback, useState } from 'react';

import { refreshActivePlaybackMatch } from '@/hooks/detection/refreshActivePlaybackMatch';
import { AniListService } from '@/services/backend/AniList';
import { MyAnimeListService } from '@/services/backend/MyAnimeList';
import { useAnimeListDataGridStore } from '@/stores/animeListDataGrid';
import { useAniListStore } from '@/stores/providers/anilist';
import { useMyAnimeListStore } from '@/stores/providers/myanimelist';
import { useProviderStore } from '@/stores/providers/provider';
import { Provider } from '@/types/List';

const RefreshListButton = () => {
  const [isRefreshing, setIsRefreshing] = useState(false);

  const activeProvider = useProviderStore((state) => state.activeProvider);

  const setIsLoading = useAnimeListDataGridStore((state) => state.setIsLoading);

  const setMyAnimeListData = useMyAnimeListStore(
    (state) => state.setAnimeListData
  );

  const setAniListData = useAniListStore((state) => state.setAnimeListData);

  const handleRefresh = useCallback(async () => {
    setIsRefreshing(true);

    switch (activeProvider) {
      case Provider.MY_ANIME_LIST:
        try {
          setIsLoading(true);
          const result = await MyAnimeListService.synchronizeList();
          setMyAnimeListData(result);
          refreshActivePlaybackMatch(Provider.MY_ANIME_LIST);
        } finally {
          setIsRefreshing(false);
          setIsLoading(false);
        }
        break;
      case Provider.ANILIST:
        try {
          setIsLoading(true);
          const result = await AniListService.synchronizeList();
          setAniListData(result);
          refreshActivePlaybackMatch(Provider.ANILIST);
        } finally {
          setIsRefreshing(false);
          setIsLoading(false);
        }
        break;
      default:
        setIsRefreshing(false);
        setIsLoading(false);
        break;
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
