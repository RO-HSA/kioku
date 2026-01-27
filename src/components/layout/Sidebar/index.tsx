import { Drawer, useMediaQuery } from '@mui/material';
import { useEffect } from 'react';

import { cn } from '@/lib/utils';
import { useSidebarStore } from '@/stores/sidebar';
import Content from './components/Content';

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

  if (isDesktop) {
    return (
      <aside
        className={cn(
          'sticky left-0 top-0 z-20 h-screen shrink-0 pr-0.5 transition-[width] duration-300 ease-in-out',
          isOpen ? 'w-60' : 'w-14'
        )}>
        <Content />
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
          className: 'bg-background-white text-text-primary w-80'
        }
      }}>
      <Content />
    </Drawer>
  );
};

export default Sidebar;
