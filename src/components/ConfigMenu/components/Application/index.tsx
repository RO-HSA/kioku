import { useConfigMenuStore } from '@/stores/config/configMenu';
import MenuItem from '../MenuItem';
import AppUpdateForm from './AppUpdateForm';
import GeneralConfigsForm from './GeneralConfigsForm';

export enum ApplicationTab {
  GENERAL = 'GENERAL',
  UPDATE = 'UPDATE'
}

export const applicationTabs = [ApplicationTab.GENERAL, ApplicationTab.UPDATE];

const Application = () => {
  const selectedTab = useConfigMenuStore((state) => state.selectedTab);

  return (
    <MenuItem tabs={applicationTabs}>
      {selectedTab === applicationTabs.indexOf(ApplicationTab.GENERAL) && (
        <GeneralConfigsForm />
      )}

      {selectedTab === applicationTabs.indexOf(ApplicationTab.UPDATE) && (
        <AppUpdateForm />
      )}
    </MenuItem>
  );
};

export default Application;
