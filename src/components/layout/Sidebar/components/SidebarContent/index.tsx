import { Box, Divider } from '@mui/material';
import { Bolt, PanelLeftClose, PanelRightClose } from 'lucide-react';

import appIcon from '@/assets/app-icon.png';
import ModeToggle from '@/components/ModeToggle';
import NavButton from '@/components/NavButton';
import Button from '@/components/ui/Button';
import useWindowSize from '@/hooks/useWindowSize';
import { cn } from '@/lib/utils';
import { useConfigMenuStore } from '@/stores/config/configMenu';
import { useSidebarStore } from '@/stores/sidebar/sidebar';
import NavLinks from '../NavLinks';
import ProfileMenu from '../ProfileMenu';

const SidebarContent = () => {
  const { isMobile } = useWindowSize();

  const isOpen = useSidebarStore((state) => state.isOpen);
  const toggle = useSidebarStore((state) => state.toggle);

  const isConfigMenuOpen = useConfigMenuStore((state) => state.isOpen);
  const openConfigMenu = useConfigMenuStore((state) => state.openConfigMenu);

  return (
    <div className="flex h-full flex-col bg-background-paper text-text-primary w-full">
      <header
        className={cn(
          'flex h-16 items-center w-full gap-3 justify-between px-3 py-2 transition-all duration-300 ease-in-out',
          !isOpen && 'gap-0'
        )}>
        <div
          className={cn(
            'flex items-center gap-2 overflow-hidden select-none opacity-100 w-full transition-[width, opacity, gap] duration-300 ease-in-out',
            !isOpen && 'w-0 opacity-0 gap-0'
          )}>
          <img className="rounded-xl size-8" src={appIcon} alt="App Icon" />
          <span className="text-sm font-semibold">Kioku</span>
        </div>

        <Button
          variant="ghost"
          className="max-w-6"
          size="small"
          onClick={toggle}>
          {isOpen ? (
            <PanelLeftClose className="size-5" />
          ) : (
            <PanelRightClose className="size-5" />
          )}
        </Button>
      </header>

      <Divider />

      <nav className="flex flex-1 flex-col justify-between overflow-y-auto px-2 py-4">
        <div className="flex w-full flex-col gap-1">
          <NavLinks onNavigate={isMobile ? toggle : undefined} />
          <NavButton
            Icon={Bolt}
            label="Settings"
            isSidebarOpen={isOpen}
            isActive={isConfigMenuOpen}
            isDisabled={false}
            onClick={openConfigMenu}
          />
        </div>

        <Box className="w-full overflow-hidden">
          <ModeToggle />

          <Box className="border-t border-divider pt-4 mt-2">
            <ProfileMenu />
          </Box>
        </Box>
      </nav>
    </div>
  );
};

export default SidebarContent;
