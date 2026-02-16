use serde::{Deserialize, Serialize};
use serde_json;
use std::time::Duration;
use tauri_plugin_http::reqwest;
use tauri_plugin_zustand::ManagerExt;

use crate::auth::anilist::PROVIDER_ID as ANILIST_PROVIDER_ID;
use crate::auth::token_manager::get_access_token;
use crate::services::anime_list_updates::AnimeListUpdateRequest;

const GRAPHQL_URL: &str = "https://graphql.anilist.co";
const REQUEST_TIMEOUT_SECS: u64 = 15;
const MEDIA_TYPE_ANIME: &str = "ANIME";
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
const SAVE_MEDIA_LIST_ENTRY_MUTATION: &str = r#"
mutation Mutation(
  $saveMediaListEntryId: Int
  $mediaId: Int
  $status: MediaListStatus
  $score: Float
  $progress: Int
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
struct SaveMediaListEntryRequest<'a> {
    query: &'a str,
    variables: SaveMediaListEntryVariables,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SaveMediaListEntryVariables {
    save_media_list_entry_id: Option<u64>,
    media_id: Option<u64>,
    status: Option<String>,
    score: Option<f64>,
    progress: Option<u32>,
    repeat: Option<u32>,
    notes: Option<String>,
    started_at: Option<FuzzyDateInput>,
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
    source: Option<String>,
    season_year: Option<u32>,
    season: Option<String>,
    episodes: Option<u32>,
    description: Option<String>,
    next_airing_episode: Option<AniListNextAiringEpisode>,
    status: Option<String>,
    studios: Option<AniListStudios>,
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

#[derive(Copy, Clone)]
enum UserStatusKey {
    Watching,
    Completed,
    OnHold,
    Dropped,
    PlanToWatch,
}

impl UserStatusKey {
    fn from_anilist(status: Option<&str>) -> Self {
        match status {
            Some("CURRENT") | Some("REPEATING") => Self::Watching,
            Some("COMPLETED") => Self::Completed,
            Some("PAUSED") => Self::OnHold,
            Some("DROPPED") => Self::Dropped,
            Some("PLANNING") => Self::PlanToWatch,
            _ => Self::PlanToWatch,
        }
    }

    fn as_user_status_str(self) -> &'static str {
        match self {
            Self::Watching => "watching",
            Self::Completed => "completed",
            Self::OnHold => "onHold",
            Self::Dropped => "dropped",
            Self::PlanToWatch => "planToWatch",
        }
    }

    fn push(self, result: &mut SynchronizedAnimeList, item: AnimeListItem) {
        match self {
            Self::Watching => result.watching.push(item),
            Self::Completed => result.completed.push(item),
            Self::OnHold => result.on_hold.push(item),
            Self::Dropped => result.dropped.push(item),
            Self::PlanToWatch => result.plan_to_watch.push(item),
        }
    }
}

fn normalize_text(value: Option<&str>) -> Option<String> {
    let value = value?.trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

fn format_upper_snake(value: &str) -> String {
    let mut result = String::new();
    for (index, part) in value.split('_').filter(|part| !part.is_empty()).enumerate() {
        if index > 0 {
            result.push(' ');
        }

        let lowercase = part.to_ascii_lowercase();
        let mut chars = lowercase.chars();
        if let Some(first) = chars.next() {
            result.push(first.to_ascii_uppercase());
            result.push_str(chars.as_str());
        }
    }

    if result.is_empty() {
        "Unknown".to_string()
    } else {
        result
    }
}

fn pick_title(title: Option<&AniListTitle>) -> String {
    let Some(title) = title else {
        return "Unknown".to_string();
    };

    normalize_text(title.english.as_deref())
        .or_else(|| normalize_text(title.romaji.as_deref()))
        .or_else(|| normalize_text(title.native_title.as_deref()))
        .unwrap_or_else(|| "Unknown".to_string())
}

fn build_alternative_titles(title: Option<&AniListTitle>, primary: &str) -> String {
    let Some(title) = title else {
        return "Unknown".to_string();
    };

    let mut parts: Vec<String> = Vec::new();

    for value in [
        title.english.as_deref(),
        title.romaji.as_deref(),
        title.native_title.as_deref(),
    ] {
        let Some(candidate) = normalize_text(value) else {
            continue;
        };

        if candidate == primary || parts.iter().any(|existing| existing == &candidate) {
            continue;
        }

        parts.push(candidate);
    }

    if parts.is_empty() {
        "Unknown".to_string()
    } else {
        parts.join(", ")
    }
}

fn map_source(source: Option<String>) -> String {
    match source {
        Some(value) => match value.as_str() {
            "ORIGINAL" => "Original".to_string(),
            "MANGA" => "Manga".to_string(),
            "LIGHT_NOVEL" => "Light Novel".to_string(),
            "VISUAL_NOVEL" => "Visual Novel".to_string(),
            "VIDEO_GAME" => "Video Game".to_string(),
            "GAME" => "Game".to_string(),
            "OTHER" => "Other".to_string(),
            _ => format_upper_snake(&value),
        },
        None => "Unknown".to_string(),
    }
}

fn map_status(status: Option<String>) -> String {
    match status {
        Some(value) => match value.as_str() {
            "FINISHED" => "Finished Airing".to_string(),
            "NOT_YET_RELEASED" => "Not Yet Aired".to_string(),
            "RELEASING" => "Currently Airing".to_string(),
            _ => format_upper_snake(&value),
        },
        None => "Unknown".to_string(),
    }
}

fn map_media_type(media_type: Option<String>) -> String {
    match media_type {
        Some(value) => match value.as_str() {
            "TV" => "TV".to_string(),
            "TV_SHORT" => "TV Short".to_string(),
            "MOVIE" => "Movie".to_string(),
            "SPECIAL" => "Special".to_string(),
            "OVA" => "OVA".to_string(),
            "ONA" => "ONA".to_string(),
            "MUSIC" => "Music".to_string(),
            "MANGA" => "Manga".to_string(),
            "NOVEL" => "Novel".to_string(),
            "ONE_SHOT" => "One Shot".to_string(),
            "ANIME" => "Anime".to_string(),
            _ => format_upper_snake(&value),
        },
        None => "Unknown".to_string(),
    }
}

fn map_user_status_to_anilist(status: &str) -> Result<&'static str, String> {
    match status {
        "watching" => Ok("CURRENT"),
        "completed" => Ok("COMPLETED"),
        "onHold" | "on_hold" => Ok("PAUSED"),
        "dropped" => Ok("DROPPED"),
        "planToWatch" | "plan_to_watch" => Ok("PLANNING"),
        _ => Err(format!("Invalid AniList status: {status}")),
    }
}

fn join_genres(genres: Vec<String>) -> String {
    let mut values: Vec<String> = Vec::new();

    for genre in genres {
        let Some(name) = normalize_text(Some(genre.as_str())) else {
            continue;
        };

        if !values.iter().any(|existing| existing == &name) {
            values.push(name);
        }
    }

    if values.is_empty() {
        "Unknown".to_string()
    } else {
        values.join(", ")
    }
}

fn format_fuzzy_date(date: Option<AniListFuzzyDate>) -> Option<String> {
    let date = date?;
    let year = date.year?;
    let month = date.month?;
    let day = date.day?;

    if month == 0 || day == 0 {
        return None;
    }

    Some(format!("{year:04}-{month:02}-{day:02}"))
}

fn parse_fuzzy_date_input(
    value: Option<&str>,
    field_name: &str,
) -> Result<Option<FuzzyDateInput>, String> {
    let Some(value) = value else {
        return Ok(None);
    };

    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }

    let mut segments = trimmed.split('-');
    let year = segments
        .next()
        .ok_or_else(|| format!("Invalid {field_name}: expected YYYY-MM-DD"))?
        .parse::<i32>()
        .map_err(|_| format!("Invalid {field_name}: expected YYYY-MM-DD"))?;
    let month = segments
        .next()
        .ok_or_else(|| format!("Invalid {field_name}: expected YYYY-MM-DD"))?
        .parse::<i32>()
        .map_err(|_| format!("Invalid {field_name}: expected YYYY-MM-DD"))?;
    let day = segments
        .next()
        .ok_or_else(|| format!("Invalid {field_name}: expected YYYY-MM-DD"))?
        .parse::<i32>()
        .map_err(|_| format!("Invalid {field_name}: expected YYYY-MM-DD"))?;

    if segments.next().is_some() {
        return Err(format!("Invalid {field_name}: expected YYYY-MM-DD"));
    }

    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return Err(format!("Invalid {field_name}: expected YYYY-MM-DD"));
    }

    Ok(Some(FuzzyDateInput { year, month, day }))
}

