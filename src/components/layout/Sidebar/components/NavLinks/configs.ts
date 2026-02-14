import { Film, List } from 'lucide-react';

import { INavLink } from '@/types/Navigation';

export const navLinks: INavLink[] = [
  {
    label: 'Now Playing',
    icon: Film,
    link: '/now-playing'
  },
  {
    label: 'Anime List',
    icon: List,
    link: '/'
  }
];
