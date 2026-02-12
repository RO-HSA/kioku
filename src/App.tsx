import { LocalizationProvider } from '@mui/x-date-pickers';
import { AdapterDateFns } from '@mui/x-date-pickers/AdapterDateFns';

import PageLayout from '@/layouts/PageLayout';
import AnimeInformations from './components/AnimeInformations';
import AnimeListDataGrid from './components/AnimeListDataGrid';
import { useMyAnimeListStore } from './stores/providers/myanimelist';
import './styles/global.css';

function App() {
  const animeListData = useMyAnimeListStore((state) => state.animeListData);

  return (
    <LocalizationProvider dateAdapter={AdapterDateFns}>
      <PageLayout>
        <AnimeListDataGrid listData={animeListData} />
        <AnimeInformations />
      </PageLayout>
    </LocalizationProvider>
  );
}

export default App;
