import { IconButton, Menu, Tooltip } from '@mui/material';
import { Search } from 'lucide-react';
import { type MouseEvent, useCallback, useState } from 'react';
import { useLocation } from 'react-router';

import SearchInput from '@/components/ui/SearchInput';
import { PathName } from '@/routes';
import { useAnimeListDataGridStore } from '@/stores/animeListDataGrid';
import useAnimeRemoteSearch from '../../hooks/useAnimeRemoteSearch';

const SearchButton = () => {
  const [anchorEl, setAnchorEl] = useState<null | HTMLElement>(null);

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
  const searchAnime = useAnimeRemoteSearch();

  const open = Boolean(anchorEl);

  const handleClick = (event: MouseEvent<HTMLElement>) => {
    setAnchorEl(event.currentTarget);
  };

  const handleClose = () => {
    setAnchorEl(null);
  };

  const handleEnterKey = useCallback(
    async (searchValue: string) => {
      await searchAnime(searchValue, { navigateToSearch: !isSearchPage });
    },
    [isSearchPage, searchAnime]
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
