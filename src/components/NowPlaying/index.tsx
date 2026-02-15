import Button from '@/components/ui/Button';
import { useNowPlayingAliasesStore } from '@/stores/nowPlayingAliases';
import { usePlayerDetectionStore } from '@/stores/playerDetection';
import { useMyAnimeListStore } from '@/stores/providers/myanimelist';
import { Provider } from '@/types/List';
import { buildEntityUrl } from '@/utils/url';
import { Box, Container, Grid, Typography } from '@mui/material';
import { useMemo } from 'react';
import AnimeTitle from '../AnimeInformations/components/AnimeTitle';
import MainInformation from '../AnimeInformations/components/MainInformation';
import AnimeCover from '../ui/AnimeCover';

const NowPlaying = () => {
  const animePlaying = usePlayerDetectionStore((state) => state.activeEpisode);
  const activeMatchedAnimeId = usePlayerDetectionStore(
    (state) => state.activeMatchedAnimeId
  );
  const suggestedAnimeIds = usePlayerDetectionStore(
    (state) => state.suggestedAnimeIds
  );
  const resolveActiveAnime = usePlayerDetectionStore(
    (state) => state.resolveActiveAnime
  );
  const animeListData = useMyAnimeListStore((state) => state.animeListData);
  const addAlias = useNowPlayingAliasesStore((state) => state.addAlias);

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
    if (!activeMatchedAnimeId) {
      return undefined;
    }

    return aggregatedData.find((anime) => anime.id === activeMatchedAnimeId);
  }, [activeMatchedAnimeId, aggregatedData]);

  const suggestedMatches = useMemo(() => {
    if (!animePlaying || exactAnimeMatch) {
      return [];
    }

    const animeById = new Map(aggregatedData.map((anime) => [anime.id, anime]));

    return suggestedAnimeIds
      .map((animeId) => animeById.get(animeId))
      .filter((anime): anime is (typeof aggregatedData)[number] =>
        Boolean(anime)
      );
  }, [aggregatedData, animePlaying, exactAnimeMatch, suggestedAnimeIds]);

  const handleSuggestionClick = (animeId: number) => {
    if (!animePlaying?.animeTitle) {
      return;
    }

    addAlias(animeId, animePlaying.animeTitle);
    resolveActiveAnime(animeId);
  };

  return (
    <Container className="h-full min-h-0 flex flex-col py-4">
      <Typography variant="h6" component="h1" color="primary" gutterBottom>
        Now Playing
      </Typography>

      <Box className="flex-1 min-h-0 overflow-y-auto pr-1">
        {!animePlaying && (
          <Typography variant="body2" color="textSecondary">
            No episode detected right now.
          </Typography>
        )}

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
                url={buildEntityUrl(
                  Provider.MY_ANIME_LIST,
                  'anime',
                  exactAnimeMatch.id
                )}
              />
            </Grid>

            <Grid size={{ xs: 12, sm: 'grow' }} className="min-w-0">
              <div className="flex flex-col gap-1 pb-2.5">
                <AnimeTitle
                  url={buildEntityUrl(
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

        {animePlaying && !exactAnimeMatch && suggestedMatches.length > 0 && (
          <Box className="flex flex-col gap-2">
            <Typography variant="subtitle2">
              We could not find an exact match. Select the correct anime:
            </Typography>

            <Box className="flex flex-wrap gap-2">
              {suggestedMatches.map((anime) => (
                <Button
                  key={anime.id}
                  variant="secondary"
                  size="small"
                  onClick={() => handleSuggestionClick(anime.id)}>
                  {anime.title}
                </Button>
              ))}
            </Box>
          </Box>
        )}

        {animePlaying && !exactAnimeMatch && suggestedMatches.length === 0 && (
          <Typography variant="body2" color="textSecondary">
            No close matches found in your list for this detected title.
          </Typography>
        )}
      </Box>
    </Container>
  );
};

export default NowPlaying;
