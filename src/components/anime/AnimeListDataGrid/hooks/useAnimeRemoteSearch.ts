import { useCallback } from 'react';
import { useNavigate } from 'react-router';

import { PathName } from '@/routes';
import { AniListService } from '@/services/backend/AniList';
import { MyAnimeListService } from '@/services/backend/MyAnimeList';
import { useAnimeListDataGridStore } from '@/stores/animeListDataGrid';
import { useAniListStore } from '@/stores/providers/anilist';
import { useMyAnimeListStore } from '@/stores/providers/myanimelist';
import { useProviderStore } from '@/stores/providers/provider';
import { Provider } from '@/types/List';

interface SearchAnimeOptions {
  navigateToSearch?: boolean;
}

const useAnimeRemoteSearch = () => {
  const navigate = useNavigate();

  const setIsLoadingGrid = useAnimeListDataGridStore(
    (state) => state.setIsLoading
  );
  const setRemoteSearchValue = useAnimeListDataGridStore(
    (state) => state.setRemoteSearchValue
  );

  const activeProvider = useProviderStore((state) => state.activeProvider);

  const setAnimeSearchResults = useMyAnimeListStore(
    (state) => state.setAnimeSearchResults
  );
  const setAniListAnimeSearchResults = useAniListStore(
    (state) => state.setAnimeSearchResults
  );

  return useCallback(
    async (
      searchValue: string,
      { navigateToSearch = false }: SearchAnimeOptions = {}
    ) => {
      const trimmedSearchValue = searchValue.trim();

      if (trimmedSearchValue.length === 0) return;

      setRemoteSearchValue(searchValue);

      if (!activeProvider) return;

      setIsLoadingGrid(true);

      if (navigateToSearch) {
        navigate(PathName.SEARCH);
      }

      try {
        switch (activeProvider) {
          case Provider.MY_ANIME_LIST: {
            const results = await MyAnimeListService.searchMedia(
              trimmedSearchValue,
              'anime'
            );
            setAnimeSearchResults(results);
            break;
          }
          case Provider.ANILIST: {
            const results = await AniListService.searchMedia(
              trimmedSearchValue,
              'anime'
            );
            setAniListAnimeSearchResults(results);
            break;
          }
          default:
            break;
        }
      } finally {
        setIsLoadingGrid(false);
      }
    },
    [
      activeProvider,
      navigate,
      setAniListAnimeSearchResults,
      setAnimeSearchResults,
      setIsLoadingGrid,
      setRemoteSearchValue
    ]
  );
};

export default useAnimeRemoteSearch;
