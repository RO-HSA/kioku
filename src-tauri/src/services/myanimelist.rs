use serde::{Deserialize, Serialize};
use tauri_plugin_http::reqwest;
use tauri_plugin_zustand::ManagerExt;

use crate::auth::mal::PROVIDER_ID as MAL_PROVIDER_ID;
use crate::auth::token_manager::get_access_token;
use crate::services::anime_list_updates::AnimeListUpdateRequest;

const BASE_URL: &str = "https://api.myanimelist.net/v2/users/";
const UPDATE_BASE_URL: &str = "https://api.myanimelist.net/v2/anime/";
const FIELDS: &str = "list_status,synopsis,alternative_titles,source,num_episodes,nsfw,start_season,media_type,studios,mean,status,genres,broadcast,start_date";
const LIMIT: u32 = 1000;

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
    status: Option<String>,
    #[serde(default)]
    genres: Vec<MalGenre>,
    start_season: Option<MalStartSeason>,
    broadcast: Option<MalBroadcast>,
    start_date: Option<String>,
    media_type: Option<String>,
    #[serde(default)]
    studios: Vec<MalStudio>,
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
    is_rewatching: Option<bool>,
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
    fn from_mal(status: Option<&str>) -> Self {
        match status {
            Some("watching") => Self::Watching,
            Some("completed") => Self::Completed,
            Some("on_hold") => Self::OnHold,
            Some("dropped") => Self::Dropped,
            Some("plan_to_watch") => Self::PlanToWatch,
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

fn build_animelist_url(username: &str, offset: u32) -> Result<String, String> {
    let mut url = reqwest::Url::parse(BASE_URL).map_err(|e| e.to_string())?;
    {
        let mut segments = url
            .path_segments_mut()
            .map_err(|_| "Invalid MyAnimeList base URL".to_string())?;
        segments.push(username);
        segments.push("animelist");
    }

    url.query_pairs_mut()
        .append_pair("fields", FIELDS)
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
            "unknown" => "Unknown".to_string(),
            _ => value,
        },
        None => "Unknown".to_string(),
    }
}

fn map_user_status_to_mal(status: &str) -> Option<&'static str> {
    match status {
        "watching" => Some("watching"),
        "completed" => Some("completed"),
        "onHold" | "on_hold" => Some("on_hold"),
        "dropped" => Some("dropped"),
        "planToWatch" | "plan_to_watch" => Some("plan_to_watch"),
        _ => None,
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

    let mut result = String::new();

    if let Some(en) = alt.en {
        result.push_str(&en);
        result.push_str(", ");
    }

    if let Some(ja) = alt.ja {
        result.push_str(&ja);
        result.push_str(", ");
    }

    if let Some(synonyms) = alt.synonyms {
        if !synonyms.is_empty() {
            result.push_str(&synonyms.join(", "));
        }
    }

    if result.is_empty() {
        "Unknown".to_string()
    } else {
        result
    }
}

fn format_start_season(start: Option<MalStartSeason>) -> String {
    let Some(start) = start else {
        return "Unknown".to_string();
    };

    let season_missing = start.season.as_deref().map(|s| s.is_empty()).unwrap_or(true);
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

fn map_entry_to_domain(entry: MalListEntry, status_key: UserStatusKey) -> AnimeListItem {
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
        user_start_date: list_status.start_date,
        user_finish_date: list_status.finish_date,
        updated_at: list_status.updated_at,
    }
}

async fn fetch_all_into(
    client: &reqwest::Client,
    token: &str,
    username: &str,
    result: &mut SynchronizedAnimeList,
) -> Result<(), String> {
    let mut offset: u32 = 0;

    loop {
        let url = build_animelist_url(username, offset)?;
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

        let parsed: MalListResponse =
            response.json().await.map_err(|e| e.to_string())?;

        for entry in parsed.data {
            let status_key = UserStatusKey::from_mal(entry.list_status.status.as_deref());
            let item = map_entry_to_domain(entry, status_key);
            status_key.push(result, item);
        }

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

    let mut params: Vec<(String, String)> = Vec::new();

    if let Some(status) = update.user_status.as_deref() {
        let mapped = map_user_status_to_mal(status)
            .ok_or_else(|| format!("Invalid MyAnimeList status: {status}"))?;
        params.push(("status".to_string(), mapped.to_string()));
    }

    if let Some(score) = update.user_score {
        params.push(("score".to_string(), score.to_string()));
    }

    if let Some(episodes) = update.user_episodes_watched {
        params.push(("num_watched_episodes".to_string(), episodes.to_string()));
    }

    if let Some(is_rewatching) = update.is_rewatching {
        params.push(("is_rewatching".to_string(), is_rewatching.to_string()));
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

    let url = format!("{}{}/my_list_status", UPDATE_BASE_URL, update.entry_id);
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
        return Err(format!(
            "MyAnimeList update failed: {} - {}",
            status, body
        ));
    }

    Ok(())
}

#[tauri::command]
pub async fn synchronize_myanimelist(
    app: tauri::AppHandle,
) -> Result<SynchronizedAnimeList, String> {
    let token = get_access_token(&app, MAL_PROVIDER_ID).await?;
    let username: Option<String> = app
        .zustand()
        .get_or_default("myanimelist", "username");
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

    let mut result = SynchronizedAnimeList::default();
    fetch_all_into(&client, &token, &username, &mut result).await?;

    Ok(result)
}
