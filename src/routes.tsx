import { createBrowserRouter } from 'react-router';

import NowPlaying from './components/anime/NowPlaying';
import ListDataGrid from './layouts/ListDataGrid';
import PageLayout from './layouts/PageLayout';

export enum PathName {
  ROOT = '/',
  NOW_PLAYING = '/now-playing',
  SEARCH = '/search'
}

export const router = createBrowserRouter([
  {
    path: PathName.ROOT,
    element: <PageLayout />,
    children: [
      {
        path: PathName.ROOT,
        element: <ListDataGrid />
      },
      {
        path: PathName.NOW_PLAYING,
        element: <NowPlaying />
      },
      {
        path: PathName.SEARCH,
        element: <ListDataGrid />
      }
    ]
  }
]);
