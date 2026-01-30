import PageLayout from '@/layouts/PageLayout';
import { Button } from '@mui/material';
import { MyAnimeListService } from './services/backend/MyAnimeList';
import { useMyAnimeListStore } from './stores/config/providers/myanimelist';
import './styles/global.css';

function App() {
  const setListData = useMyAnimeListStore((state) => state.setListData);
  const listData = useMyAnimeListStore((state) => state.listData);
  console.log({ listData });

  const synchronize = async () => {
    const response = await MyAnimeListService.synchronizeList();
    setListData(response);
  };
  return (
    <PageLayout>
      Hello, World!
      <Button variant="contained" onClick={synchronize}>
        Click Me
      </Button>
    </PageLayout>
  );
}

export default App;
