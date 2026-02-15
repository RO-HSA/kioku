import { useConfigMenuStore } from '@/stores/config/configMenu';
import MenuItem from '../MenuItem';
import AppUpdateForm from './AppUpdateForm';

const tabs = ['Application'];

const Updates = () => {
  const selectedTab = useConfigMenuStore((state) => state.selectedTab);

  return (
    <MenuItem tabs={tabs}>{selectedTab === 0 && <AppUpdateForm />}</MenuItem>
  );
};

export default Updates;
