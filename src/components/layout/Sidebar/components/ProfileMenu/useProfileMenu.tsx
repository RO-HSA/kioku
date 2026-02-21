import { openUrl } from '@tauri-apps/plugin-opener';
import { ArrowLeftRight, LogOut, User } from 'lucide-react';
import {
  ReactNode,
  useCallback,
  useMemo,
  useState,
  type MouseEvent
} from 'react';

import { calculatePlaybackMatches } from '@/hooks/detection/utils';
import { useConfigMenuStore } from '@/stores/config/configMenu';
import { useNowPlayingAliasesStore } from '@/stores/detection/nowPlayingAliases';
import { usePlayerDetectionStore } from '@/stores/detection/playerDetection';
import { useAniListStore } from '@/stores/providers/anilist';
import { useMyAnimeListStore } from '@/stores/providers/myanimelist';
import { useProviderStore } from '@/stores/providers/provider';
import { Provider } from '@/types/List';
import { ConfigMenuStep } from '@/types/Navigation';
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
  const myAnimeListAnimeData = useMyAnimeListStore(
    (state) => state.animeListData
  );
  const signOutMyAnimeList = useMyAnimeListStore((state) => state.signOut);

  const isAniListAuthenticated = useAniListStore(
    (state) => state.isAuthenticated
  );
  const anilistUsername = useAniListStore((state) => state.username);
  const anilistProfilePictureUrl = useAniListStore(
    (state) => state.profilePictureUrl
  );
  const aniListAnimeData = useAniListStore((state) => state.animeListData);
  const signOutAniList = useAniListStore((state) => state.signOut);

  const detection = usePlayerDetectionStore((state) => state.activeEpisode);
  const setMatchingResult = usePlayerDetectionStore(
    (state) => state.setMatchingResult
  );

  const getAliasesByProvider = useNowPlayingAliasesStore(
    (state) => state.getAliasesByProvider
  );

  const setStep = useConfigMenuStore((state) => state.setStep);
  const setSelectedTab = useConfigMenuStore((state) => state.setSelectedTab);
  const openConfigMenu = useConfigMenuStore((state) => state.openConfigMenu);

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
    if (!activeProvider) {
      setSelectedTab(0);
      setStep(ConfigMenuStep.INTEGRATIONS);
      openConfigMenu();
      return;
    }
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

  const handleSwitchAccount = useCallback(
    (provider: Provider) => {
      if (provider === activeProvider) {
        return handleCloseSwitchAccountPopover();
      }

      setActiveProvider(provider);
      handleCloseSwitchAccountPopover();
      handleCloseMainPopover();

      if (!detection) return;

      switch (provider) {
        case Provider.MY_ANIME_LIST:
          if (!myAnimeListAnimeData) return;

          calculatePlaybackMatches({
            animeListData: myAnimeListAnimeData,
            animeTitle: detection.animeTitle,
            episodeNumber: detection.episode,
            aliasesByAnimeId: getAliasesByProvider(Provider.MY_ANIME_LIST),
            setMatchingResult
          });
          break;
        case Provider.ANILIST:
          if (!aniListAnimeData) return;

          calculatePlaybackMatches({
            animeListData: aniListAnimeData,
            animeTitle: detection.animeTitle,
            episodeNumber: detection.episode,
            aliasesByAnimeId: getAliasesByProvider(Provider.ANILIST),
            setMatchingResult
          });
          break;
        default:
          return;
      }
    },
    [
      activeProvider,
      detection,
      myAnimeListAnimeData,
      aniListAnimeData,
      setActiveProvider,
      handleCloseSwitchAccountPopover,
      handleCloseMainPopover,
      setMatchingResult,
      getAliasesByProvider
    ]
  );

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
