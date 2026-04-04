use serde::{Deserialize, Serialize};

use crate::services::anime_list_updates::ListType;

mod api;
mod mapping;

pub use api::{fetch_anilist_user_info, synchronize_anilist, update_anilist_list_entry};

const GRAPHQL_URL: &str = "https://graphql.anilist.co";
const REQUEST_TIMEOUT_SECS: u64 = 15;
const MEDIA_TYPE_ANIME: &str = "ANIME";
const MEDIA_TYPE_MANGA: &str = "MANGA";
const MEDIA_LIST_COLLECTION_QUERY: &str = r#"
query ($type: MediaType!, $userName: String) {
  MediaListCollection(type: $type, userName: $userName) {
    lists {
      name
      entries {
        id
        media {
          id
          title {
            romaji
            native
            english
          }
          coverImage {
            large
            extraLarge
          }
          endDate {
            day
            month
            year
          }
          meanScore
          mediaListEntry {
            completedAt {
              day
              month
              year
            }
            notes
            progress
            progressVolumes
            repeat
            startedAt {
              day
              month
              year
            }
            status
            score
            id
          }
          startDate {
            year
            month
            day
          }
          source
          seasonYear
          season
          episodes
          chapters
          volumes
          description
          nextAiringEpisode {
            episode
          }
          status
          studios {
            nodes {
              name
            }
          }
          staff {
            edges {
              role
              node {
                name {
                  full
                }
              }
            }
          }
          type
          genres
          format
        }
        completedAt {
          day
          month
          year
        }
        createdAt
        customLists
      }
      status
    }
    hasNextChunk
  }
}
"#;
const VIEWER_QUERY: &str = r#"
query Viewer {
  Viewer {
    avatar {
      large
    }
    id
    name
    statistics {
      anime {
        count
        episodesWatched
        meanScore
        minutesWatched
        statuses {
          count
          meanScore
          minutesWatched
          status
        }
      }
    }
  }
}
"#;
const SAVE_MEDIA_LIST_ENTRY_MUTATION: &str = r#"
mutation Mutation(
  $saveMediaListEntryId: Int
  $mediaId: Int
  $status: MediaListStatus
  $score: Float
  $progress: Int
  $progressVolumes: Int
  $repeat: Int
  $notes: String
  $startedAt: FuzzyDateInput
  $completedAt: FuzzyDateInput
) {
  SaveMediaListEntry(
    id: $saveMediaListEntryId
    mediaId: $mediaId
    status: $status
    score: $score
    progress: $progress
    progressVolumes: $progressVolumes
    repeat: $repeat
    notes: $notes
    startedAt: $startedAt
    completedAt: $completedAt
  ) {
    id
  }
}
"#;

#[derive(Serialize)]
struct GraphQlRequest<'a> {
    query: &'a str,
    variables: GraphQlVariables<'a>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GraphQlVariables<'a> {
    r#type: &'a str,
    user_name: Option<&'a str>,
}

#[derive(Serialize)]
struct ViewerRequest<'a> {
    query: &'a str,
}

#[derive(Serialize)]
struct SaveMediaListEntryRequest<'a> {
    query: &'a str,
    variables: SaveMediaListEntryVariables,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SaveMediaListEntryVariables {
    #[serde(skip_serializing_if = "Option::is_none")]
    save_media_list_entry_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    media_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    score: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    progress: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    progress_volumes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repeat: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    started_at: Option<FuzzyDateInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    completed_at: Option<FuzzyDateInput>,
}

#[derive(Serialize)]
struct FuzzyDateInput {
    year: i32,
    month: i32,
    day: i32,
}

#[derive(Deserialize)]
struct GraphQlResponse {
    data: Option<GraphQlData>,
    errors: Option<Vec<GraphQlError>>,
}

#[derive(Deserialize)]
struct GraphQlData {
    #[serde(rename = "MediaListCollection")]
    media_list_collection: Option<AniListCollection>,
}

#[derive(Deserialize)]
struct ViewerResponse {
    data: Option<ViewerData>,
    errors: Option<Vec<GraphQlError>>,
}

#[derive(Deserialize)]
struct ViewerData {
    #[serde(rename = "Viewer")]
    viewer: Option<AniListViewer>,
}

#[derive(Deserialize)]
struct SaveMediaListEntryMutationResponse {
    data: Option<SaveMediaListEntryMutationData>,
    errors: Option<Vec<GraphQlError>>,
}

#[derive(Deserialize)]
struct SaveMediaListEntryMutationData {
    #[serde(rename = "SaveMediaListEntry")]
    save_media_list_entry: Option<SaveMediaListEntryMutationPayload>,
}

