import { createBrowserRouter } from 'react-router';
import AnimeListDataGrid from './components/AnimeListDataGrid';
import NowPlaying from './components/NowPlaying';
import PageLayout from './layouts/PageLayout';

export const router = createBrowserRouter([
  {
    path: '/',
    element: <PageLayout />,
    children: [
      {
        path: '/',
        element: <AnimeListDataGrid />
      },
      {
        path: '/now-playing',
        element: <NowPlaying />
      }
    ]
  }
]);
