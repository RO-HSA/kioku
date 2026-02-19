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
import { useCallback, useState } from 'react';

import { useAnimeDetailsStore } from '@/stores/animeDetails';
import { useProviderStore } from '@/stores/providers/provider';
import { Provider } from '@/types/List';
import { buildEntityUrl } from '@/utils/url';
import AnimeCover from '../ui/AnimeCover';
import Button from '../ui/Button';
import AnimeListForm from './components/AnimeListForm';
import AnimeTitle from './components/AnimeTitle';
import MainInformation from './components/MainInformation';

const tabs = ['Informations', 'List Settings'];

const AnimeInformations = () => {
  const [selectedTab, setSelectedTab] = useState(0);

  const selectedAnime = useAnimeDetailsStore((state) => state.selectedAnime);
  const isOpen = useAnimeDetailsStore((state) => state.isOpen);
  const formRef = useAnimeDetailsStore((state) => state.formRef);
  const setIsOpen = useAnimeDetailsStore((state) => state.setIsOpen);
  const setSelectedAnime = useAnimeDetailsStore(
    (state) => state.setSelectedAnime
  );

  const activeProvider = useProviderStore((state) => state.activeProvider);

  const handleSubmit = useCallback(() => {
    if (!formRef?.current) return;

    formRef.current.requestSubmit();
  }, [formRef]);

  if (!selectedAnime) return null;

  const { imageUrl, title } = selectedAnime;

  const handleClose = () => {
    setIsOpen(false);
    setSelectedAnime(null);
    setSelectedTab(0);
  };

  return (
    <Dialog open={isOpen} onClose={handleClose} fullWidth maxWidth="md">
      <DialogTitle className="flex justify-between items-center">
        <Typography variant="body1">Anime Informations</Typography>

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
                'anime',
                selectedAnime.id
              )}
            />
          </Grid>

          <Grid size={{ xs: 12, sm: 'grow' }} className="min-w-0">
            <div className="flex flex-col gap-1">
              <AnimeTitle
                url={buildEntityUrl(
                  activeProvider || Provider.MY_ANIME_LIST,
                  'anime',
                  selectedAnime.id
                )}>
                {title}
              </AnimeTitle>

              <div className="flex flex-col gap-2 self-start w-full">
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

                {selectedTab === 0 && <MainInformation anime={selectedAnime} />}

                {selectedTab === 1 && (
                  <div className="pt-2">
                    <AnimeListForm />
                  </div>
                )}
              </div>
            </div>
          </Grid>
        </Grid>
      </DialogContent>

      <DialogActions>
        <Button type="submit" variant="primary" onClick={handleSubmit}>
          Confirm
        </Button>

        <Button onClick={handleClose} variant="secondary">
          Close
        </Button>
      </DialogActions>
    </Dialog>
  );
};

export default AnimeInformations;
