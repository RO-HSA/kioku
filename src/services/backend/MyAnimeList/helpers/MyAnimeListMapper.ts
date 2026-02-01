import { IAnimeList } from '../../types';
import {
  MyAnimeListListEntry,
  MyAnimeListMediaTypeEnum,
  MyAnimeListSourceEnum,
  MyAnimeListStatusEnum,
  MyAnimeListUserStatusEnum
} from '../types';

class MyAnimeListMapper {
  static mapListEntryToDomain(data: MyAnimeListListEntry): IAnimeList {
    const { node, list_status } = data || {};

    const englishAlternativeTitle = node.alternative_titles?.en
      ? `${node.alternative_titles.en}, `
      : '';
    const japaneseAlternativeTitle = node.alternative_titles?.ja
      ? `${node.alternative_titles.ja}, `
      : '';
    const synonymsAlternativeTitles =
      node.alternative_titles?.synonyms?.join(', ') || '';
    const startSeason = node.start_season
      ? node.start_season.season + ' '
      : 'Unknown';
    const startYear = node.start_season ? node.start_season.year : 'Unknown';

    return {
      id: node.id,
      title: node.title,
      imageUrl: node.main_picture?.large || node.main_picture?.medium || '',
      synopsis: node.synopsis || 'No synopsis available.',
      alternativeTitles:
        englishAlternativeTitle +
          japaneseAlternativeTitle +
          synonymsAlternativeTitles || 'Unknown',
      score: node.mean || 0.0,
      source: node.source
        ? MyAnimeListSourceEnum[node.source]
        : node.source || 'Unknown',
      status: node.status
        ? MyAnimeListStatusEnum[node.status]
        : node.status || 'Unknown',
      totalEpisodes: node.num_episodes || 0,
      genres: node.genres.map((genre) => genre.name).join(', ') || 'Unknown',
      startSeason:
        !node.start_season?.season && !node.start_season?.year
          ? 'Unknown'
          : startSeason + startYear,
      studios:
        node.studios?.map((studio) => studio.name).join(', ') || 'Unknown',
      mediaType: node.media_type
        ? MyAnimeListMediaTypeEnum[node.media_type]
        : 'Unknown',
      userStatus: list_status.status
        ? MyAnimeListUserStatusEnum[list_status.status]
        : 'planToWatch',
      userScore: list_status.score || 0,
      userEpisodesWatched: list_status.num_episodes_watched || 0,
      isRewatching: list_status.is_rewatching || false,
      userStartDate: list_status.start_date,
      userFinishDate: list_status.finish_date,
      updatedAt: list_status.updated_at
    };
  }
}

export default MyAnimeListMapper;
