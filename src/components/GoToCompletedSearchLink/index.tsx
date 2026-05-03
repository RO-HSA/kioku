import { Link } from '@mui/material';
import { FC } from 'react';

import useAnimeRemoteSearch from '@/components/anime/AnimeListDataGrid/hooks/useAnimeRemoteSearch';
import { useProviderStore } from '@/stores/providers/provider';

interface GoToCompletedSearchLinkProps {
  searchTerm: string;
}

const GoToCompletedSearchLink: FC<GoToCompletedSearchLinkProps> = ({
  searchTerm
}) => {
  const setSelectedListType = useProviderStore(
    (state) => state.setSelectedListType
  );
  const searchAnime = useAnimeRemoteSearch();

  const handleClick = async () => {
    setSelectedListType('anime');
    await searchAnime(searchTerm, { navigateToSearch: true });
  };

  return (
    <Link sx={{ cursor: 'pointer' }} onClick={handleClick}>
      Search
    </Link>
  );
};

export default GoToCompletedSearchLink;
