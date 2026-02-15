import { LocalizationProvider } from '@mui/x-date-pickers';
import { AdapterDateFns } from '@mui/x-date-pickers/AdapterDateFns';
import { RouterProvider } from 'react-router';

import AppUpdateSnackbar from '@/components/AppUpdateSnackbar';
import usePlaybackObserverEvents from '@/hooks/detection/usePlaybackObserverEvents';
import useAppUpdater from '@/hooks/useAppUpdater';
import AnimeInformations from './components/AnimeInformations';
import { router } from './routes';
import './styles/fonts.css';
import './styles/global.css';

function App() {
  useAppUpdater();
  usePlaybackObserverEvents();

  return (
    <LocalizationProvider dateAdapter={AdapterDateFns}>
      <RouterProvider router={router} />
      <AnimeInformations />
      <AppUpdateSnackbar />
    </LocalizationProvider>
  );
}

export default App;
