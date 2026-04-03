import { createBrowserRouter } from 'react-router';
import NowPlaying from './components/anime/NowPlaying';
import ListDataGrid from './layouts/ListDataGrid';
import PageLayout from './layouts/PageLayout';

export const router = createBrowserRouter([
  {
    path: '/',
    element: <PageLayout />,
    children: [
      {
        path: '/',
        element: <ListDataGrid />
      },
      {
        path: '/now-playing',
        element: <NowPlaying />
      }
    ]
  }
]);
