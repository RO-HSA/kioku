import { useAnimeListDataGridStore } from '@/stores/animeListDataGrid';
import { IconButton, Menu, Tooltip } from '@mui/material';
import { Search } from 'lucide-react';
import { type MouseEvent, useCallback, useState } from 'react';
import { useLocation, useNavigate } from 'react-router';

import SearchInput from '@/components/ui/SearchInput';
import { PathName } from '@/routes';
import { MyAnimeListService } from '@/services/backend/MyAnimeList';
import { useMyAnimeListStore } from '@/stores/providers/myanimelist';
import { useProviderStore } from '@/stores/providers/provider';
import { Provider } from '@/types/List';

const SearchButton = () => {
  const [anchorEl, setAnchorEl] = useState<null | HTMLElement>(null);

  const navigate = useNavigate();
  const location = useLocation();

  const isSearchPage = location.pathname === PathName.SEARCH;

  const localSearchValue = useAnimeListDataGridStore(
    (state) => state.localSearchValue
  );
  const remoteSearchValue = useAnimeListDataGridStore(
    (state) => state.remoteSearchValue
  );
  const setLocalSearchValue = useAnimeListDataGridStore(
    (state) => state.setLocalSearchValue
  );
  const setRemoteSearchValue = useAnimeListDataGridStore(
    (state) => state.setRemoteSearchValue
  );

  const activeProvider = useProviderStore((state) => state.activeProvider);

  const setAnimeSearchResults = useMyAnimeListStore(
    (state) => state.setAnimeSearchResults
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
      if (!isSearchPage) {
        setRemoteSearchValue(searchValue);
        navigate('/search');
      }

      switch (activeProvider) {
        case Provider.MY_ANIME_LIST:
          const results = await MyAnimeListService.searchMedia(
            searchValue,
            'anime'
          );
          setAnimeSearchResults(results);
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
      setAnimeSearchResults
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
