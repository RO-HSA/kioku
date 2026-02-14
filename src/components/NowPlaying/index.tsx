import Button from '@/components/ui/Button';
import { useNowPlayingAliasesStore } from '@/stores/nowPlayingAliases';
import { usePlayerDetectionStore } from '@/stores/playerDetection';
import { useMyAnimeListStore } from '@/stores/providers/myanimelist';
import { Provider } from '@/types/List';
import { buildUrl } from '@/utils/url';
import { Box, Container, Grid, Typography } from '@mui/material';
import { useMemo } from 'react';
import AnimeTitle from '../AnimeInformations/components/AnimeTitle';
import MainInformation from '../AnimeInformations/components/MainInformation';
import AnimeCover from '../ui/AnimeCover';
import { findExactAnimeMatch, findSuggestedAnimeMatches } from './utils';

const NowPlaying = () => {
  const animePlaying = usePlayerDetectionStore((state) => state.activeEpisode);
  const animeListData = useMyAnimeListStore((state) => state.animeListData);
  const aliasesByAnimeId = useNowPlayingAliasesStore(
    (state) => state.aliasesByAnimeId
  );
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
    if (!animePlaying?.animeTitle) {
      return undefined;
    }

    return findExactAnimeMatch(
      aggregatedData,
      animePlaying.animeTitle,
      aliasesByAnimeId
    );
  }, [aggregatedData, animePlaying?.animeTitle, aliasesByAnimeId]);

  const suggestedMatches = useMemo(() => {
    if (!animePlaying?.animeTitle || exactAnimeMatch) {
      return [];
    }

    return findSuggestedAnimeMatches(
      aggregatedData,
      animePlaying.animeTitle,
      aliasesByAnimeId
    );
  }, [
    aggregatedData,
    animePlaying?.animeTitle,
    aliasesByAnimeId,
    exactAnimeMatch
  ]);

  const handleSuggestionClick = (animeId: number) => {
    if (!animePlaying?.animeTitle) {
      return;
    }

    addAlias(animeId, animePlaying.animeTitle);
  };

  return (
    <Container>
      <Typography variant="h6" component="h1" color="primary" gutterBottom>
        Now Playing
      </Typography>

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
    </Container>
  );
};

export default NowPlaying;
