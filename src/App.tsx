import { LocalizationProvider } from '@mui/x-date-pickers';
import { AdapterDateFns } from '@mui/x-date-pickers/AdapterDateFns';
import { RouterProvider } from 'react-router';

import usePlaybackObserverEvents from '@/hooks/usePlaybackObserverEvents';
import AnimeInformations from './components/AnimeInformations';
import { router } from './routes';
import './styles/global.css';

function App() {
  usePlaybackObserverEvents();

  return (
    <LocalizationProvider dateAdapter={AdapterDateFns}>
      <RouterProvider router={router} />
      <AnimeInformations />
    </LocalizationProvider>
  );
}

export default App;