fn format_start_season(season: Option<String>, season_year: Option<u32>) -> String {
    let season_part = season.map(|value| format_upper_snake(&value));

    match (season_part, season_year) {
        (Some(season), Some(year)) if !season.is_empty() => format!("{season} {year}"),
        (Some(season), None) if !season.is_empty() => season,
        (None, Some(year)) => year.to_string(),
        _ => "Unknown".to_string(),
    }
}

fn join_studio_names(studios: Option<AniListStudios>) -> String {
    let Some(studios) = studios else {
        return "Unknown".to_string();
    };

    for studio in studios.nodes {
        let Some(name) = normalize_text(studio.name.as_deref()) else {
            continue;
        };
        return name;
    }

    "Unknown".to_string()
}

fn normalize_score(value: Option<f64>) -> u32 {
    let Some(value) = value else {
        return 0;
    };

    if !value.is_finite() {
        return 0;
    }

    if value <= 0.0 {
        return 0;
    }

    if value >= u32::MAX as f64 {
        return u32::MAX;
    }

    value.round() as u32
}

fn map_media_to_domain(
    media: AniListMedia,
    media_list_entry: AniListMediaListEntry,
    status_key: UserStatusKey,
) -> AnimeListItem {
    let title = pick_title(media.title.as_ref());
    let alternative_titles = build_alternative_titles(media.title.as_ref(), &title);
    let available_episodes = media
        .next_airing_episode
        .as_ref()
        .map(|next| next.episode.saturating_sub(1))
        .or(media.episodes);

    let image_url = media
        .cover_image
        .and_then(|cover| cover.extra_large.or(cover.large))
        .unwrap_or_default();

    AnimeListItem {
        id: media.id,
        entry_id: media_list_entry.id,
        title,
        image_url,
        synopsis: media
            .description
            .unwrap_or_else(|| "No synopsis available.".to_string()),
        alternative_titles,
        score: media.mean_score.unwrap_or(0) as f64,
        source: map_source(media.source),
        status: map_status(media.status),
        total_episodes: media.episodes.unwrap_or(0),
        genres: join_genres(media.genres),
        start_season: format_start_season(media.season, media.season_year),
        start_date: format_fuzzy_date(media.start_date).unwrap_or_default(),
        broadcast: AnimeListBroadcast {
            day_of_the_week: String::new(),
            start_time: String::new(),
            available_episodes,
        },
        studios: join_studio_names(media.studios),
        media_type: map_media_type(media.format.or(media.r#type)),
        user_status: status_key.as_user_status_str().to_string(),
        user_score: normalize_score(media_list_entry.score),
        user_episodes_watched: media_list_entry.progress.unwrap_or(0),
        is_rewatching: media_list_entry.repeat.unwrap_or(0) > 0,
        user_comments: media_list_entry.notes.unwrap_or_default(),
        user_num_times_rewatched: media_list_entry.repeat.unwrap_or(0),
        user_start_date: format_fuzzy_date(media_list_entry.started_at),
        user_finish_date: format_fuzzy_date(media_list_entry.completed_at),
        updated_at: None,
    }
}

async fn fetch_collection(
    client: &reqwest::Client,
    token: &str,
    username: Option<&str>,
) -> Result<AniListCollection, String> {
    let request = GraphQlRequest {
        query: MEDIA_LIST_COLLECTION_QUERY,
        variables: GraphQlVariables {
            r#type: MEDIA_TYPE_ANIME,
            user_name: username,
        },
    };

    let response = client
        .post(GRAPHQL_URL)
        .bearer_auth(token)
        .json(&request)
        .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let status = response.status();

    let body = response.text().await.map_err(|e| e.to_string())?;
    if !status.is_success() {
        return Err(format!("AniList request failed: {} - {}", status, body));
    }

    let parsed: GraphQlResponse = serde_json::from_str(&body)
        .map_err(|e| format!("Failed to parse AniList response: {e}"))?;

    if let Some(errors) = parsed.errors {
        if !errors.is_empty() {
            let message = errors
                .into_iter()
                .map(|error| error.message)
                .collect::<Vec<_>>()
                .join("; ");
            return Err(format!("AniList GraphQL error: {message}"));
        }
    }

    parsed
        .data
        .and_then(|data| data.media_list_collection)
        .ok_or_else(|| "AniList response missing MediaListCollection".to_string())
}

#[tauri::command]
pub async fn synchronize_anilist(app: tauri::AppHandle) -> Result<SynchronizedAnimeList, String> {
    let token = get_access_token(&app, ANILIST_PROVIDER_ID).await?;
    let username: Option<String> = app.zustand().get_or_default("anilist", "username");
    let username = username.and_then(|value| {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    });

    let client = reqwest::Client::new();
    let collection = fetch_collection(&client, &token, username.as_deref()).await?;
    let mut result = SynchronizedAnimeList::default();

    if collection.has_next_chunk {
        eprintln!("AniList returned additional chunks; only the first chunk is currently synchronized.");
    }

    for list in collection.lists {
        let list_status = list.status;

        for entry in list.entries {
            let Some(mut media) = entry.media else {
                continue;
            };

            let status_key = UserStatusKey::from_anilist(
                media
                    .media_list_entry
                    .as_ref()
                    .and_then(|item| item.status.as_deref())
                    .or(list_status.as_deref()),
            );

            let Some(media_list_entry) = media.media_list_entry.take() else {
                eprintln!(
                    "AniList entry missing mediaListEntry for media_id={}",
                    media.id
                );
                continue;
            };

            let item = map_media_to_domain(media, media_list_entry, status_key);
            status_key.push(&mut result, item);
        }
    }

    Ok(result)
}

pub async fn update_anilist_list_entry(
    app: &tauri::AppHandle,
    client: &reqwest::Client,
    update: &AnimeListUpdateRequest,
) -> Result<(), String> {
    let token = get_access_token(app, ANILIST_PROVIDER_ID).await?;

    let status = update
        .user_status
        .as_deref()
        .map(map_user_status_to_anilist)
        .transpose()?
        .map(|value| value.to_string());
    let score = update.user_score.map(|value| value as f64);
    let progress = update.user_episodes_watched;
    let repeat = match (update.user_num_times_rewatched, update.is_rewatching) {
        (Some(value), _) => Some(value),
        (None, Some(true)) => Some(1),
        (None, Some(false)) => Some(0),
        (None, None) => None,
    };
    let notes = update.user_comments.as_ref().and_then(|value| {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    });
    let started_at = parse_fuzzy_date_input(update.user_start_date.as_deref(), "userStartDate")?;
    let completed_at =
        parse_fuzzy_date_input(update.user_finish_date.as_deref(), "userFinishDate")?;

    if status.is_none()
        && score.is_none()
        && progress.is_none()
        && repeat.is_none()
        && notes.is_none()
        && started_at.is_none()
        && completed_at.is_none()
    {
        return Err("No update fields provided".to_string());
    }

    let save_media_list_entry_id = if update.media_id.is_some() {
        None
    } else {
        update.entry_id
    };
    let media_id = update.media_id;

    if save_media_list_entry_id.is_none() && media_id.is_none() {
        return Err("Missing AniList target id: provide entryId or mediaId".to_string());
    }

    let payload = SaveMediaListEntryRequest {
        query: SAVE_MEDIA_LIST_ENTRY_MUTATION,
        variables: SaveMediaListEntryVariables {
            save_media_list_entry_id,
            media_id,
            status,
            score,
            progress,
            repeat,
            notes,
            started_at,
            completed_at,
        },
    };

    let response = client
        .post(GRAPHQL_URL)
        .bearer_auth(token)
        .json(&payload)
        .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let status_code = response.status();
    let body = response.text().await.map_err(|e| e.to_string())?;

    if !status_code.is_success() {
        return Err(format!("AniList update failed: {} - {}", status_code, body));
    }

    let parsed: SaveMediaListEntryMutationResponse = serde_json::from_str(&body)
        .map_err(|e| format!("Failed to parse AniList update response: {e}"))?;

    if let Some(errors) = parsed.errors {
        if !errors.is_empty() {
            let message = errors
                .into_iter()
                .map(|error| error.message)
                .collect::<Vec<_>>()
                .join("; ");
            return Err(format!("AniList GraphQL error: {message}"));
        }
    }

    let Some(data) = parsed.data else {
        return Err("AniList update response missing data".to_string());
    };

    let Some(saved_entry) = data.save_media_list_entry else {
        return Err("AniList update response missing SaveMediaListEntry".to_string());
    };

    if saved_entry.id == 0 {
        return Err("AniList update returned an invalid entry id".to_string());
    }

    Ok(())
}
