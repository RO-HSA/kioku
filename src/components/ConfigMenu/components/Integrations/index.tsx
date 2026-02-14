import { useConfigMenuStore } from '@/stores/config/configMenu';
import MenuItem from '../MenuItem';
import Section from '../Section';
import MyanimelistForm from './MyanimelistForm';

const tabs = ['MyAnimeList'];

const Integrations = () => {
  const selectedTab = useConfigMenuStore((state) => state.selectedTab);

  return (
    <MenuItem tabs={tabs}>
      {selectedTab === 0 && (
        <Section title="Account">
          <MyanimelistForm />
        </Section>
      )}
    </MenuItem>
  );
};

export default Integrations;
