import { useConfigMenuStore } from '@/stores/config/configMenu';
import MenuItem from '../MenuItem';
import DiscordSharingForm from './DiscordSharingForm';

export enum SharingTab {
  DISCORD = 'DISCORD'
}

export const sharingTabs = [SharingTab.DISCORD];

const Sharing = () => {
  const selectedTab = useConfigMenuStore((state) => state.selectedTab);

  return (
    <MenuItem tabs={sharingTabs}>
      {selectedTab === sharingTabs.indexOf(SharingTab.DISCORD) && (
        <DiscordSharingForm />
      )}
    </MenuItem>
  );
};

export default Sharing;
