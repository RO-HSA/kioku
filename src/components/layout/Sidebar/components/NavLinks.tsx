import { type FC } from 'react';

import { navLinks } from '../configs';
import { cn } from '@/lib/utils';
import { useSidebarNavigationStore } from '@/stores/useSidebarNavigationStore';
import { useSidebarStore } from '@/stores/useSidebarStore';

interface NavLinksProps {
  onNavigate?: () => void;
}

const NavLinks: FC<NavLinksProps> = ({ onNavigate }) => {
  const navigationStep = useSidebarNavigationStore(
    (state) => state.navigationStep
  );
  const setNavigationStep = useSidebarNavigationStore(
    (state) => state.setNavigationStep
  );
  const isSidebarOpen = useSidebarStore((state) => state.isOpen);

  return (
    <div className="flex w-full flex-col gap-1">
      {navLinks.map(({ step, icon: Icon, label }) => {
        const isActive = navigationStep === step;
        return (
          <button
            key={step}
            type="button"
            onClick={() => {
              setNavigationStep(step);
              onNavigate?.();
            }}
            aria-current={isActive ? 'page' : undefined}
            className={cn(
              'flex w-full items-center justify-center gap-3 cursor-pointer rounded-xl px-3 py-2 text-left text-sm font-medium transition-[colors, gap] duration-200 ease-in-out',
              isActive
                ? 'bg-primary/10 text-primary'
                : 'text-text-secondary hover:bg-primary/5 hover:text-text-primary',
              !isSidebarOpen && 'justify-center gap-0'
            )}>
            <Icon className="size-5 shrink-0 justify-self-center" />
            <span
              className={cn(
                'truncate w-full opacity-100 transition-opacity duration-300 ease-in-out',
                !isSidebarOpen && 'opacity-0'
              )}>
              {label}
            </span>
          </button>
        );
      })}
    </div>
  );
};

export default NavLinks;
