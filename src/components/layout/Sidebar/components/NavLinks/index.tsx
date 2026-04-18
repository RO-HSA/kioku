import { Divider } from '@mui/material';
import { Fragment, useMemo, type FC } from 'react';
import { useLocation } from 'react-router';

import NavButton from '@/components/NavButton';
import { PathName } from '@/routes';
import { usePlayerDetectionStore } from '@/stores/detection/playerDetection';
import { useProviderStore } from '@/stores/providers/provider';
import { useSidebarStore } from '@/stores/sidebar/sidebar';
import { INavLink } from '@/types/Navigation';
import { Film, List, Search } from 'lucide-react';

interface NavLinksProps {
  onNavigate?: () => void;
}

const NavLinks: FC<NavLinksProps> = ({ onNavigate }) => {
  const location = useLocation();

  const isSidebarOpen = useSidebarStore((state) => state.isOpen);
  const activeEpisode = usePlayerDetectionStore((state) => state.activeEpisode);

  const selectedListType = useProviderStore((state) => state.selectedListType);

  const navLinks: INavLink[] = useMemo(
    () => [
      {
        label: 'Now Playing',
        icon: Film,
        link: PathName.NOW_PLAYING
      },
      {
        label: `${selectedListType === 'anime' ? 'Anime' : 'Manga'} List`,
        icon: List,
        link: PathName.ROOT
      },
      {
        label: 'Search',
        icon: Search,
        link: PathName.SEARCH
      }
    ],
    [selectedListType]
  );

  return (
    <div className="flex w-full flex-col gap-1">
      {navLinks.map(({ icon, label, link }) => (
        <Fragment key={label}>
          <NavButton
            isSidebarOpen={isSidebarOpen}
            Icon={icon}
            label={label}
            link={link}
            isActive={location.pathname === link}
            isDisabled={label === 'Now Playing' && !activeEpisode}
            onClick={() => {
              onNavigate?.();
            }}
          />
          {link === '/now-playing' && <Divider />}
        </Fragment>
      ))}
    </div>
  );
};

export default NavLinks;
