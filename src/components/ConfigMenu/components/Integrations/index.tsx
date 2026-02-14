import { useState } from 'react';
import Section from '../Section';
import Tabs from '../Tabs';
import MyanimelistForm from './MyanimelistForm';
import PlayerDetectionForm from './PlayerDetectionForm';

const tabs = ['MyAnimeList', 'Players'];

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
        {selectedTab === 0 && (
          <Section title="Account">
            <MyanimelistForm />
          </Section>
        )}

        {selectedTab === 1 && (
          <Section title="Player Detection">
            <PlayerDetectionForm />
          </Section>
        )}
      </div>
    </div>
  );
};

export default Integrations;
