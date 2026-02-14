import { useConfigMenuStore } from '@/stores/config/configMenu';
import { FC, ReactNode } from 'react';
import Tabs from '../Tabs';

interface MenuItemProps {
  tabs: string[];
  children: ReactNode;
}

const MenuItem: FC<MenuItemProps> = ({ tabs, children }) => {
  const selectedTab = useConfigMenuStore((state) => state.selectedTab);
  const setSelectedTab = useConfigMenuStore((state) => state.setSelectedTab);

  return (
    <div className="flex flex-col gap-2 w-full h-full">
      <Tabs
        tabs={tabs}
        selectedTab={selectedTab}
        setSelectedTab={setSelectedTab}
      />

      <div className="flex flex-col gap-3 w-full h-full">{children}</div>
    </div>
  );
};

export default MenuItem;
