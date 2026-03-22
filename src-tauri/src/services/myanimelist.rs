use serde::{Deserialize, Serialize};
use tauri_plugin_http::reqwest;
use tauri_plugin_zustand::ManagerExt;

use crate::auth::mal::PROVIDER_ID as MAL_PROVIDER_ID;
use crate::auth::token_manager::get_access_token;
use crate::services::anime_list_updates::{AnimeListUpdateRequest, ListType};

const BASE_URL: &str = "https://api.myanimelist.net/v2/users/";
const ANIME_UPDATE_BASE_URL: &str = "https://api.myanimelist.net/v2/anime/";
const MANGA_UPDATE_BASE_URL: &str = "https://api.myanimelist.net/v2/manga/";
const USER_INFO_FIELDS: &str = "anime_statistics";
const ANIME_FIELDS: &str = "list_status{comments,num_times_rewatched},synopsis,alternative_titles,source,num_episodes,nsfw,start_season,media_type,studios,mean,status,genres,broadcast,start_date";
const MANGA_FIELDS: &str = "list_status{comments,num_times_reread},synopsis,alternative_titles,mean,media_type,status,genres,num_volumes,num_chapters,authors{first_name,last_name},serialization{name},start_date,end_date";
const LIMIT: u32 = 1000;

#[derive(Copy, Clone, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MyAnimeListListType {
    #[default]
    Anime,
    Manga,
}

#[derive(Deserialize)]
struct MalListResponse {
    data: Vec<MalListEntry>,
    paging: Option<MalPaging>,
}

#[derive(Deserialize)]
struct MalPaging {
    next: Option<String>,
}

#[derive(Deserialize)]
struct MalListEntry {
    node: MalNode,
    #[serde(default)]
    list_status: MalListStatus,
}

#[derive(Deserialize)]
struct MalNode {
    id: u64,
    title: String,
    main_picture: Option<MalPicture>,
    synopsis: Option<String>,
    alternative_titles: Option<MalAlternativeTitles>,
    mean: Option<f64>,
    source: Option<String>,
    num_episodes: Option<u32>,
    num_volumes: Option<u32>,
    num_chapters: Option<u32>,
    status: Option<String>,
    #[serde(default)]
    genres: Vec<MalGenre>,
    start_season: Option<MalStartSeason>,
    broadcast: Option<MalBroadcast>,
    start_date: Option<String>,
    end_date: Option<String>,
    media_type: Option<String>,
    #[serde(default)]
    studios: Vec<MalStudio>,
    #[serde(default)]
    authors: Vec<MalAuthorRole>,
    serialization: Option<MalSerialization>,
}

#[derive(Deserialize)]
struct MalPicture {
    medium: Option<String>,
    large: Option<String>,
}

#[derive(Deserialize)]
struct MalAlternativeTitles {
    synonyms: Option<Vec<String>>,
    en: Option<String>,
    ja: Option<String>,
}

#[derive(Deserialize)]
struct MalGenre {
    name: String,
}

#[derive(Deserialize)]
struct MalStudio {
    name: String,
}

#[derive(Deserialize)]
struct MalAuthorRole {
    node: Option<MalAuthor>,
}

#[derive(Deserialize)]
struct MalAuthor {
    first_name: Option<String>,
    last_name: Option<String>,
}

#[derive(Deserialize)]
struct MalSerialization {
    name: Option<String>,
}

#[derive(Deserialize)]
struct MalStartSeason {
    season: Option<String>,
    year: Option<u32>,
}

