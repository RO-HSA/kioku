import AnimeListDataGrid from '@/components/anime/AnimeListDataGrid';
import MangaListDataGrid from '@/components/manga/MangaListDataGrid';
import { useProviderStore } from '@/stores/providers/provider';

const ListDataGrid = () => {
  const selectedListType = useProviderStore((state) => state.selectedListType);

  return (
    <>
      {selectedListType === 'anime' ? (
        <AnimeListDataGrid />
      ) : (
        <MangaListDataGrid />
      )}
    </>
  );
};

export default ListDataGrid;
