import useAnilistCallback from '@/hooks/integrations/useAnilistCallback';
import useMyanimelistCallback from '@/hooks/integrations/useMyanimelistCallback';
import { AniListService } from '@/services/backend/AniList';
import { MyAnimeListService } from '@/services/backend/MyAnimeList';
import { useConfigMenuStore } from '@/stores/config/configMenu';
import { useAniListStore } from '@/stores/providers/anilist';
import { useMyAnimeListStore } from '@/stores/providers/myanimelist';
import { Provider } from '@/types/List';
import MenuItem from '../MenuItem';
import Section from '../Section';
import OauthForm from './OauthForm';

const tabs = ['MyAnimeList', 'AniList'];

const Integrations = () => {
  useMyanimelistCallback();
  useAnilistCallback();

  const selectedTab = useConfigMenuStore((state) => state.selectedTab);

  const myAnimeListUsername = useMyAnimeListStore((state) => state.username);
  const isMyAnimeListAuthenticated = useMyAnimeListStore(
    (state) => state.isAuthenticated
  );
  const isAuthenticatingMyAnimeList = useMyAnimeListStore(
    (state) => state.isAuthenticating
  );
  const isReauthenticatingMyAnimeList = useMyAnimeListStore(
    (state) => state.isReauthenticating
  );
  const setMyAnimeListUsername = useMyAnimeListStore(
    (state) => state.setUsername
  );
  const setIsAuthenticatingMyAnimeList = useMyAnimeListStore(
    (state) => state.setIsAuthenticating
  );
  const setIsReauthenticatingMyAnimeList = useMyAnimeListStore(
    (state) => state.setIsReauthenticating
  );

  const aniListUsername = useAniListStore((state) => state.username);
  const isAniListAuthenticated = useAniListStore(
    (state) => state.isAuthenticated
  );
  const isAuthenticatingAniList = useAniListStore(
    (state) => state.isAuthenticating
  );
  const isReauthenticatingAniList = useAniListStore(
    (state) => state.isReauthenticating
  );
  const setAniListUsername = useAniListStore((state) => state.setUsername);
  const setIsAuthenticatingAniList = useAniListStore(
    (state) => state.setIsAuthenticating
  );
  const setIsReauthenticatingAniList = useAniListStore(
    (state) => state.setIsReauthenticating
  );

  return (
    <MenuItem tabs={tabs}>
      {selectedTab === 0 && (
        <Section title="Account">
          <OauthForm
            provider={Provider.MY_ANIME_LIST}
            username={myAnimeListUsername}
            setUsername={setMyAnimeListUsername}
            isAuthenticated={isMyAnimeListAuthenticated}
            isAuthenticating={isAuthenticatingMyAnimeList}
            isReauthenticating={isReauthenticatingMyAnimeList}
            setIsAuthenticating={setIsAuthenticatingMyAnimeList}
            setIsReauthenticating={setIsReauthenticatingMyAnimeList}
            authorizeFn={MyAnimeListService.authorize}
          />
        </Section>
      )}

      {selectedTab === 1 && (
        <Section title="Account">
          <OauthForm
            provider={Provider.ANILIST}
            username={aniListUsername}
            setUsername={setAniListUsername}
            isAuthenticated={isAniListAuthenticated}
            isAuthenticating={isAuthenticatingAniList}
            isReauthenticating={isReauthenticatingAniList}
            setIsAuthenticating={setIsAuthenticatingAniList}
            setIsReauthenticating={setIsReauthenticatingAniList}
            authorizeFn={AniListService.authorize}
          />
        </Section>
      )}
    </MenuItem>
  );
};

export default Integrations;
