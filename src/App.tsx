import PageLayout from '@/layouts/PageLayout';
import AnimeInformations from './components/AnimeInformations';
import AnimeListDataGrid from './components/AnimeListDataGrid';
import { useMyAnimeListStore } from './stores/providers/myanimelist';
import './styles/global.css';

function App() {
  const animeListData = useMyAnimeListStore((state) => state.animeListData);

  return (
    <PageLayout>
      <AnimeListDataGrid listData={animeListData} />
      <AnimeInformations />
    </PageLayout>
  );
}

export default App;
