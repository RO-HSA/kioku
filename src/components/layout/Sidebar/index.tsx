import { Drawer, useMediaQuery } from '@mui/material';
import { PanelLeftClose, PanelRightClose } from 'lucide-react';
import { useEffect } from 'react';

import Button from '@/components/ui/Button';
import { cn } from '@/lib/utils';
import { useSidebarStore } from '@/stores/useSidebarStore';
import NavLinks from './components/NavLinks';

const Sidebar = () => {
  const isOpen = useSidebarStore((state) => state.isOpen);
  const toggle = useSidebarStore((state) => state.toggle);
  const close = useSidebarStore((state) => state.close);
  const isDesktop = useMediaQuery('(min-width: 900px)');

  useEffect(() => {
    const handleResize = () => {
      close();
    };

    window.addEventListener('resize', handleResize);

    return () => {
      window.removeEventListener('resize', handleResize);
    };
  }, [close]);

  const content = (
    <div className="flex h-full flex-col bg-background-paper text-text-primary">
      <header
        className={cn(
          'flex h-16 items-center gap-3 border-b border-divider px-3',
          !isOpen ? 'justify-center' : 'justify-between'
        )}>
        <div
          className={cn(
            'flex items-center gap-2 overflow-hidden transition-all',
            !isOpen && 'w-0 opacity-0'
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

      <nav className="flex flex-1 flex-col gap-1 overflow-y-auto px-2 py-4">
        <NavLinks onNavigate={!isDesktop ? toggle : undefined} />
      </nav>
    </div>
  );

  if (isDesktop) {
    return (
      <aside
        className={cn(
          'sticky left-0 top-0 z-20 h-screen shrink-0 transition-[width] duration-300 ease-in-out',
          isOpen ? 'w-72' : 'w-16'
        )}>
        {content}
      </aside>
    );
  }

  return (
    <Drawer
      variant="temporary"
      open={isOpen}
      onClose={toggle}
      ModalProps={{ keepMounted: true }}
      slotProps={{
        paper: {
          className: cn('bg-background-white text-text-primary w-80')
        }
      }}>
      {content}
    </Drawer>
  );
};

export default Sidebar;
