import { Divider } from '@mui/material';
import { type FC } from 'react';
import { useLocation } from 'react-router';

import NavButton from '@/components/NavButton';
import { usePlayerDetectionStore } from '@/stores/playerDetection';
import { useSidebarStore } from '@/stores/sidebar/sidebar';
import { navLinks } from './configs';

interface NavLinksProps {
  onNavigate?: () => void;
}

const NavLinks: FC<NavLinksProps> = ({ onNavigate }) => {
  const location = useLocation();

  const isSidebarOpen = useSidebarStore((state) => state.isOpen);
  const activeEpisode = usePlayerDetectionStore((state) => state.activeEpisode);

  return (
    <div className="flex w-full flex-col gap-1">
      {navLinks.map(({ icon, label, link }) => (
        <>
          <NavButton
            key={label}
            isSidebarOpen={isSidebarOpen}
            Icon={icon}
            label={label}
            link={link}
            isActive={location.pathname === link}
            isDisabled={label === 'Now Playing' && !!activeEpisode}
            onClick={() => {
              onNavigate?.();
            }}
          />
          {link === '/now-playing' && <Divider />}
        </>
      ))}
    </div>
  );
};

export default NavLinks;
