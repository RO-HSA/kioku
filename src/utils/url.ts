import { ListType, Provider } from '@/types/List';

export const buildEntityUrl = (
  provider: Provider,
  type: ListType,
  id: number
) => {
  switch (provider) {
    case Provider.MY_ANIME_LIST:
      return `https://myanimelist.net/${type}/${id}`;
    case Provider.ANILIST:
      return `https://anilist.co/${type}/${id}`;
    default:
      return `https://myanimelist.net/${type}/${id}`;
  }
};

export const buildRegisterUrl = (provider: Provider) => {
  switch (provider) {
    case Provider.MY_ANIME_LIST:
      return 'https://myanimelist.net/register.php';
    case Provider.ANILIST:
      return 'https://anilist.co/signup';
    default:
      return 'https://myanimelist.net/register.php';
  }
};

export const buildProfileUrl = (provider: Provider, username: string) => {
  switch (provider) {
    case Provider.MY_ANIME_LIST:
      return `https://myanimelist.net/profile/${username}`;
    case Provider.ANILIST:
      return `https://anilist.co/user/${username}`;
    default:
      return '';
  }
};