#[derive(Deserialize)]
struct SaveMediaListEntryMutationPayload {
    id: u64,
}

#[derive(Deserialize)]
struct GraphQlError {
    message: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AniListViewer {
    id: u64,
    name: String,
    avatar: Option<AniListViewerAvatar>,
    statistics: Option<AniListViewerStatistics>,
}

#[derive(Deserialize)]
struct AniListViewerAvatar {
    large: Option<String>,
}

#[derive(Deserialize)]
struct AniListViewerStatistics {
    anime: Option<AniListAnimeStatistics>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct AniListAnimeStatistics {
    count: Option<u32>,
    episodes_watched: Option<u32>,
    mean_score: Option<f64>,
    minutes_watched: Option<u64>,
    #[serde(default)]
    statuses: Vec<AniListAnimeStatisticsStatus>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct AniListAnimeStatisticsStatus {
    count: Option<u32>,
    minutes_watched: Option<u64>,
    status: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserStatistics {
    num_items_watching: u32,
    num_items_completed: u32,
    num_items_on_hold: u32,
    num_items_dropped: u32,
    num_items_plan_to_watch: u32,
    num_items: u32,
    num_days_watched: f64,
    num_days_watching: f64,
    num_days_completed: f64,
    num_days_on_hold: f64,
    num_days_dropped: f64,
    num_days: f64,
    num_episodes: u32,
    num_times_rewatched: u32,
    mean_score: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AniListUserInfo {
    pub id: u64,
    pub name: String,
    pub picture: Option<String>,
    pub statistics: Option<UserStatistics>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct AniListCollection {
    #[serde(default)]
    lists: Vec<AniListList>,
    has_next_chunk: bool,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct AniListList {
    status: Option<String>,
    #[serde(default)]
    entries: Vec<AniListEntry>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct AniListEntry {
    media: Option<AniListMedia>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct AniListMedia {
    id: u64,
    title: Option<AniListTitle>,
    cover_image: Option<AniListCoverImage>,
    mean_score: Option<u32>,
    media_list_entry: Option<AniListMediaListEntry>,
    start_date: Option<AniListFuzzyDate>,
    end_date: Option<AniListFuzzyDate>,
    source: Option<String>,
    season_year: Option<u32>,
    season: Option<String>,
    episodes: Option<u32>,
    chapters: Option<u32>,
    volumes: Option<u32>,
    description: Option<String>,
    next_airing_episode: Option<AniListNextAiringEpisode>,
    status: Option<String>,
    studios: Option<AniListStudios>,
    staff: Option<AniListStaff>,
    #[serde(default)]
    genres: Vec<String>,
    format: Option<String>,
    r#type: Option<String>,
}

#[derive(Deserialize, Default)]
struct AniListCoverImage {
    large: Option<String>,
    #[serde(rename = "extraLarge")]
    extra_large: Option<String>,
}

#[derive(Deserialize, Default)]
struct AniListTitle {
    romaji: Option<String>,
    #[serde(rename = "native")]
    native_title: Option<String>,
    english: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AniListMediaListEntry {
    id: u64,
    completed_at: Option<AniListFuzzyDate>,
    notes: Option<String>,
    progress: Option<u32>,
    progress_volumes: Option<u32>,
    repeat: Option<u32>,
    started_at: Option<AniListFuzzyDate>,
    status: Option<String>,
    score: Option<f64>,
}

#[derive(Deserialize, Default)]
struct AniListStudios {
    #[serde(default)]
    nodes: Vec<AniListStudio>,
}

#[derive(Deserialize, Default)]
struct AniListStudio {
    name: Option<String>,
}

#[derive(Deserialize, Default)]
struct AniListStaff {
    #[serde(default)]
    edges: Vec<AniListStaffEdge>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct AniListStaffEdge {
    role: Option<String>,
    node: Option<AniListStaffNode>,
}

#[derive(Deserialize, Default)]
struct AniListStaffNode {
    name: Option<AniListStaffName>,
}

#[derive(Deserialize, Default)]
struct AniListStaffName {
    full: Option<String>,
}

#[derive(Deserialize, Default)]
struct AniListFuzzyDate {
    day: Option<u32>,
    month: Option<u32>,
    year: Option<i32>,
}

#[derive(Deserialize, Default)]
struct AniListNextAiringEpisode {
    episode: u32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AnimeListBroadcast {
    day_of_the_week: String,
    start_time: String,
    available_episodes: Option<u32>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AnimeListItem {
    id: u64,
    entry_id: u64,
    title: String,
    image_url: String,
    synopsis: String,
    alternative_titles: String,
    score: f64,
    source: String,
    status: String,
    total_episodes: u32,
    genres: String,
    start_season: String,
    start_date: String,
    broadcast: AnimeListBroadcast,
    studios: String,
    media_type: String,
    user_status: String,
    user_score: u32,
    user_episodes_watched: u32,
    is_rewatching: bool,
    user_comments: String,
    user_num_times_rewatched: u32,
    user_start_date: Option<String>,
    user_finish_date: Option<String>,
    updated_at: Option<String>,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SynchronizedAnimeList {
    watching: Vec<AnimeListItem>,
    completed: Vec<AnimeListItem>,
    on_hold: Vec<AnimeListItem>,
    dropped: Vec<AnimeListItem>,
    plan_to_watch: Vec<AnimeListItem>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MangaListItem {
    id: u64,
    entry_id: u64,
    title: String,
    image_url: String,
    synopsis: String,
    alternative_titles: String,
    score: f64,
    status: String,
    total_volumes: u32,
    total_chapters: u32,
    genres: String,
    start_date: Option<String>,
    end_date: Option<String>,
    authors: String,
    serialization: String,
    media_type: String,
    user_status: String,
    user_score: u32,
    user_volumes_read: u32,
    user_chapters_read: u32,
    is_rereading: bool,
    user_comments: String,
    user_num_times_reread: u32,
    user_start_date: Option<String>,
    user_finish_date: Option<String>,
    updated_at: Option<String>,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SynchronizedMangaList {
    reading: Vec<MangaListItem>,
    completed: Vec<MangaListItem>,
    on_hold: Vec<MangaListItem>,
    dropped: Vec<MangaListItem>,
    plan_to_read: Vec<MangaListItem>,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum SynchronizedListResult {
    Anime(SynchronizedAnimeList),
    Manga(SynchronizedMangaList),
}

#[derive(Copy, Clone)]
enum UserStatusKey {
    Reading,
    Watching,
    Completed,
    OnHold,
    Dropped,
    PlanToWatch,
    PlanToRead,
}

impl UserStatusKey {
    fn from_anilist(list_type: ListType, status: Option<&str>) -> Self {
        match list_type {
            ListType::Anime => match status {
                Some("CURRENT") | Some("REPEATING") => Self::Watching,
                Some("COMPLETED") => Self::Completed,
                Some("PAUSED") => Self::OnHold,
                Some("DROPPED") => Self::Dropped,
                Some("PLANNING") => Self::PlanToWatch,
                _ => Self::PlanToWatch,
            },
            ListType::Manga => match status {
                Some("CURRENT") | Some("REPEATING") => Self::Reading,
                Some("COMPLETED") => Self::Completed,
                Some("PAUSED") => Self::OnHold,
                Some("DROPPED") => Self::Dropped,
                Some("PLANNING") => Self::PlanToRead,
                _ => Self::PlanToRead,
            },
        }
    }

    fn as_user_status_str(self) -> &'static str {
        match self {
            Self::Reading => "reading",
            Self::Watching => "watching",
            Self::Completed => "completed",
            Self::OnHold => "onHold",
            Self::Dropped => "dropped",
            Self::PlanToWatch => "planToWatch",
            Self::PlanToRead => "planToRead",
        }
    }

    fn push_anime(self, result: &mut SynchronizedAnimeList, item: AnimeListItem) {
        match self {
            Self::Watching => result.watching.push(item),
            Self::Completed => result.completed.push(item),
            Self::OnHold => result.on_hold.push(item),
            Self::Dropped => result.dropped.push(item),
            Self::PlanToWatch => result.plan_to_watch.push(item),
            Self::Reading => result.watching.push(item),
            Self::PlanToRead => result.plan_to_watch.push(item),
        }
    }

    fn push_manga(self, result: &mut SynchronizedMangaList, item: MangaListItem) {
        match self {
            Self::Reading => result.reading.push(item),
            Self::Completed => result.completed.push(item),
            Self::OnHold => result.on_hold.push(item),
            Self::Dropped => result.dropped.push(item),
            Self::PlanToRead => result.plan_to_read.push(item),
            Self::Watching => result.reading.push(item),
            Self::PlanToWatch => result.plan_to_read.push(item),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn anime_item(id: u64) -> AnimeListItem {
        AnimeListItem {
            id,
            entry_id: id,
            title: format!("Anime {id}"),
            image_url: String::new(),
            synopsis: String::new(),
            alternative_titles: String::new(),
            score: 0.0,
            source: String::new(),
            status: String::new(),
            total_episodes: 0,
            genres: String::new(),
            start_season: String::new(),
            start_date: String::new(),
            broadcast: AnimeListBroadcast {
                day_of_the_week: String::new(),
                start_time: String::new(),
                available_episodes: None,
            },
            studios: String::new(),
            media_type: String::new(),
            user_status: String::new(),
            user_score: 0,
            user_episodes_watched: 0,
            is_rewatching: false,
            user_comments: String::new(),
            user_num_times_rewatched: 0,
            user_start_date: None,
            user_finish_date: None,
            updated_at: None,
        }
    }

    fn manga_item(id: u64) -> MangaListItem {
        MangaListItem {
            id,
            entry_id: id,
            title: format!("Manga {id}"),
            image_url: String::new(),
            synopsis: String::new(),
            alternative_titles: String::new(),
            score: 0.0,
            status: String::new(),
            total_volumes: 0,
            total_chapters: 0,
            genres: String::new(),
            start_date: None,
            end_date: None,
            authors: String::new(),
            serialization: String::new(),
            media_type: String::new(),
            user_status: String::new(),
            user_score: 0,
            user_volumes_read: 0,
            user_chapters_read: 0,
            is_rereading: false,
            user_comments: String::new(),
            user_num_times_reread: 0,
            user_start_date: None,
            user_finish_date: None,
            updated_at: None,
        }
    }

    #[test]
    fn user_status_key_from_anilist_maps_statuses_per_list_type() {
        assert!(matches!(
            UserStatusKey::from_anilist(ListType::Anime, Some("CURRENT")),
            UserStatusKey::Watching
        ));
        assert!(matches!(
            UserStatusKey::from_anilist(ListType::Anime, Some("PAUSED")),
            UserStatusKey::OnHold
        ));
        assert!(matches!(
            UserStatusKey::from_anilist(ListType::Manga, Some("CURRENT")),
            UserStatusKey::Reading
        ));
        assert!(matches!(
            UserStatusKey::from_anilist(ListType::Manga, Some("unknown")),
            UserStatusKey::PlanToRead
        ));
    }

    #[test]
    fn user_status_key_exposes_expected_status_strings() {
        assert_eq!(UserStatusKey::Reading.as_user_status_str(), "reading");
        assert_eq!(UserStatusKey::Watching.as_user_status_str(), "watching");
        assert_eq!(
            UserStatusKey::PlanToWatch.as_user_status_str(),
            "planToWatch"
        );
        assert_eq!(UserStatusKey::PlanToRead.as_user_status_str(), "planToRead");
    }

    #[test]
    fn user_status_key_pushes_items_into_expected_anime_buckets() {
        let mut result = SynchronizedAnimeList::default();
        UserStatusKey::Watching.push_anime(&mut result, anime_item(1));
        UserStatusKey::Completed.push_anime(&mut result, anime_item(2));
        UserStatusKey::OnHold.push_anime(&mut result, anime_item(3));
        UserStatusKey::Dropped.push_anime(&mut result, anime_item(4));
        UserStatusKey::PlanToWatch.push_anime(&mut result, anime_item(5));
        UserStatusKey::Reading.push_anime(&mut result, anime_item(6));
        UserStatusKey::PlanToRead.push_anime(&mut result, anime_item(7));

        assert_eq!(result.watching.len(), 2);
        assert_eq!(result.completed.len(), 1);
        assert_eq!(result.on_hold.len(), 1);
        assert_eq!(result.dropped.len(), 1);
        assert_eq!(result.plan_to_watch.len(), 2);
    }

    #[test]
    fn user_status_key_pushes_items_into_expected_manga_buckets() {
        let mut result = SynchronizedMangaList::default();
        UserStatusKey::Reading.push_manga(&mut result, manga_item(1));
        UserStatusKey::Completed.push_manga(&mut result, manga_item(2));
        UserStatusKey::OnHold.push_manga(&mut result, manga_item(3));
        UserStatusKey::Dropped.push_manga(&mut result, manga_item(4));
        UserStatusKey::PlanToRead.push_manga(&mut result, manga_item(5));
        UserStatusKey::Watching.push_manga(&mut result, manga_item(6));
        UserStatusKey::PlanToWatch.push_manga(&mut result, manga_item(7));

        assert_eq!(result.reading.len(), 2);
        assert_eq!(result.completed.len(), 1);
        assert_eq!(result.on_hold.len(), 1);
        assert_eq!(result.dropped.len(), 1);
        assert_eq!(result.plan_to_read.len(), 2);
    }
}
