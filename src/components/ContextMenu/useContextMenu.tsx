import { ChevronRight } from 'lucide-react';
import {
  type MouseEvent,
  ReactNode,
  useCallback,
  useMemo,
  useState
} from 'react';
import { useLocation } from 'react-router';

import { PathName } from '@/routes';
import { useAnimeDetailsStore } from '@/stores/animeDetails';
import { useContextMenuStore } from '@/stores/contextMenu';
import { useMangaDetailsStore } from '@/stores/mangaDetails';
import { useProviderStore } from '@/stores/providers/provider';
import { IAnimeList } from '@/types/AnimeList';
import { IMangaList } from '@/types/MangaList';

type MenuItem = {
  label: string;
  disabled: boolean;
  shouldRender: boolean;
  renderDivider: boolean;
  rightIcon?: ReactNode;
  handleClick?: (event: MouseEvent<HTMLElement>) => void;
};

const useContextMenu = () => {
  const [addToListAnchorEl, setAddToListAnchorEl] =
    useState<HTMLElement | null>(null);

  const location = useLocation();

  const popoverPosition = useContextMenuStore((state) => state.popoverPosition);
  const state = useContextMenuStore((state) => state.state);
  const close = useContextMenuStore((state) => state.closeContextMenu);

  const openAnimeDetails = useAnimeDetailsStore((state) => state.setIsOpen);
  const setSelectedAnime = useAnimeDetailsStore(
    (state) => state.setSelectedAnime
  );

  const openMangaDetails = useMangaDetailsStore((state) => state.setIsOpen);
  const setSelectedManga = useMangaDetailsStore(
    (state) => state.setSelectedManga
  );

  const selectedListType = useProviderStore((state) => state.selectedListType);

  const handleOpenInformation = useCallback(() => {
    switch (selectedListType) {
      case 'anime':
        setSelectedAnime(state as IAnimeList);
        openAnimeDetails(true);
        break;
      case 'manga':
        setSelectedManga(state as IMangaList);
        openMangaDetails(true);
        break;
      default:
        break;
    }

    close();
  }, [
    state,
    selectedListType,
    setSelectedAnime,
    openAnimeDetails,
    setSelectedManga,
    openMangaDetails,
    close
  ]);

  const closeAddToListMenu = useCallback(() => {
    setAddToListAnchorEl(null);
    close();
  }, [setAddToListAnchorEl, close]);

  const menuItems: MenuItem[] = useMemo(() => {
    return [
      {
        label: 'Information',
        disabled: false,
        shouldRender: true,
        renderDivider: location.pathname === PathName.SEARCH,
        handleClick: handleOpenInformation
      },
      {
        label: 'Add to list',
        disabled: false,
        shouldRender: location.pathname === PathName.SEARCH,
        rightIcon: <ChevronRight size={16} />,
        renderDivider: false,
        handleClick: (event: MouseEvent<HTMLElement>) => {
          setAddToListAnchorEl(event.currentTarget);
        }
      }
    ].filter((item) => item.shouldRender);
  }, [location, handleOpenInformation]);

  return {
    popoverPosition,
    state,
    menuItems,
    addToListAnchorEl,
    close,
    closeAddToListMenu
  };
};

export default useContextMenu;
