use std::time::Duration;

use serde_json;
use tauri_plugin_http::reqwest;
use tauri_plugin_zustand::ManagerExt;

use crate::auth::anilist::PROVIDER_ID as ANILIST_PROVIDER_ID;
use crate::auth::token_manager::get_access_token;
use crate::services::anime_list_updates::{AnimeListUpdateRequest, ListType};

use super::mapping::{
    map_anilist_statistics, map_anime_to_domain, map_manga_to_domain,
    map_user_status_to_anilist, parse_fuzzy_date_input,
};
use super::{
    AniListCollection, AniListUserInfo, GraphQlError, GraphQlRequest, GraphQlResponse,
    GraphQlVariables, SaveMediaListEntryMutationResponse, SaveMediaListEntryRequest,
    SaveMediaListEntryVariables, SynchronizedAnimeList, SynchronizedListResult,
    SynchronizedMangaList, UserStatusKey, ViewerRequest, ViewerResponse, GRAPHQL_URL,
    MEDIA_LIST_COLLECTION_QUERY, MEDIA_TYPE_ANIME, MEDIA_TYPE_MANGA, REQUEST_TIMEOUT_SECS,
    SAVE_MEDIA_LIST_ENTRY_MUTATION, VIEWER_QUERY,
};

fn map_graphql_errors(errors: Option<Vec<GraphQlError>>) -> Result<(), String> {
    if let Some(errors) = errors {
        if !errors.is_empty() {
            let message = errors
                .into_iter()
                .map(|error| error.message)
                .collect::<Vec<_>>()
                .join("; ");
            return Err(format!("AniList GraphQL error: {message}"));
        }
    }

    Ok(())
}

async fn fetch_collection(
    client: &reqwest::Client,
    token: &str,
    username: Option<&str>,
    list_type: ListType,
) -> Result<AniListCollection, String> {
    let request = GraphQlRequest {
        query: MEDIA_LIST_COLLECTION_QUERY,
        variables: GraphQlVariables {
            r#type: match list_type {
                ListType::Anime => MEDIA_TYPE_ANIME,
                ListType::Manga => MEDIA_TYPE_MANGA,
            },
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

    map_graphql_errors(parsed.errors)?;

    parsed
        .data
        .and_then(|data| data.media_list_collection)
        .ok_or_else(|| "AniList response missing MediaListCollection".to_string())
}

async fn fetch_viewer(client: &reqwest::Client, token: &str) -> Result<AniListUserInfo, String> {
    let request = ViewerRequest {
        query: VIEWER_QUERY,
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

    let parsed: ViewerResponse = serde_json::from_str(&body)
        .map_err(|e| format!("Failed to parse AniList response: {e}"))?;

    map_graphql_errors(parsed.errors)?;

    let viewer = parsed
        .data
        .and_then(|data| data.viewer)
        .ok_or_else(|| "AniList response missing Viewer".to_string())?;

    Ok(AniListUserInfo {
        id: viewer.id,
        name: viewer.name,
        picture: viewer.avatar.and_then(|avatar| avatar.large),
        statistics: viewer
            .statistics
            .and_then(|statistics| statistics.anime)
            .map(map_anilist_statistics),
    })
}

#[tauri::command]
pub async fn fetch_anilist_user_info(app: tauri::AppHandle) -> Result<AniListUserInfo, String> {
    let token = get_access_token(&app, ANILIST_PROVIDER_ID).await?;
    let client = reqwest::Client::new();
    fetch_viewer(&client, &token).await
}

#[tauri::command]
pub async fn synchronize_anilist(
    app: tauri::AppHandle,
    list_type: Option<ListType>,
) -> Result<SynchronizedListResult, String> {
    let token = get_access_token(&app, ANILIST_PROVIDER_ID).await?;
    let list_type = list_type.unwrap_or_default();
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
    let collection = fetch_collection(&client, &token, username.as_deref(), list_type).await?;
    let mut anime_result = SynchronizedAnimeList::default();
    let mut manga_result = SynchronizedMangaList::default();

    if collection.has_next_chunk {
        eprintln!(
            "AniList returned additional chunks; only the first chunk is currently synchronized."
        );
    }

    for list in collection.lists {
        let list_status = list.status;

        for entry in list.entries {
            let Some(mut media) = entry.media else {
                continue;
            };

            let status_key = UserStatusKey::from_anilist(
                list_type,
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

            match list_type {
                ListType::Anime => {
                    let item = map_anime_to_domain(media, media_list_entry, status_key);
                    status_key.push_anime(&mut anime_result, item);
                }
                ListType::Manga => {
                    let item = map_manga_to_domain(media, media_list_entry, status_key);
                    status_key.push_manga(&mut manga_result, item);
                }
            }
        }
    }

    match list_type {
        ListType::Anime => Ok(SynchronizedListResult::Anime(anime_result)),
        ListType::Manga => Ok(SynchronizedListResult::Manga(manga_result)),
    }
}

pub async fn update_anilist_list_entry(
    app: &tauri::AppHandle,
    client: &reqwest::Client,
    update: &AnimeListUpdateRequest,
) -> Result<(), String> {
    let token = get_access_token(app, ANILIST_PROVIDER_ID).await?;
    let list_type = update.list_type.unwrap_or_default();

    let status = update
        .user_status
        .as_deref()
        .map(|value| map_user_status_to_anilist(list_type, value))
        .transpose()?
        .map(|value| value.to_string());
    let score = update.user_score.map(|value| value as f64);
    let (progress, progress_volumes, repeat) = match list_type {
        ListType::Anime => {
            let repeat = match (update.user_num_times_rewatched, update.is_rewatching) {
                (Some(value), _) => Some(value),
                (None, Some(true)) => Some(1),
                (None, Some(false)) => Some(0),
                (None, None) => None,
            };

            (update.user_episodes_watched, None, repeat)
        }
        ListType::Manga => {
            let repeat = match (update.user_num_times_reread, update.is_rereading) {
                (Some(value), _) => Some(value),
                (None, Some(true)) => Some(1),
                (None, Some(false)) => Some(0),
                (None, None) => None,
            };

            (update.user_chapters_read, update.user_volumes_read, repeat)
        }
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
        && progress_volumes.is_none()
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
            progress_volumes,
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

    map_graphql_errors(parsed.errors)?;

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
