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
import { openUrl } from '@tauri-apps/plugin-opener';
import { X } from 'lucide-react';
import { useCallback, useState } from 'react';

import { useAnimeDetailsStore } from '@/stores/animeDetails';
import { Provider } from '@/types/List';
import { buildUrl } from '@/utils/url';
import Button from '../ui/Button';
import AnimeListForm from './components/AnimeListForm';
import Details from './components/Details';
import InfoHeader from './components/InfoHeader';

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

  const handleSubmit = useCallback(() => {
    if (!formRef?.current) return;

    formRef.current.requestSubmit();
  }, [formRef]);

  if (!selectedAnime) return null;

  const {
    imageUrl,
    title,
    alternativeTitles,
    mediaType,
    totalEpisodes,
    status,
    startSeason,
    genres,
    studios,
    score,
    synopsis
  } = selectedAnime;

  const handleClose = () => {
    setIsOpen(false);
    setSelectedAnime(null);
    setSelectedTab(0);
  };

  const handleOpenInBrowser = async () => {
    await openUrl(buildUrl(Provider.MY_ANIME_LIST, 'anime', selectedAnime.id));
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
            <div className="p-0.5 border border-dashed border-primary rounded-md inline-block">
              <img
                className="w-36 h-52 object-fill rounded-md cursor-pointer"
                src={imageUrl}
                alt={title}
                onClick={handleOpenInBrowser}
              />
            </div>
          </Grid>

          <Grid size={{ xs: 12, sm: 'grow' }} className="min-w-0">
            <div className="flex flex-col gap-1">
              <div className="flex">
                <span className="cursor-pointer" onClick={handleOpenInBrowser}>
                  <Typography variant="body1" color="primary">
                    {title}
                  </Typography>
                </span>
              </div>

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

                {selectedTab === 0 && (
                  <div>
                    <InfoHeader label="Alternative titles" />

                    <Typography className="py-2!" variant="body2">
                      {alternativeTitles}
                    </Typography>

                    <InfoHeader label="Details" />

                    <Details
                      mediaType={mediaType}
                      status={status}
                      totalEpisodes={totalEpisodes}
                      startSeason={startSeason}
                      genres={genres}
                      studios={studios}
                      score={score}
                    />

                    <InfoHeader label="Synopsis" />

                    <Typography
                      className="py-2!"
                      variant="body2"
                      sx={{ whiteSpace: 'pre-line' }}>
                      {synopsis}
                    </Typography>
                  </div>
                )}

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
