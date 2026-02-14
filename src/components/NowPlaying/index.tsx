import { usePlayerDetectionStore } from '@/stores/playerDetection';
import { useMyAnimeListStore } from '@/stores/providers/myanimelist';
import { Provider } from '@/types/List';
import { buildUrl } from '@/utils/url';
import { Container, Grid, Typography } from '@mui/material';
import { useMemo } from 'react';
import AnimeTitle from '../AnimeInformations/components/AnimeTitle';
import MainInformation from '../AnimeInformations/components/MainInformation';
import AnimeCover from '../ui/AnimeCover';

const NowPlaying = () => {
  const animePlaying = usePlayerDetectionStore((state) => state.activeEpisode);
  const animeListData = useMyAnimeListStore((state) => state.animeListData);

  const aggregatedData = useMemo(() => {
    if (!animeListData) return [];

    return [
      ...animeListData.watching,
      ...animeListData.completed,
      ...animeListData.onHold,
      ...animeListData.dropped,
      ...animeListData.planToWatch
    ];
  }, [animeListData]);

  const exactAnimeMatch = useMemo(() => {
    return aggregatedData.find(
      (anime) =>
        anime.title.toLowerCase().trim() ===
          animePlaying?.animeTitle.toLowerCase().trim() ||
        anime.alternativeTitles
          .toLowerCase()
          .includes(
            animePlaying?.animeTitle.toLowerCase().trim() || '-9999999999'
          )
    );
  }, [aggregatedData, animePlaying]);

  return (
    <Container>
      <Typography variant="h6" component="h1" color="primary" gutterBottom>
        Now Playing
      </Typography>

      {exactAnimeMatch && (
        <Grid container spacing={2} width="100%!important" size="auto">
          <Grid
            size={{ xs: 12, sm: 'auto' }}
            sx={{
              width: { sm: 150 },
              flexBasis: { sm: 150 },
              maxWidth: { sm: 150 }
            }}>
            <AnimeCover
              title={exactAnimeMatch.title}
              imageUrl={exactAnimeMatch.imageUrl}
              url={buildUrl(
                Provider.MY_ANIME_LIST,
                'anime',
                exactAnimeMatch.id
              )}
            />
          </Grid>

          <Grid size={{ xs: 12, sm: 'grow' }} className="min-w-0">
            <div className="flex flex-col gap-1">
              <AnimeTitle
                url={buildUrl(
                  Provider.MY_ANIME_LIST,
                  'anime',
                  exactAnimeMatch.id
                )}>
                {exactAnimeMatch.title}
              </AnimeTitle>
            </div>

            <div className="flex flex-col gap-2 self-start w-full">
              <MainInformation anime={exactAnimeMatch} />
            </div>
          </Grid>
        </Grid>
      )}
    </Container>
  );
};

export default NowPlaying;
