import PageLayout from '@/layouts/PageLayout';
import AnimeListDataGrid from './components/AnimeListDataGrid';
import { useMyAnimeListStore } from './stores/config/providers/myanimelist';
import './styles/global.css';

function App() {
  const animeListData = useMyAnimeListStore((state) => state.animeListData);

  return (
    <PageLayout>
      <AnimeListDataGrid listData={animeListData} />
    </PageLayout>
  );
}

export default App;
