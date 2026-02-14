import { Divider } from '@mui/material';
import { Menu } from 'lucide-react';
import { Outlet } from 'react-router';

import ConfigMenu from '@/components/ConfigMenu';
import Sidebar from '@/components/layout/Sidebar';
import Button from '@/components/ui/Button';
import { useSidebarStore } from '@/stores/sidebar/sidebar';

const PageLayout = () => {
  const toggle = useSidebarStore((state) => state.toggle);

  return (
    <div className="flex h-screen w-full overflow-hidden bg-background-default text-text-primary">
      <Sidebar />

      <Divider orientation="vertical" flexItem />

      <div className="flex min-w-0 flex-1 flex-col min-h-0">
        <header className="flex items-center gap-3 bg-background-paper px-4 py-3 sm:hidden">
          <Button
            variant="ghost"
            size="small"
            onClick={toggle}
            className="min-w-0 px-2">
            <Menu className="size-5" />
          </Button>

          <div className="flex flex-col">
            <span className="text-sm font-semibold">kioku</span>
            <span className="text-xs text-text-secondary">Anime list</span>
          </div>
        </header>

        <Divider className="sm:hidden" />

        <main className="flex-1 min-h-0 min-w-0 overflow-hidden">
          <div className="w-full h-full min-h-0">
            <Outlet />
          </div>
        </main>
      </div>

      <ConfigMenu />
    </div>
  );
};

export default PageLayout;
