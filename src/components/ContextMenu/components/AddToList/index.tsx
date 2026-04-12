import { ListItemText, Menu, MenuItem } from '@mui/material';
import { FC, useCallback, useMemo } from 'react';

import { useContextMenuStore } from '@/stores/contextMenu';
import { useAniListStore } from '@/stores/providers/anilist';
import { useMyAnimeListStore } from '@/stores/providers/myanimelist';
import { useProviderStore } from '@/stores/providers/provider';
import { AnimeListUserStatus, IAnimeList } from '@/types/AnimeList';
import { Provider } from '@/types/List';
import { IMangaList, MangaListUserStatus } from '@/types/MangaList';

type Options = {
  label: string;
  status: AnimeListUserStatus | MangaListUserStatus;
};

interface AddToListProps {
  anchorEl: HTMLElement | null;
  open: boolean;
  onClose: () => void;
}

const AddToList: FC<AddToListProps> = ({ anchorEl, open, onClose }) => {
  const activeProvider = useProviderStore((state) => state.activeProvider);
  const selectedListType = useProviderStore((state) => state.selectedListType);

  const state = useContextMenuStore((state) => state.state);
  const close = useContextMenuStore((state) => state.closeContextMenu);

  const addToMyAnimeListAnimeList = useMyAnimeListStore(
    (state) => state.addToAnimeList
  );
  const addToMyAnimeListMangaList = useMyAnimeListStore(
    (state) => state.addToMangaList
  );

  const addToAnilistAnimeList = useAniListStore(
    (state) => state.addToAnimeList
  );
  const addToAnilistMangaList = useAniListStore(
    (state) => state.addToMangaList
  );

  const addToList = useCallback(
    (status: AnimeListUserStatus | MangaListUserStatus) => {
      if (!state) return;

      switch (activeProvider) {
        case Provider.MY_ANIME_LIST:
          if (selectedListType === 'anime') {
            addToMyAnimeListAnimeList({
              ...state,
              userStatus: status
            } as IAnimeList);
          } else {
            addToMyAnimeListMangaList({
              ...state,
              userStatus: status
            } as IMangaList);
          }
          break;
        case Provider.ANILIST:
          if (selectedListType === 'anime') {
            addToAnilistAnimeList({
              ...state,
              userStatus: status
            } as IAnimeList);
          } else {
            addToAnilistMangaList({
              ...state,
              userStatus: status
            } as IMangaList);
          }
          break;
        default:
          break;
      }

      close();
    },
    [
      activeProvider,
      state,
      addToMyAnimeListAnimeList,
      addToMyAnimeListMangaList,
      addToAnilistAnimeList,
      addToAnilistMangaList
    ]
  );

  const options: Options[] = useMemo(() => {
    return [
      {
        label: `Currently ${selectedListType === 'anime' ? 'watching' : 'reading'}`,
        status: selectedListType === 'anime' ? 'watching' : 'reading'
      },
      {
        label: 'Completed',
        status: 'completed'
      },
      {
        label: 'On hold',
        status: 'onHold'
      },
      {
        label: 'Dropped',
        status: 'dropped'
      },
      {
        label: `Plan to ${selectedListType === 'anime' ? 'watch' : 'read'}`,
        status: selectedListType === 'anime' ? 'planToWatch' : 'planToRead'
      }
    ];
  }, [selectedListType]);
  return (
    <Menu
      anchorEl={anchorEl}
      open={open}
      onClose={onClose}
      anchorOrigin={{
        vertical: 'top',
        horizontal: 'right'
      }}
      transformOrigin={{
        vertical: 'top',
        horizontal: 'left'
      }}
      slotProps={{
        paper: {
          sx: {
            mt: 1.5,
            '&::before': {
              content: '""',
              display: 'block',
              position: 'absolute',
              top: 0,
              right: 14,
              width: 10,
              height: 10,
              bgcolor: 'background.paper',
              transform: 'translateY(-50%) rotate(45deg)',
              zIndex: 0
            }
          }
        }
      }}>
      {options.map(({ label, status }) => (
        <MenuItem key={label} dense onClick={() => addToList(status)}>
          <ListItemText>{label}</ListItemText>
        </MenuItem>
      ))}
    </Menu>
  );
};

export default AddToList;
