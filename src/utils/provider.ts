import { Provider } from '@/types/List';

export const mapProviderToDisplayName = (provider: Provider) => {
  switch (provider) {
    case Provider.MY_ANIME_LIST:
      return 'MyAnimeList';
    case Provider.ANILIST:
      return 'AniList';
    default:
      return provider;
  }
};
