import { LocalizationProvider } from '@mui/x-date-pickers';
import { AdapterDateFns } from '@mui/x-date-pickers/AdapterDateFns';
import { RouterProvider } from 'react-router';

import AppUpdateSnackbar from '@/components/AppUpdateSnackbar';
import useDiscordRichPresence from '@/hooks/detection/useDiscordRichPresence';
import usePlaybackObserverEvents from '@/hooks/detection/usePlaybackObserverEvents';
import useProviderMigration from '@/hooks/migrations/useProviderMigration';
import useAppUpdater from '@/hooks/useAppUpdater';
import AnimeInformations from './components/anime/AnimeInformations';
import { router } from './routes';
import './styles/fonts.css';
import './styles/global.css';

function App() {
  useAppUpdater();
  usePlaybackObserverEvents();
  useDiscordRichPresence();
  useProviderMigration();

  return (
    <LocalizationProvider dateAdapter={AdapterDateFns}>
      <RouterProvider router={router} />
      <AnimeInformations />
      <AppUpdateSnackbar />
    </LocalizationProvider>
  );
}

export default App;
