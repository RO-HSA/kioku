import { openUrl } from '@tauri-apps/plugin-opener';
import { ArrowLeftRight, LogOut, User } from 'lucide-react';
import {
  ReactNode,
  useCallback,
  useMemo,
  useState,
  type MouseEvent
} from 'react';

import { useAniListStore } from '@/stores/providers/anilist';
import { useMyAnimeListStore } from '@/stores/providers/myanimelist';
import { useProviderStore } from '@/stores/providers/provider';
import { Provider } from '@/types/List';
import { mapProviderToName } from '@/utils/provider';
import { buildProfileUrl } from '@/utils/url';

type MenuItem = {
  label: string;
  icon: ReactNode;
  renderDivider: boolean;
  handleClick: (event: MouseEvent<HTMLElement>) => void;
};

const useProfileMenu = () => {
  const [mainPopoverEl, setMainPopoverEl] = useState<null | HTMLElement>(null);
  const [switchAccountEl, setSwitchAccountEl] = useState<null | HTMLElement>(
    null
  );

  const activeProvider = useProviderStore((state) => state.activeProvider);
  const setActiveProvider = useProviderStore(
    (state) => state.setActiveProvider
  );

  const isMyAnimeListAuthenticated = useMyAnimeListStore(
    (state) => state.isAuthenticated
  );
  const myAnimeListUsername = useMyAnimeListStore((state) => state.username);
  const myAnimeListProfilePictureUrl = useMyAnimeListStore(
    (state) => state.profilePictureUrl
  );
  const signOutMyAnimeList = useMyAnimeListStore((state) => state.signOut);

  const isAniListAuthenticated = useAniListStore(
    (state) => state.isAuthenticated
  );
  const anilistUsername = useAniListStore((state) => state.username);
  const anilistProfilePictureUrl = useAniListStore(
    (state) => state.profilePictureUrl
  );
  const signOutAniList = useAniListStore((state) => state.signOut);

  const mainPopoverOpen = Boolean(mainPopoverEl);
  const switchAccountOpen = Boolean(switchAccountEl);

  const profileImage = useMemo(() => {
    switch (activeProvider) {
      case Provider.MY_ANIME_LIST:
        return myAnimeListProfilePictureUrl;
      case Provider.ANILIST:
        return anilistProfilePictureUrl;
      default:
        return null;
    }
  }, [activeProvider, myAnimeListProfilePictureUrl, anilistProfilePictureUrl]);

  const username = useMemo(() => {
    switch (activeProvider) {
      case Provider.MY_ANIME_LIST:
        return myAnimeListUsername;
      case Provider.ANILIST:
        return anilistUsername;
      default:
        return 'Not Logged In';
    }
  }, [activeProvider, myAnimeListUsername, anilistUsername]);

  const providerName = useMemo(() => {
    if (!activeProvider) return 'Unknown';
    return mapProviderToName(activeProvider);
  }, [activeProvider]);

  const handleSignOut = useCallback(() => {
    switch (activeProvider) {
      case Provider.MY_ANIME_LIST:
        if (isAniListAuthenticated) {
          setActiveProvider(Provider.ANILIST);
        } else {
          setActiveProvider(null);
        }

        signOutMyAnimeList();
        break;
      case Provider.ANILIST:
        if (isMyAnimeListAuthenticated) {
          setActiveProvider(Provider.MY_ANIME_LIST);
        } else {
          setActiveProvider(null);
        }

        signOutAniList();
        break;
      default:
        break;
    }
  }, [
    activeProvider,
    isAniListAuthenticated,
    isMyAnimeListAuthenticated,
    setActiveProvider,
    signOutMyAnimeList,
    signOutAniList
  ]);

  const handleOpenMainPopover = (event: MouseEvent<HTMLElement>) => {
    if (!activeProvider) return;
    setMainPopoverEl(event.currentTarget);
  };

  const handleCloseMainPopover = () => {
    setMainPopoverEl(null);
  };

  const handleOpenSwitchAccountPopover = (event: MouseEvent<HTMLElement>) => {
    setSwitchAccountEl(event.currentTarget);
  };

  const handleCloseSwitchAccountPopover = () => {
    setSwitchAccountEl(null);
  };

  const handleSwitchAccount = (provider: Provider) => {
    setActiveProvider(provider);
    handleCloseSwitchAccountPopover();
    handleCloseMainPopover();
  };

  const connectedAccounts = useMemo(() => {
    const accounts = [];

    if (isMyAnimeListAuthenticated) {
      accounts.push(Provider.MY_ANIME_LIST);
    }

    if (isAniListAuthenticated) {
      accounts.push(Provider.ANILIST);
    }

    return accounts;
  }, [isMyAnimeListAuthenticated, isAniListAuthenticated]);

  const menuItems: MenuItem[] = useMemo(() => {
    return [
      {
        label: 'My Profile',
        icon: <User />,
        renderDivider: false,
        handleClick: async () => {
          if (activeProvider && username) {
            const profileUrl = buildProfileUrl(activeProvider, username);

            if (profileUrl) {
              await openUrl(profileUrl);
            }
          }
          handleCloseMainPopover();
        }
      },
      {
        label: 'Switch Account',
        icon: <ArrowLeftRight />,
        renderDivider: true,
        handleClick: (event: MouseEvent<HTMLElement>) => {
          if (connectedAccounts.length > 0) {
            return handleOpenSwitchAccountPopover(event);
          }
          handleCloseMainPopover();
        }
      },
      {
        label: 'Sign Out',
        icon: <LogOut />,
        renderDivider: false,
        handleClick: handleSignOut
      }
    ];
  }, [activeProvider, username, connectedAccounts, handleSignOut]);

  return {
    mainPopoverEl,
    mainPopoverOpen,
    switchAccountEl,
    switchAccountOpen,
    profileImage,
    providerName,
    username,
    menuItems,
    connectedAccounts,
    handleOpenMainPopover,
    handleCloseMainPopover,
    handleOpenSwitchAccountPopover,
    handleCloseSwitchAccountPopover,
    handleSwitchAccount,
    handleSignOut
  };
};

export default useProfileMenu;
