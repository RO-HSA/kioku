import SearchInput from '@/components/ui/SearchInput';
import { useAnimeListDataGridStore } from '@/stores/animeListDataGrid';
import { IconButton, Menu, Tooltip } from '@mui/material';
import { Search } from 'lucide-react';
import { type MouseEvent, useState } from 'react';

const SearchButton = () => {
  const [anchorEl, setAnchorEl] = useState<null | HTMLElement>(null);
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
          <SearchInput onDebounceEnd={setSearchValue} />
        </div>
      </Menu>
    </>
  );
};

export default SearchButton;
