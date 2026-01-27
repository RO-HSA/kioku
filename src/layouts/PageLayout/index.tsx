import { type FC, ReactNode } from 'react';
import { Menu } from 'lucide-react';

import Sidebar from '@/components/layout/Sidebar';
import Button from '@/components/ui/Button';
import { useSidebarStore } from '@/stores/sidebar';
import { Divider } from '@mui/material';

interface PageLayoutProps {
  children: ReactNode;
}

const PageLayout: FC<PageLayoutProps> = ({ children }) => {
  const toggle = useSidebarStore((state) => state.toggle);

  return (
    <div className="flex h-screen w-full overflow-hidden bg-background-default text-text-primary">
      <Sidebar />

      <Divider orientation="vertical" flexItem />

      <div className="flex min-w-0 flex-1 flex-col">
        <header className="flex items-center gap-3 bg-background-paper px-4 py-3 min-[900px]:hidden">
          <Button
            variant="ghost"
            size="small"
            onClick={toggle}
            className="min-w-0 px-2">
            <Menu className="size-5" />
          </Button>
          <div className="flex flex-col">
            <span className="text-sm font-semibold">Dashboard</span>
            <span className="text-xs text-text-secondary">Overview</span>
          </div>
        </header>

        <Divider className="min-[900px]:hidden" />

        <main className="flex-1 overflow-y-auto">
          <div className="mx-auto w-full max-w-6xl p-6">{children}</div>
        </main>
      </div>
    </div>
  );
};

export default PageLayout;
