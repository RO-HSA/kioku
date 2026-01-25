import { Box, Divider, Typography, useMediaQuery } from '@mui/material';
import { PanelLeftClose, PanelRightClose } from 'lucide-react';

import NavLinks from './NavLinks';
import { cn } from '@/lib/utils';
import { useSidebarStore } from '@/stores/useSidebarStore';
import Button from '@/components/ui/Button';
import ModeToggle from '@/components/ModeToggle';

const Content = () => {
  const isDesktop = useMediaQuery('(min-width: 900px)');
  const isOpen = useSidebarStore((state) => state.isOpen);
  const toggle = useSidebarStore((state) => state.toggle);

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
          <div className="flex size-9 items-center justify-center rounded-xl bg-primary/10 text-primary">
            <span className="text-xs font-semibold tracking-wide">K</span>
          </div>
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
        <NavLinks onNavigate={!isDesktop ? toggle : undefined} />

        <Box className="w-full overflow-hidden">
          <ModeToggle />
        </Box>
      </nav>
    </div>
  );
};

export default Content;