#[derive(Deserialize)]
struct MalBroadcast {
    day_of_the_week: Option<String>,
    start_time: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AnimeListBroadcast {
    day_of_the_week: String,
    start_time: String,
}

#[derive(Deserialize, Default)]
struct MalListStatus {
    status: Option<String>,
    score: Option<u32>,
    num_episodes_watched: Option<u32>,
    num_volumes_read: Option<u32>,
    num_chapters_read: Option<u32>,
    is_rewatching: Option<bool>,
    is_rereading: Option<bool>,
    comments: Option<String>,
    num_times_rewatched: Option<u32>,
    num_times_reread: Option<u32>,
    updated_at: Option<String>,
    start_date: Option<String>,
    finish_date: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AnimeListItem {
    id: u64,
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

#[derive(Deserialize)]
struct MalUserInfoResponse {
    id: u64,
    name: String,
    picture: Option<String>,
    anime_statistics: Option<MalAnimeStatistics>,
}

#[derive(Deserialize)]
struct MalAnimeStatistics {
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
pub struct MyAnimeListUserInfo {
    pub id: u64,
    pub name: String,
    pub picture: Option<String>,
    pub statistics: Option<UserStatistics>,
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
    fn from_mal(list_type: MyAnimeListListType, status: Option<&str>) -> Self {
        match list_type {
            MyAnimeListListType::Anime => match status {
                Some("watching") => Self::Watching,
                Some("completed") => Self::Completed,
                Some("on_hold") => Self::OnHold,
                Some("dropped") => Self::Dropped,
                Some("plan_to_watch") => Self::PlanToWatch,
                _ => Self::PlanToWatch,
            },
            MyAnimeListListType::Manga => match status {
                Some("reading") => Self::Reading,
                Some("completed") => Self::Completed,
                Some("on_hold") => Self::OnHold,
                Some("dropped") => Self::Dropped,
                Some("plan_to_read") => Self::PlanToRead,
                _ => Self::PlanToRead,
            },
        }
    }

    fn as_user_status_str(self) -> &'static str {
        match self {
            Self::Watching => "watching",
            Self::Completed => "completed",
            Self::OnHold => "onHold",
            Self::Dropped => "dropped",
            Self::PlanToWatch => "planToWatch",
            Self::Reading => "reading",
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

impl MyAnimeListListType {
    fn path_segment(self) -> &'static str {
        match self {
            Self::Anime => "animelist",
            Self::Manga => "mangalist",
        }
    }

    fn fields(self) -> &'static str {
        match self {
            Self::Anime => ANIME_FIELDS,
            Self::Manga => MANGA_FIELDS,
        }
    }
}

fn build_user_list_url(
    username: &str,
    list_type: MyAnimeListListType,
    offset: u32,
) -> Result<String, String> {
    let mut url = reqwest::Url::parse(BASE_URL).map_err(|e| e.to_string())?;
    {
        let mut segments = url
            .path_segments_mut()
            .map_err(|_| "Invalid MyAnimeList base URL".to_string())?;
        segments.push(username);
        segments.push(list_type.path_segment());
    }

    url.query_pairs_mut()
        .append_pair("fields", list_type.fields())
        .append_pair("nsfw", "true")
        .append_pair("limit", &LIMIT.to_string())
        .append_pair("offset", &offset.to_string());

    Ok(url.to_string())
}

fn parse_next_offset(next_url: &str) -> Option<u32> {
    let url = reqwest::Url::parse(next_url).ok()?;
    url.query_pairs()
        .find(|(key, _)| key == "offset")
        .and_then(|(_, value)| value.parse::<u32>().ok())
}

fn map_source(source: Option<String>) -> String {
    match source {
        Some(value) => match value.as_str() {
            "anime" => "Anime".to_string(),
            "manga" => "Manga".to_string(),
            "light_novel" => "Light Novel".to_string(),
            "visual_novel" => "Visual Novel".to_string(),
            "original" => "Original".to_string(),
            _ => value,
        },
        None => "Unknown".to_string(),
    }
}

fn map_status(status: Option<String>) -> String {
    match status {
        Some(value) => match value.as_str() {
            "finished_airing" => "Finished Airing".to_string(),
            "not_yet_aired" => "Not Yet Aired".to_string(),
            "currently_airing" => "Currently Airing".to_string(),
            "finished" => "Finished".to_string(),
            "currently_publishing" => "Currently Publishing".to_string(),
            "not_yet_published" => "Not Yet Published".to_string(),
            "on_hiatus" => "On Hiatus".to_string(),
            "discontinued" => "Discontinued".to_string(),
            _ => value,
        },
        None => "Unknown".to_string(),
    }
}

fn map_media_type(media_type: Option<String>) -> String {
    match media_type {
        Some(value) => match value.as_str() {
            "tv" => "TV".to_string(),
            "tv_special" => "Special".to_string(),
            "movie" => "Movie".to_string(),
            "special" => "Special".to_string(),
            "ona" => "ONA".to_string(),
            "ova" => "OVA".to_string(),
            "manga" => "Manga".to_string(),
            "novel" => "Novel".to_string(),
            "light_novel" => "Light Novel".to_string(),
            "one_shot" => "One-shot".to_string(),
            "doujinshi" => "Doujinshi".to_string(),
            "manhwa" => "Manhwa".to_string(),
            "manhua" => "Manhua".to_string(),
            "oel" => "OEL".to_string(),
            "unknown" => "Unknown".to_string(),
            _ => value,
        },
        None => "Unknown".to_string(),
    }
}

fn map_user_status_to_mal(
    list_type: MyAnimeListListType,
    status: &str,
) -> Option<&'static str> {
    match list_type {
        MyAnimeListListType::Anime => match status {
            "watching" => Some("watching"),
            "completed" => Some("completed"),
            "onHold" | "on_hold" => Some("on_hold"),
            "dropped" => Some("dropped"),
            "planToWatch" | "plan_to_watch" => Some("plan_to_watch"),
            _ => None,
        },
        MyAnimeListListType::Manga => match status {
            "reading" => Some("reading"),
            "completed" => Some("completed"),
            "onHold" | "on_hold" => Some("on_hold"),
            "dropped" => Some("dropped"),
            "planToRead" | "plan_to_read" => Some("plan_to_read"),
            _ => None,
        },
    }
}

impl From<ListType> for MyAnimeListListType {
    fn from(value: ListType) -> Self {
        match value {
            ListType::Anime => Self::Anime,
            ListType::Manga => Self::Manga,
        }
    }
}

fn join_names<T, F>(items: Vec<T>, mut f: F) -> String
where
    F: FnMut(T) -> String,
{
    if items.is_empty() {
        return "Unknown".to_string();
    }

    let mut result = String::new();
    for item in items.into_iter() {
        let name = f(item);
        if name.is_empty() {
            continue;
        }
        if !result.is_empty() {
            result.push_str(", ");
        }
        result.push_str(&name);
    }

    if result.is_empty() {
        "Unknown".to_string()
    } else {
        result
    }
}

fn build_alternative_titles(alt: Option<MalAlternativeTitles>) -> String {
    let Some(alt) = alt else {
        return "Unknown".to_string();
    };

    let mut parts: Vec<String> = Vec::new();

    if let Some(en) = alt.en {
        let trimmed = en.trim();
        if !trimmed.is_empty() {
            parts.push(trimmed.to_string());
        }
    }

    if let Some(ja) = alt.ja {
        let trimmed = ja.trim();
        if !trimmed.is_empty() {
            parts.push(trimmed.to_string());
        }
    }

    if let Some(synonyms) = alt.synonyms {
        for synonym in synonyms {
            let trimmed = synonym.trim();
            if !trimmed.is_empty() {
                parts.push(trimmed.to_string());
            }
        }
    }

    if parts.is_empty() {
        "Unknown".to_string()
    } else {
        parts.join(", ")
    }
}

fn format_start_season(start: Option<MalStartSeason>) -> String {
    let Some(start) = start else {
        return "Unknown".to_string();
    };

    let season_missing = start
        .season
        .as_deref()
        .map(|s| s.is_empty())
        .unwrap_or(true);
    let year_missing = start.year.is_none();

    if season_missing && year_missing {
        return "Unknown".to_string();
    }

    let mut season_part = start.season.unwrap_or_else(|| "Unknown".to_string());
    season_part.push(' ');
    let year_part = start
        .year
        .map(|year| year.to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    format!("{season_part}{year_part}")
}

fn build_author_names(authors: Vec<MalAuthorRole>) -> String {
    join_names(authors, |author_role| {
        let Some(author) = author_role.node else {
            return String::new();
        };

        let first_name = author.first_name.unwrap_or_default().trim().to_string();
        let last_name = author.last_name.unwrap_or_default().trim().to_string();

        match (first_name.is_empty(), last_name.is_empty()) {
            (true, true) => String::new(),
            (false, true) => first_name,
            (true, false) => last_name,
            (false, false) => format!("{first_name} {last_name}"),
        }
    })
}

fn build_serialization_label(serialization: Option<MalSerialization>) -> String {
    serialization
        .and_then(|value| value.name)
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "Unknown".to_string())
}

fn map_anime_entry_to_domain(entry: MalListEntry, status_key: UserStatusKey) -> AnimeListItem {
    let node = entry.node;
    let list_status = entry.list_status;

    let image_url = match node.main_picture {
        Some(picture) => picture.large.or(picture.medium).unwrap_or_default(),
        None => String::new(),
    };

    let alternative_titles = build_alternative_titles(node.alternative_titles);
    let synopsis = node
        .synopsis
        .unwrap_or_else(|| "No synopsis available.".to_string());
    let genres = join_names(node.genres, |genre| genre.name);
    let studios = join_names(node.studios, |studio| studio.name);
    let start_season = format_start_season(node.start_season);
    let broadcast = node.broadcast.unwrap_or(MalBroadcast {
        day_of_the_week: None,
        start_time: None,
    });

    AnimeListItem {
        id: node.id,
        title: node.title,
        image_url,
        synopsis,
        alternative_titles,
        score: node.mean.unwrap_or(0.0),
        source: map_source(node.source),
        status: map_status(node.status),
        total_episodes: node.num_episodes.unwrap_or(0),
        genres,
        start_season,
        start_date: node.start_date.unwrap_or_default(),
        studios,
        broadcast: AnimeListBroadcast {
            day_of_the_week: broadcast.day_of_the_week.unwrap_or_default(),
            start_time: broadcast.start_time.unwrap_or_default(),
        },
        media_type: map_media_type(node.media_type),
        user_status: status_key.as_user_status_str().to_string(),
        user_score: list_status.score.unwrap_or(0),
        user_episodes_watched: list_status.num_episodes_watched.unwrap_or(0),
        is_rewatching: list_status.is_rewatching.unwrap_or(false),
        user_comments: list_status.comments.unwrap_or_default(),
        user_num_times_rewatched: list_status.num_times_rewatched.unwrap_or(0),
        user_start_date: list_status.start_date,
        user_finish_date: list_status.finish_date,
        updated_at: list_status.updated_at,
    }
}

fn map_manga_entry_to_domain(entry: MalListEntry, status_key: UserStatusKey) -> MangaListItem {
    let node = entry.node;
    let list_status = entry.list_status;

    let image_url = match node.main_picture {
        Some(picture) => picture.large.or(picture.medium).unwrap_or_default(),
        None => String::new(),
    };

    MangaListItem {
        id: node.id,
        title: node.title,
        image_url,
        synopsis: node
            .synopsis
            .unwrap_or_else(|| "No synopsis available.".to_string()),
        alternative_titles: build_alternative_titles(node.alternative_titles),
        score: node.mean.unwrap_or(0.0),
        status: map_status(node.status),
        total_volumes: node.num_volumes.unwrap_or(0),
        total_chapters: node.num_chapters.unwrap_or(0),
        genres: join_names(node.genres, |genre| genre.name),
        start_date: node.start_date,
        end_date: node.end_date,
        authors: build_author_names(node.authors),
        serialization: build_serialization_label(node.serialization),
        media_type: map_media_type(node.media_type),
        user_status: status_key.as_user_status_str().to_string(),
        user_score: list_status.score.unwrap_or(0),
        user_volumes_read: list_status.num_volumes_read.unwrap_or(0),
        user_chapters_read: list_status.num_chapters_read.unwrap_or(0),
        is_rereading: list_status.is_rereading.unwrap_or(false),
        user_comments: list_status.comments.unwrap_or_default(),
        user_num_times_reread: list_status.num_times_reread.unwrap_or(0),
        user_start_date: list_status.start_date,
        user_finish_date: list_status.finish_date,
        updated_at: list_status.updated_at,
    }
}

fn map_mal_statistics(statistics: MalAnimeStatistics) -> UserStatistics {
    UserStatistics {
        num_items_watching: statistics.num_items_watching,
        num_items_completed: statistics.num_items_completed,
        num_items_on_hold: statistics.num_items_on_hold,
        num_items_dropped: statistics.num_items_dropped,
        num_items_plan_to_watch: statistics.num_items_plan_to_watch,
        num_items: statistics.num_items,
        num_days_watched: statistics.num_days_watched,
        num_days_watching: statistics.num_days_watching,
        num_days_completed: statistics.num_days_completed,
        num_days_on_hold: statistics.num_days_on_hold,
        num_days_dropped: statistics.num_days_dropped,
        num_days: statistics.num_days,
        num_episodes: statistics.num_episodes,
        num_times_rewatched: statistics.num_times_rewatched,
        mean_score: statistics.mean_score,
    }
}

async fn fetch_all_entries(
    client: &reqwest::Client,
    token: &str,
    username: &str,
    list_type: MyAnimeListListType,
    result: &mut Vec<MalListEntry>,
) -> Result<(), String> {
    let mut offset: u32 = 0;

    loop {
        let url = build_user_list_url(username, list_type, offset)?;
        let response = client
            .get(url)
            .bearer_auth(token)
            .timeout(std::time::Duration::from_secs(15))
            .send()
            .await
            .map_err(|e| e.to_string())?;
        let status_code = response.status();

        if !status_code.is_success() {
            let body = response.text().await.map_err(|e| e.to_string())?;
            return Err(format!(
                "MyAnimeList request failed: {} - {}",
                status_code, body
            ));
        }

        let parsed: MalListResponse = response.json().await.map_err(|e| e.to_string())?;

        result.extend(parsed.data);

        let next = parsed.paging.and_then(|p| p.next);
        let Some(next_url) = next else { break };
        offset = parse_next_offset(&next_url).unwrap_or(offset + LIMIT);
    }

    Ok(())
}

pub async fn update_myanimelist_list_entry(
    app: &tauri::AppHandle,
    client: &reqwest::Client,
    update: &AnimeListUpdateRequest,
) -> Result<(), String> {
    let token = get_access_token(app, MAL_PROVIDER_ID).await?;
    let list_type = MyAnimeListListType::from(update.list_type.unwrap_or_default());
    let entry_id = update
        .entry_id
        .ok_or_else(|| "Missing entryId for MyAnimeList update".to_string())?;

    let mut params: Vec<(String, String)> = Vec::new();

    if let Some(status) = update.user_status.as_deref() {
        let mapped = map_user_status_to_mal(list_type, status)
            .ok_or_else(|| format!("Invalid MyAnimeList status: {status}"))?;
        params.push(("status".to_string(), mapped.to_string()));
    }

    if let Some(score) = update.user_score {
        params.push(("score".to_string(), score.to_string()));
    }

    match list_type {
        MyAnimeListListType::Anime => {
            if let Some(episodes) = update.user_episodes_watched {
                params.push(("num_watched_episodes".to_string(), episodes.to_string()));
            }

            if let Some(is_rewatching) = update.is_rewatching {
                params.push(("is_rewatching".to_string(), is_rewatching.to_string()));
            }

            if let Some(num_times_rewatched) = update.user_num_times_rewatched {
                let clamped = num_times_rewatched.min(5);
                params.push(("num_times_rewatched".to_string(), clamped.to_string()));
            }
        }
        MyAnimeListListType::Manga => {
            if let Some(volumes) = update.user_volumes_read {
                params.push(("num_read_volumes".to_string(), volumes.to_string()));
            }

            if let Some(chapters) = update.user_chapters_read {
                params.push(("num_read_chapters".to_string(), chapters.to_string()));
            }

            if let Some(is_rereading) = update.is_rereading {
                params.push(("is_rereading".to_string(), is_rereading.to_string()));
            }

            if let Some(num_times_reread) = update.user_num_times_reread {
                let clamped = num_times_reread.min(5);
                params.push(("num_times_reread".to_string(), clamped.to_string()));
            }
        }
    }

    if let Some(comments) = update.user_comments.as_ref() {
        params.push(("comments".to_string(), comments.to_string()));
    }

    if let Some(start_date) = update.user_start_date.as_ref() {
        let trimmed = start_date.trim();
        if !trimmed.is_empty() {
            params.push(("start_date".to_string(), trimmed.to_string()));
        }
    }

    if let Some(finish_date) = update.user_finish_date.as_ref() {
        let trimmed = finish_date.trim();
        if !trimmed.is_empty() {
            params.push(("finish_date".to_string(), trimmed.to_string()));
        }
    }

    if params.is_empty() {
        return Err("No update fields provided".to_string());
    }

    let update_base_url = match list_type {
        MyAnimeListListType::Anime => ANIME_UPDATE_BASE_URL,
        MyAnimeListListType::Manga => MANGA_UPDATE_BASE_URL,
    };

    let url = format!("{}{}/my_list_status", update_base_url, entry_id);
    let response = client
        .put(url)
        .bearer_auth(token)
        .form(&params)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.map_err(|e| e.to_string())?;
        return Err(format!("MyAnimeList update failed: {} - {}", status, body));
    }

    Ok(())
}

#[tauri::command]
pub async fn fetch_myanimelist_user_info(
    app: tauri::AppHandle,
) -> Result<MyAnimeListUserInfo, String> {
    let token = get_access_token(&app, MAL_PROVIDER_ID).await?;
    let mut url = reqwest::Url::parse(BASE_URL).map_err(|e| e.to_string())?;
    {
        let mut segments = url
            .path_segments_mut()
            .map_err(|_| "Invalid MyAnimeList base URL".to_string())?;
        segments.push("@me");
    }

    url.query_pairs_mut()
        .append_pair("fields", USER_INFO_FIELDS);

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .bearer_auth(token)
        .timeout(std::time::Duration::from_secs(15))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.map_err(|e| e.to_string())?;
        return Err(format!(
            "MyAnimeList user info request failed: {} - {}",
            status, body
        ));
    }

