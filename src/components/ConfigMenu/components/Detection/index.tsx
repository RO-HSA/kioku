import { useConfigMenuStore } from '@/stores/config/configMenu';
import MenuItem from '../MenuItem';
import PlayerDetectionForm from './PlayerDetectionForm';

const tabs = ['Players'];

const Detection = () => {
  const selectedTab = useConfigMenuStore((state) => state.selectedTab);

  return (
    <MenuItem tabs={tabs}>
      {selectedTab === 0 && <PlayerDetectionForm />}
    </MenuItem>
  );
};

export default Detection;
