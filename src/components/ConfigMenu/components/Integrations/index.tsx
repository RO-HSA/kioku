import { useState } from 'react';
import Section from '../Section';
import Tabs from '../Tabs';
import MyanimelistForm from './MyanimelistForm';

const tabs = ['MyAnimeList'];

const Integrations = () => {
  const [selectedTab, setSelectedTab] = useState(0);

  return (
    <div className="flex flex-col gap-2 w-full">
      <Tabs
        tabs={tabs}
        selectedTab={selectedTab}
        setSelectedTab={setSelectedTab}
      />

      <div className="flex flex-col gap-3 w-full">
        <Section title="Account">
          {selectedTab === 0 && <MyanimelistForm />}
        </Section>
      </div>
    </div>
  );
};

export default Integrations;
