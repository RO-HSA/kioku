import { IAnimeList } from '@/types/AnimeList';

const SPLIT_ALTERNATIVE_TITLES_REGEX = /(?:\s\/\s|[,\n;|])+/;
const COMBINING_MARKS_REGEX = /[\u0300-\u036f]/g;
const NON_ALPHANUMERIC_REGEX = /[^a-z0-9\s]/g;
const WHITESPACE_REGEX = /\s+/g;

export const normalizeTitle = (value: string): string => {
  return value
    .normalize('NFD')
    .replace(COMBINING_MARKS_REGEX, '')
    .toLowerCase()
    .replace(NON_ALPHANUMERIC_REGEX, ' ')
    .replace(WHITESPACE_REGEX, ' ')
    .trim();
};

const splitAlternativeTitles = (value: string): string[] => {
  return value
    .split(SPLIT_ALTERNATIVE_TITLES_REGEX)
    .map((title) => title.trim())
    .filter(Boolean);
};

const tokenize = (value: string): string[] => {
  return value
    .split(' ')
    .map((token) => token.trim())
    .filter(Boolean);
};

const scoreTitleMatch = (query: string, candidate: string): number => {
  if (!query || !candidate) {
    return 0;
  }

  if (query === candidate) {
    return 100;
  }

  let score = 0;
  if (candidate.includes(query) || query.includes(candidate)) {
    const ratio =
      Math.min(candidate.length, query.length) /
      Math.max(candidate.length, query.length);
    score = Math.max(score, 70 + Math.round(ratio * 20));
  }

  const queryTokens = tokenize(query);
  const candidateTokens = tokenize(candidate);
  if (!queryTokens.length || !candidateTokens.length) {
    return score;
  }

  const candidateTokenSet = new Set(candidateTokens);
  const intersectionCount = queryTokens.filter((token) =>
    candidateTokenSet.has(token)
  ).length;
  const tokenScore = Math.round((intersectionCount / queryTokens.length) * 65);

  score = Math.max(score, tokenScore);

  if (queryTokens[0] === candidateTokens[0]) {
    score += 8;
  }

  return Math.min(score, 100);
};

const buildCandidateTitles = (
  anime: IAnimeList,
  localAliases: string[]
): string[] => {
  return [
    anime.title,
    ...splitAlternativeTitles(anime.alternativeTitles),
    ...localAliases
  ]
    .map((title) => title.trim())
    .filter(Boolean);
};

export const findExactAnimeMatch = (
  animeList: IAnimeList[],
  detectedTitle: string,
  aliasesByAnimeId: Record<string, string[]>
): IAnimeList | undefined => {
  const normalizedDetectedTitle = normalizeTitle(detectedTitle);
  if (!normalizedDetectedTitle) {
    return undefined;
  }

  return animeList.find((anime) => {
    const localAliases = aliasesByAnimeId[anime.id.toString()] || [];
    const candidateTitles = buildCandidateTitles(anime, localAliases);

    return candidateTitles.some(
      (candidateTitle) =>
        normalizeTitle(candidateTitle) === normalizedDetectedTitle
    );
  });
};

export const findSuggestedAnimeMatches = (
  animeList: IAnimeList[],
  detectedTitle: string,
  aliasesByAnimeId: Record<string, string[]>,
  options?: { maxSuggestions?: number; minimumScore?: number }
): IAnimeList[] => {
  const normalizedDetectedTitle = normalizeTitle(detectedTitle);
  if (!normalizedDetectedTitle) {
    return [];
  }

  const maxSuggestions = options?.maxSuggestions || 5;
  const minimumScore = options?.minimumScore || 35;

  return animeList
    .map((anime) => {
      const localAliases = aliasesByAnimeId[anime.id.toString()] || [];
      const candidateTitles = buildCandidateTitles(anime, localAliases);

      const score = candidateTitles.reduce((bestScore, candidateTitle) => {
        const normalizedCandidateTitle = normalizeTitle(candidateTitle);
        const currentScore = scoreTitleMatch(
          normalizedDetectedTitle,
          normalizedCandidateTitle
        );

        return Math.max(bestScore, currentScore);
      }, 0);

      return { anime, score };
    })
    .filter(({ score }) => score >= minimumScore)
    .sort(
      (a, b) => b.score - a.score || a.anime.title.localeCompare(b.anime.title)
    )
    .slice(0, maxSuggestions)
    .map(({ anime }) => anime);
};
