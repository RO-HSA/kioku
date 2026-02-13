import { Tabs as MuiTabs, Tab } from '@mui/material';
import { FC } from 'react';

interface TabsProps {
  selectedTab: number;
  setSelectedTab: (index: number) => void;
  tabs: string[];
}

const Tabs: FC<TabsProps> = ({ selectedTab, setSelectedTab, tabs }) => {
  return (
    <MuiTabs value={selectedTab} onChange={(_, index) => setSelectedTab(index)}>
      {tabs.map((tab) => (
        <Tab key={tab} label={tab} />
      ))}
    </MuiTabs>
  );
};

export default Tabs;
