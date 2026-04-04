use serde::{Deserialize, Serialize};

use crate::services::anime_list_updates::ListType;

mod api;
mod mapping;

pub use api::{
    fetch_myanimelist_user_info, synchronize_myanimelist, update_myanimelist_list_entry,
};

const BASE_URL: &str = "https://api.myanimelist.net/v2/users";
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

impl From<ListType> for MyAnimeListListType {
    fn from(value: ListType) -> Self {
        match value {
            ListType::Anime => Self::Anime,
            ListType::Manga => Self::Manga,
        }
    }
}
