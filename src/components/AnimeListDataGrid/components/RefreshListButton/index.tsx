import { IconButton, Tooltip } from '@mui/material';
import { useCallback, useState } from 'react';

import { MyAnimeListService } from '@/services/backend/MyAnimeList';
import { useMyAnimeListStore } from '@/stores/providers/myanimelist';
import { LoaderCircle, RefreshCw } from 'lucide-react';

const RefreshListButton = () => {
  const [isRefreshing, setIsRefreshing] = useState(false);
  const setAnimeListData = useMyAnimeListStore(
    (state) => state.setAnimeListData
  );

  const handleRefresh = useCallback(async () => {
    setIsRefreshing(true);

    try {
      const result = await MyAnimeListService.synchronizeList();
      setAnimeListData(result);
    } finally {
      setIsRefreshing(false);
    }
  }, [setAnimeListData, setIsRefreshing]);

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
