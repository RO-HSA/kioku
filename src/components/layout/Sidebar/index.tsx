import { Drawer } from '@mui/material';
import { useEffect } from 'react';

import useWindowSize from '@/hooks/useWindowSize';
import { cn } from '@/lib/utils';
import { useSidebarStore } from '@/stores/sidebar/sidebar';
import SidebarContent from './components/SidebarContent';

const Sidebar = () => {
  const isOpen = useSidebarStore((state) => state.isOpen);
  const toggle = useSidebarStore((state) => state.toggle);
  const close = useSidebarStore((state) => state.close);
  const { isMobile } = useWindowSize();

  useEffect(() => {
    const handleResize = () => {
      if (isMobile) {
        close();
      }
    };

    window.addEventListener('resize', handleResize);

    return () => {
      window.removeEventListener('resize', handleResize);
    };
  }, [close, isMobile]);

  if (!isMobile) {
    return (
      <aside
        className={cn(
          'sticky left-0 top-0 z-20 h-full shrink-0 pr-0.5 transition-[width] duration-300 ease-in-out',
          isOpen ? 'w-60' : 'w-14'
        )}>
        {<SidebarContent />}
      </aside>
    );
  }

  return (
    <Drawer
      variant="temporary"
      open={isOpen}
      onClose={toggle}
      hideBackdrop
      sx={{
        flexShrink: 0,
        [`& .MuiDrawer-paper`]: { boxSizing: 'border-box' }
      }}
      slotProps={{
        paper: {
          className: 'bg-background-white text-text-primary w-80'
        }
      }}>
      <SidebarContent />
    </Drawer>
  );
};

export default Sidebar;
