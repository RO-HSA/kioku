import { IconButton, Tooltip } from '@mui/material';
import { LoaderCircle, RefreshCw } from 'lucide-react';
import { useCallback, useState } from 'react';

import { AniListService } from '@/services/backend/AniList';
import { MyAnimeListService } from '@/services/backend/MyAnimeList';
import { useAniListStore } from '@/stores/providers/anilist';
import { useMyAnimeListStore } from '@/stores/providers/myanimelist';
import { useProviderStore } from '@/stores/providers/provider';
import { Provider } from '@/types/List';

const RefreshListButton = () => {
  const [isRefreshing, setIsRefreshing] = useState(false);

  const activeProvider = useProviderStore((state) => state.activeProvider);
  const setMyAnimeListData = useMyAnimeListStore(
    (state) => state.setAnimeListData
  );
  const setAniListData = useAniListStore((state) => state.setAnimeListData);

  const handleRefresh = useCallback(async () => {
    setIsRefreshing(true);

    switch (activeProvider) {
      case Provider.MY_ANIME_LIST:
        try {
          const result = await MyAnimeListService.synchronizeList();
          setMyAnimeListData(result);
        } finally {
          setIsRefreshing(false);
        }
        break;
      case Provider.ANILIST:
        try {
          const result = await AniListService.synchronizeList();
          setAniListData(result);
        } finally {
          setIsRefreshing(false);
        }
        break;
      default:
        setIsRefreshing(false);
    }
  }, [setMyAnimeListData, setAniListData, setIsRefreshing, activeProvider]);

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
