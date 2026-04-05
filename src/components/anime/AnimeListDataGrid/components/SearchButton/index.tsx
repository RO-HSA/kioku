import { useAnimeListDataGridStore } from '@/stores/animeListDataGrid';
import { IconButton, Menu, Tooltip } from '@mui/material';
import { Search } from 'lucide-react';
import { type MouseEvent, useState } from 'react';
import { useLocation, useNavigate } from 'react-router';

import SearchInput from '@/components/ui/SearchInput';
import { PathName } from '@/routes';

const SearchButton = () => {
  const [anchorEl, setAnchorEl] = useState<null | HTMLElement>(null);

  const navigate = useNavigate();
  const location = useLocation();

  const isSearchPage = location.pathname === PathName.SEARCH;

  const searchValue = useAnimeListDataGridStore((state) => state.searchValue);
  const setSearchValue = useAnimeListDataGridStore(
    (state) => state.setSearchValue
  );

  const open = Boolean(anchorEl);

  const handleClick = (event: MouseEvent<HTMLElement>) => {
    setAnchorEl(event.currentTarget);
  };

  const handleClose = () => {
    setAnchorEl(null);
  };

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
            value={searchValue}
            onChange={setSearchValue}
            onEnterKey={() => !isSearchPage && navigate('/search')}
          />
        </div>
      </Menu>
    </>
  );
};

export default SearchButton;
