import {
  Dialog,
  DialogActions,
  DialogContent,
  DialogTitle,
  Grid,
  IconButton,
  Tab,
  Tabs,
  Typography
} from '@mui/material';
import { X } from 'lucide-react';
import { useCallback, useMemo, useState } from 'react';

import { useMangaDetailsStore } from '@/stores/mangaDetails';
import { useAniListStore } from '@/stores/providers/anilist';
import { useMyAnimeListStore } from '@/stores/providers/myanimelist';
import { useProviderStore } from '@/stores/providers/provider';
import { Provider } from '@/types/List';
import { buildEntityUrl } from '@/utils/url';
import AnimeCover from '../../ui/AnimeCover';
import Button from '../../ui/Button';
import MainInformation from './components/MainInformation';
import MangaListForm from './components/MangaListForm';
import MangaTitle from './components/MangaTitle';

const tabs = ['Informations', 'List Settings'];

const MangaInformations = () => {
  const [selectedTab, setSelectedTab] = useState(0);

  const selectedManga = useMangaDetailsStore((state) => state.selectedManga);
  const isOpen = useMangaDetailsStore((state) => state.isOpen);
  const formRef = useMangaDetailsStore((state) => state.formRef);
  const setIsOpen = useMangaDetailsStore((state) => state.setIsOpen);
  const setSelectedManga = useMangaDetailsStore(
    (state) => state.setSelectedManga
  );

  const myAnimeListMangaData = useMyAnimeListStore(
    (state) => state.mangaListData
  );
  const addToMyAnimeList = useMyAnimeListStore((state) => state.addToMangaList);

  const anilistMangaData = useAniListStore((state) => state.mangaListData);
  const addToAnilist = useAniListStore((state) => state.addToMangaList);

  const activeProvider = useProviderStore((state) => state.activeProvider);

  const currentMangaList = useMemo(() => {
    if (!selectedManga) return [];

    switch (activeProvider) {
      case Provider.MY_ANIME_LIST:
        if (!myAnimeListMangaData) return [];

        return [
          ...myAnimeListMangaData.reading,
          ...myAnimeListMangaData.completed,
          ...myAnimeListMangaData.onHold,
          ...myAnimeListMangaData.dropped,
          ...myAnimeListMangaData.planToRead
        ];
      case Provider.ANILIST:
        if (!anilistMangaData) return [];

        return [
          ...anilistMangaData.reading,
          ...anilistMangaData.completed,
          ...anilistMangaData.onHold,
          ...anilistMangaData.dropped,
          ...anilistMangaData.planToRead
        ];
      default:
        return [];
    }
  }, [myAnimeListMangaData, anilistMangaData, activeProvider, selectedManga]);

  const addToList = useCallback(() => {
    if (!selectedManga) return;

    switch (activeProvider) {
      case Provider.MY_ANIME_LIST:
        addToMyAnimeList(selectedManga);
        break;
      case Provider.ANILIST:
        addToAnilist(selectedManga);
        break;
      default:
        break;
    }
  }, [activeProvider, selectedManga, addToMyAnimeList, addToAnilist]);

  const handleSubmit = useCallback(() => {
    if (!formRef?.current) return;

    formRef.current.requestSubmit();
  }, [formRef]);

  const isAdded = useMemo(() => {
    if (!selectedManga) return false;

    return !!currentMangaList.find((manga) =>
      manga.entryId !== undefined
        ? manga.entryId === selectedManga.entryId
        : manga.id === selectedManga.id
    );
  }, [currentMangaList, selectedManga]);

  if (!selectedManga) return null;

  const { imageUrl, title } = selectedManga;

  const handleClose = () => {
    setIsOpen(false);
    setSelectedManga(null);
    setSelectedTab(0);
  };

  return (
    <Dialog open={isOpen} onClose={handleClose} fullWidth maxWidth="md">
      <DialogTitle className="flex justify-between items-center">
        <Typography variant="body1">Manga Informations</Typography>

        <IconButton onClick={handleClose}>
          <X />
        </IconButton>
      </DialogTitle>

      <DialogContent>
        <Grid container spacing={2} width="100%!important" size="auto">
          <Grid
            size={{ xs: 12, sm: 'auto' }}
            sx={{
              width: { sm: 150 },
              flexBasis: { sm: 150 },
              maxWidth: { sm: 150 }
            }}>
            <AnimeCover
              title={title}
              imageUrl={imageUrl}
              url={buildEntityUrl(
                Provider.MY_ANIME_LIST,
                'manga',
                selectedManga.id
              )}
            />
          </Grid>

          <Grid size={{ xs: 12, sm: 'grow' }} className="min-w-0">
            <div className="flex flex-col gap-1">
              <MangaTitle
                url={buildEntityUrl(
                  activeProvider || Provider.MY_ANIME_LIST,
                  'manga',
                  selectedManga.id
                )}>
                {title}
              </MangaTitle>

              <div className="flex flex-col gap-2 self-start w-full">
                {isAdded ? (
                  <>
                    <Tabs
                      value={selectedTab}
                      onChange={(_, newValue) => setSelectedTab(newValue)}>
                      {tabs.map((tab) => (
                        <Tab
                          key={tab}
                          label={tab}
                          className="p-1! text-sm!"
                          sx={{
                            textTransform: 'none',
                            '&:hover': { color: 'GrayText' }
                          }}
                        />
                      ))}
                    </Tabs>

                    {selectedTab === 0 && (
                      <MainInformation manga={selectedManga} />
                    )}

                    {selectedTab === 1 && (
                      <div className="pt-2">
                        <MangaListForm />
                      </div>
                    )}
                  </>
                ) : (
                  <>
                    <Typography
                      variant="body2"
                      color="success"
                      sx={{
                        cursor: 'pointer',
                        '&:hover': { opacity: 0.8 }
                      }}
                      onClick={addToList}>
                      Add to list
                    </Typography>
                    <MainInformation manga={selectedManga} />
                  </>
                )}
              </div>
            </div>
          </Grid>
        </Grid>
      </DialogContent>

      {isAdded && (
        <DialogActions>
          <>
            <Button type="submit" variant="primary" onClick={handleSubmit}>
              Confirm
            </Button>

            <Button onClick={handleClose} variant="secondary">
              Close
            </Button>
          </>
        </DialogActions>
      )}
    </Dialog>
  );
};

export default MangaInformations;
