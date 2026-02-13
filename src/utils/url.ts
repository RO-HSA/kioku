import { ListType } from '@/types/List';

export const buildUrl = (provider: string, type: ListType, id: number) => {
  switch (provider) {
    case 'myanimelist':
      return `https://myanimelist.net/${type}/${id}`;
    default:
      return `https://myanimelist.net/${type}/${id}`;
  }
};
