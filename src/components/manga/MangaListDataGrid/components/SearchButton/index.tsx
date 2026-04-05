import { IconButton, Menu, Tooltip } from '@mui/material';
import { Search } from 'lucide-react';
import { type MouseEvent, useCallback, useState } from 'react';
import { useLocation, useNavigate } from 'react-router';

import SearchInput from '@/components/ui/SearchInput';
import { PathName } from '@/routes';
import { MyAnimeListService } from '@/services/backend/MyAnimeList';
import { useMangaListDataGridStore } from '@/stores/mangaListDataGrid';
import { useMyAnimeListStore } from '@/stores/providers/myanimelist';
import { useProviderStore } from '@/stores/providers/provider';
import { Provider } from '@/types/List';

const SearchButton = () => {
  const [anchorEl, setAnchorEl] = useState<null | HTMLElement>(null);

  const navigate = useNavigate();
  const location = useLocation();

  const isSearchPage = location.pathname === PathName.SEARCH;

  const localSearchValue = useMangaListDataGridStore(
    (state) => state.localSearchValue
  );
  const remoteSearchValue = useMangaListDataGridStore(
    (state) => state.remoteSearchValue
  );
  const setLocalSearchValue = useMangaListDataGridStore(
    (state) => state.setLocalSearchValue
  );
  const setRemoteSearchValue = useMangaListDataGridStore(
    (state) => state.setRemoteSearchValue
  );

  const activeProvider = useProviderStore((state) => state.activeProvider);

  const setMangaSearchResults = useMyAnimeListStore(
    (state) => state.setMangaSearchResults
  );

  const open = Boolean(anchorEl);

  const handleClick = (event: MouseEvent<HTMLElement>) => {
    setAnchorEl(event.currentTarget);
  };

  const handleClose = () => {
    setAnchorEl(null);
  };

  const handleEnterKey = useCallback(
    async (searchValue: string) => {
      if (searchValue.trim().length === 0) return;

      if (!isSearchPage) {
        setRemoteSearchValue(searchValue);
        navigate('/search');
      }

      switch (activeProvider) {
        case Provider.MY_ANIME_LIST:
          const results = await MyAnimeListService.searchMedia(
            searchValue,
            'manga'
          );
          setMangaSearchResults(results);
          break;
        case Provider.ANILIST:
          break;
        default:
          break;
      }
    },
    [
      isSearchPage,
      activeProvider,
      navigate,
      setRemoteSearchValue,
      setMangaSearchResults
    ]
  );

  return (
    <>
      <Tooltip title="Search">
        <IconButton onClick={handleClick}>
          <Search />
        </IconButton>
      </Tooltip>

      <Menu anchorEl={anchorEl} open={open} onClose={handleClose}>
        <div className="px-2">
          <SearchInput
            value={isSearchPage ? remoteSearchValue : localSearchValue}
            onChange={isSearchPage ? setRemoteSearchValue : setLocalSearchValue}
            onEnterKey={handleEnterKey}
          />
        </div>
      </Menu>
    </>
  );
};

export default SearchButton;
