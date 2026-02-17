import { Provider } from '@/types/List';

export const mapProviderToName = (provider: Provider) => {
  switch (provider) {
    case Provider.ANILIST:
      return 'AniList';
    case Provider.MY_ANIME_LIST:
      return 'MyAnimeList';
    default:
      return 'Unknown';
  }
};