    let raw = response
        .json::<MalUserInfoResponse>()
        .await
        .map_err(|e| e.to_string())?;

    Ok(MyAnimeListUserInfo {
        id: raw.id,
        name: raw.name,
        picture: raw.picture,
        statistics: raw.anime_statistics.map(map_mal_statistics),
    })
}

#[tauri::command]
pub async fn synchronize_myanimelist(
    app: tauri::AppHandle,
    list_type: Option<MyAnimeListListType>,
) -> Result<SynchronizedListResult, String> {
    let token = get_access_token(&app, MAL_PROVIDER_ID).await?;
    let list_type = list_type.unwrap_or_default();
    let username: Option<String> = app.zustand().get_or_default("myanimelist", "username");
    let username = username
        .and_then(|value| {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        })
        .unwrap_or_else(|| "@me".to_string());
    let client = reqwest::Client::new();
    let mut entries = Vec::new();
    fetch_all_entries(&client, &token, &username, list_type, &mut entries).await?;

    match list_type {
        MyAnimeListListType::Anime => {
            let mut result = SynchronizedAnimeList::default();

            for entry in entries {
                let status_key = UserStatusKey::from_mal(
                    MyAnimeListListType::Anime,
                    entry.list_status.status.as_deref(),
                );
                let item = map_anime_entry_to_domain(entry, status_key);
                status_key.push_anime(&mut result, item);
            }

            Ok(SynchronizedListResult::Anime(result))
        }
        MyAnimeListListType::Manga => {
            let mut result = SynchronizedMangaList::default();

            for entry in entries {
                let status_key = UserStatusKey::from_mal(
                    MyAnimeListListType::Manga,
                    entry.list_status.status.as_deref(),
                );
                let item = map_manga_entry_to_domain(entry, status_key);
                status_key.push_manga(&mut result, item);
            }

            Ok(SynchronizedListResult::Manga(result))
        }
    }
}
