import { type FC } from 'react';

import NavButton from '@/components/NavButton';
import { useSidebarStore } from '@/stores/sidebar/sidebar';
import { useSidebarNavigationStore } from '@/stores/sidebar/sidebarNavigation';
import { navLinks } from './configs';

interface NavLinksProps {
  onNavigate?: () => void;
}

const NavLinks: FC<NavLinksProps> = ({ onNavigate }) => {
  const isSidebarOpen = useSidebarStore((state) => state.isOpen);
  const navigationStep = useSidebarNavigationStore(
    (state) => state.navigationStep
  );
  const setNavigationStep = useSidebarNavigationStore(
    (state) => state.setNavigationStep
  );

  return (
    <div className="flex w-full flex-col gap-1">
      {navLinks.map(({ step, icon, label }) => (
        <NavButton
          key={step}
          isSidebarOpen={isSidebarOpen}
          isActive={step === navigationStep}
          Icon={icon}
          label={label}
          onClick={() => {
            setNavigationStep(step);
            onNavigate?.();
          }}
        />
      ))}
    </div>
  );
};

export default NavLinks;
