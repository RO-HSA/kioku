import {
  Dialog,
  DialogContent,
  DialogTitle,
  Drawer,
  IconButton,
  Typography
} from '@mui/material';
import { Menu, X } from 'lucide-react';
import { useEffect, useState } from 'react';

import useWindowSize from '@/hooks/useWindowSize';
import { useConfigMenuStore } from '@/stores/config/configMenu';
import MenuContent from './components/MenuContent';
import NavContent from './components/NavContent';

const ConfigMenu = () => {
  const [drawerOpen, setDrawerOpen] = useState(false);
  const isOpen = useConfigMenuStore((state) => state.isOpen);
  const closeMenu = useConfigMenuStore((state) => state.closeConfigMenu);
  const { isMobile } = useWindowSize();

  useEffect(() => {
    const handleResize = () => {
      setDrawerOpen(false);
    };

    window.addEventListener('resize', handleResize);

    return () => {
      window.removeEventListener('resize', handleResize);
    };
  }, [setDrawerOpen]);

  return (
    <Dialog
      open={isOpen}
      fullScreen={isMobile}
      fullWidth
      maxWidth="md"
      className="relative"
      onClose={closeMenu}>
      <DialogTitle className="flex justify-between items-center">
        <div className="flex gap-1.5 items-center">
          <Menu
            className="sm:hidden text-primary"
            cursor="pointer"
            onClick={() => setDrawerOpen(true)}
          />
          <Typography variant="h6">Settings</Typography>
        </div>
        <IconButton onClick={closeMenu}>
          <X />
        </IconButton>
      </DialogTitle>

      <DialogContent className="h-screen relative px-0!">
        <div className="flex w-full h-full overflow-hidden text-text-primary sm:pt-2 border-t border-primary/25">
          <div className="hidden sm:block h-full w-full pl-1 max-w-44 sm:border-r sm:border-primary/25 pr-2">
            <NavContent />
          </div>

          <MenuContent />
        </div>
      </DialogContent>

      <Drawer
        open={drawerOpen}
        variant="persistent"
        elevation={24}
        onClose={() => setDrawerOpen(false)}
        slotProps={{
          paper: {
            sx: {
              position: 'absolute',
              top: 0,
              left: 0,
              height: '100%',
              maxHeight: '100%',
              maxWidth: '250px',
              width: '100%'
            }
          }
        }}>
        <div>
          <NavContent onClickCloseButton={() => setDrawerOpen(false)} />
        </div>
      </Drawer>
    </Dialog>
  );
};

export default ConfigMenu;
