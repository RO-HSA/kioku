use std::time::Duration;

use serde_json;
use tauri_plugin_http::reqwest;
use tauri_plugin_zustand::ManagerExt;

use crate::auth::anilist::PROVIDER_ID as ANILIST_PROVIDER_ID;
use crate::auth::token_manager::get_access_token;
use crate::services::anime_list_updates::{AnimeListUpdateRequest, ListType};

use super::mapping::{
    map_anilist_statistics, map_anime_to_domain, map_manga_to_domain, map_user_status_to_anilist,
    parse_fuzzy_date_input,
};
use super::{
    AniListCollection, AniListSearchPage, AniListSearchResult, AniListUserInfo, GraphQlError,
    GraphQlRequest, GraphQlResponse, GraphQlVariables, SaveMediaListEntryMutationResponse,
    SaveMediaListEntryRequest, SaveMediaListEntryVariables, SearchMediaRequest,
    SearchMediaResponse, SearchMediaVariables, SynchronizedAnimeList, SynchronizedListResult,
    SynchronizedMangaList, UserStatusKey, ViewerRequest, ViewerResponse, GRAPHQL_URL,
    MEDIA_LIST_COLLECTION_QUERY, MEDIA_TYPE_ANIME, MEDIA_TYPE_MANGA, REQUEST_TIMEOUT_SECS,
    SAVE_MEDIA_LIST_ENTRY_MUTATION, SEARCH_LIMIT_MAX, SEARCH_MEDIA_QUERY, VIEWER_QUERY,
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

fn normalize_search_query(query: &str) -> Result<String, String> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        return Err("Search query cannot be empty".to_string());
    }

    Ok(trimmed.to_string())
}

fn normalize_search_limit(limit: Option<u32>) -> u32 {
    limit.unwrap_or(SEARCH_LIMIT_MAX).clamp(1, SEARCH_LIMIT_MAX)
}

fn format_transport_error(operation: &str, error: &reqwest::Error) -> String {
    let hint = if error.is_timeout() {
        "request timed out"
    } else if error.is_connect() {
        "connection failed"
    } else if error.is_body() {
        "response body read failed"
    } else if error.is_decode() {
        "response decode failed"
    } else if error.is_request() {
        "request failed before a response was received"
    } else {
        "request failed"
    };

    format!("{operation}: {hint} ({error})")
}

fn build_save_media_list_entry_variables(
    update: &AnimeListUpdateRequest,
) -> Result<SaveMediaListEntryVariables, String> {
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

    Ok(SaveMediaListEntryVariables {
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
    })
}

fn parse_collection_response(
    status: reqwest::StatusCode,
    body: &str,
) -> Result<AniListCollection, String> {
    if !status.is_success() {
        return Err(format!("AniList request failed: {} - {}", status, body));
    }

    let parsed: GraphQlResponse =
        serde_json::from_str(body).map_err(|e| format!("Failed to parse AniList response: {e}"))?;

    map_graphql_errors(parsed.errors)?;

    parsed
        .data
        .and_then(|data| data.media_list_collection)
        .ok_or_else(|| "AniList response missing MediaListCollection".to_string())
}

fn parse_viewer_response(
    status: reqwest::StatusCode,
    body: &str,
) -> Result<AniListUserInfo, String> {
    if !status.is_success() {
        return Err(format!("AniList request failed: {} - {}", status, body));
    }

    let parsed: ViewerResponse =
        serde_json::from_str(body).map_err(|e| format!("Failed to parse AniList response: {e}"))?;

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

fn parse_search_response(
    status: reqwest::StatusCode,
    body: &str,
) -> Result<AniListSearchPage, String> {
    if !status.is_success() {
        return Err(format!("AniList request failed: {} - {}", status, body));
    }

    let parsed: SearchMediaResponse =
        serde_json::from_str(body).map_err(|e| format!("Failed to parse AniList response: {e}"))?;

    map_graphql_errors(parsed.errors)?;

    parsed
        .data
        .and_then(|data| data.page)
        .ok_or_else(|| "AniList response missing Page".to_string())
}

fn parse_save_media_list_entry_response(
    status: reqwest::StatusCode,
    body: &str,
) -> Result<(), String> {
    if !status.is_success() {
        return Err(format!("AniList update failed: {} - {}", status, body));
    }

    let parsed: SaveMediaListEntryMutationResponse = serde_json::from_str(body)
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
        .map_err(|e| format_transport_error("AniList sync request failed", &e))?;
    let status = response.status();

    let body = response
        .text()
        .await
        .map_err(|e| format_transport_error("AniList sync response read failed", &e))?;
    parse_collection_response(status, &body)
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
        .map_err(|e| format_transport_error("AniList user info request failed", &e))?;
    let status = response.status();

    let body = response
        .text()
        .await
        .map_err(|e| format_transport_error("AniList user info response read failed", &e))?;
    parse_viewer_response(status, &body)
}

async fn fetch_search_media(
    client: &reqwest::Client,
    token: &str,
    query: &str,
    list_type: ListType,
    limit: Option<u32>,
) -> Result<AniListSearchPage, String> {
    let query = normalize_search_query(query)?;
    let request = SearchMediaRequest {
        query: SEARCH_MEDIA_QUERY,
        variables: SearchMediaVariables {
            search: &query,
            r#type: match list_type {
                ListType::Anime => MEDIA_TYPE_ANIME,
                ListType::Manga => MEDIA_TYPE_MANGA,
            },
            per_page: normalize_search_limit(limit),
        },
    };

    let response = client
        .post(GRAPHQL_URL)
        .bearer_auth(token)
        .json(&request)
        .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
        .send()
        .await
        .map_err(|e| format_transport_error("AniList search request failed", &e))?;
    let status = response.status();

    let body = response
        .text()
        .await
        .map_err(|e| format_transport_error("AniList search response read failed", &e))?;
    parse_search_response(status, &body)
}

#[tauri::command]
pub async fn fetch_anilist_user_info(app: tauri::AppHandle) -> Result<AniListUserInfo, String> {
    let token = get_access_token(&app, ANILIST_PROVIDER_ID).await?;
    let client = reqwest::Client::new();
    fetch_viewer(&client, &token).await
}

#[tauri::command]
pub async fn search_anilist_media(
    app: tauri::AppHandle,
    query: String,
    list_type: Option<ListType>,
    limit: Option<u32>,
) -> Result<AniListSearchResult, String> {
    let token = get_access_token(&app, ANILIST_PROVIDER_ID).await?;
    let list_type = list_type.unwrap_or_default();
    let client = reqwest::Client::new();
    let page = fetch_search_media(&client, &token, &query, list_type, limit).await?;

    match list_type {
        ListType::Anime => Ok(AniListSearchResult::Anime(
            page.media
                .into_iter()
                .flatten()
                .map(|mut media| {
                    let status_key = media
                        .media_list_entry
                        .as_ref()
                        .and_then(|entry| entry.status.as_deref())
                        .map(|status| UserStatusKey::from_anilist(ListType::Anime, Some(status)))
                        .unwrap_or_else(|| UserStatusKey::default_search(ListType::Anime));
                    let media_list_entry = media.media_list_entry.take().unwrap_or_default();

                    map_anime_to_domain(media, media_list_entry, status_key)
                })
                .collect(),
        )),
        ListType::Manga => Ok(AniListSearchResult::Manga(
            page.media
                .into_iter()
                .flatten()
                .map(|mut media| {
                    let status_key = media
                        .media_list_entry
                        .as_ref()
                        .and_then(|entry| entry.status.as_deref())
                        .map(|status| UserStatusKey::from_anilist(ListType::Manga, Some(status)))
                        .unwrap_or_else(|| UserStatusKey::default_search(ListType::Manga));
                    let media_list_entry = media.media_list_entry.take().unwrap_or_default();

                    map_manga_to_domain(media, media_list_entry, status_key)
                })
                .collect(),
        )),
    }
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
    let variables = build_save_media_list_entry_variables(update)?;

    let payload = SaveMediaListEntryRequest {
        query: SAVE_MEDIA_LIST_ENTRY_MUTATION,
        variables,
    };

    let response = client
        .post(GRAPHQL_URL)
        .bearer_auth(token)
        .json(&payload)
        .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
        .send()
        .await
        .map_err(|e| format_transport_error("AniList update request failed", &e))?;
    let status_code = response.status();
    let body = response
        .text()
        .await
        .map_err(|e| format_transport_error("AniList update response read failed", &e))?;
    parse_save_media_list_entry_response(status_code, &body)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_update() -> AnimeListUpdateRequest {
        AnimeListUpdateRequest {
            provider_id: ANILIST_PROVIDER_ID.to_string(),
            list_type: Some(ListType::Anime),
            entry_id: Some(10),
            media_id: None,
            user_status: None,
            user_score: None,
            user_episodes_watched: None,
            user_volumes_read: None,
            user_chapters_read: None,
            is_rewatching: None,
            is_rereading: None,
            user_comments: None,
            user_num_times_rewatched: None,
            user_num_times_reread: None,
            user_start_date: None,
            user_finish_date: None,
        }
    }

    #[test]
    fn map_graphql_errors_joins_messages_and_ignores_empty_sets() {
        assert!(map_graphql_errors(None).is_ok());
        assert!(map_graphql_errors(Some(Vec::new())).is_ok());

        let error = map_graphql_errors(Some(vec![
            GraphQlError {
                message: "first".to_string(),
            },
            GraphQlError {
                message: "second".to_string(),
            },
        ]))
        .unwrap_err();

        assert_eq!(error, "AniList GraphQL error: first; second");
    }

    #[test]
    fn build_save_media_list_entry_variables_for_anime_trims_and_derives_fields() {
        let mut update = base_update();
        update.user_status = Some("completed".to_string());
        update.user_score = Some(9);
        update.user_episodes_watched = Some(12);
        update.is_rewatching = Some(true);
        update.user_comments = Some("  finale  ".to_string());
        update.user_start_date = Some("2024-01-01".to_string());
        update.user_finish_date = Some("2024-03-22".to_string());

        let variables =
            build_save_media_list_entry_variables(&update).expect("variables should build");

        assert_eq!(variables.save_media_list_entry_id, Some(10));
        assert_eq!(variables.media_id, None);
        assert_eq!(variables.status.as_deref(), Some("COMPLETED"));
        assert_eq!(variables.score, Some(9.0));
        assert_eq!(variables.progress, Some(12));
        assert_eq!(variables.progress_volumes, None);
        assert_eq!(variables.repeat, Some(1));
        assert_eq!(variables.notes.as_deref(), Some("finale"));
        assert_eq!(
            variables.started_at.as_ref().map(|date| date.year),
            Some(2024)
        );
        assert_eq!(
            variables.completed_at.as_ref().map(|date| date.day),
            Some(22)
        );
    }

    #[test]
    fn build_save_media_list_entry_variables_for_manga_prefers_media_id_and_explicit_reread_count()
    {
        let mut update = base_update();
        update.list_type = Some(ListType::Manga);
        update.entry_id = Some(55);
        update.media_id = Some(77);
        update.user_status = Some("plan_to_read".to_string());
        update.user_chapters_read = Some(42);
        update.user_volumes_read = Some(7);
        update.user_num_times_reread = Some(3);
        update.is_rereading = Some(true);
        update.user_comments = Some("   ".to_string());

        let variables =
            build_save_media_list_entry_variables(&update).expect("variables should build");

        assert_eq!(variables.save_media_list_entry_id, None);
        assert_eq!(variables.media_id, Some(77));
        assert_eq!(variables.status.as_deref(), Some("PLANNING"));
        assert_eq!(variables.progress, Some(42));
        assert_eq!(variables.progress_volumes, Some(7));
        assert_eq!(variables.repeat, Some(3));
        assert_eq!(variables.notes, None);
    }

    #[test]
    fn build_save_media_list_entry_variables_rejects_missing_fields_and_invalid_targets() {
        let update = base_update();
        assert_eq!(
            build_save_media_list_entry_variables(&update)
                .err()
                .as_deref(),
            Some("No update fields provided")
        );

        let mut update = base_update();
        update.entry_id = None;
        update.user_score = Some(8);
        assert_eq!(
            build_save_media_list_entry_variables(&update)
                .err()
                .as_deref(),
            Some("Missing AniList target id: provide entryId or mediaId")
        );
    }

    #[test]
    fn build_save_media_list_entry_variables_rejects_invalid_status_and_dates() {
        let mut invalid_status = base_update();
        invalid_status.user_status = Some("reading".to_string());
        assert_eq!(
            build_save_media_list_entry_variables(&invalid_status)
                .err()
                .as_deref(),
            Some("Invalid AniList status: reading")
        );

        let mut invalid_date = base_update();
        invalid_date.user_score = Some(7);
        invalid_date.user_start_date = Some("2024-99-01".to_string());
        assert_eq!(
            build_save_media_list_entry_variables(&invalid_date)
                .err()
                .as_deref(),
            Some("Invalid userStartDate: expected YYYY-MM-DD")
        );
    }

    #[test]
    fn parse_collection_response_accepts_successful_payloads() {
        let collection = parse_collection_response(
            reqwest::StatusCode::OK,
            r#"{
                "data": {
                    "MediaListCollection": {
                        "lists": [],
                        "hasNextChunk": true
                    }
                }
            }"#,
        )
        .expect("collection should parse");

        assert!(collection.lists.is_empty());
        assert!(collection.has_next_chunk);
    }

    #[test]
    fn parse_collection_response_rejects_http_errors_invalid_json_and_missing_data() {
        assert_eq!(
            parse_collection_response(reqwest::StatusCode::UNAUTHORIZED, "denied")
                .err()
                .as_deref(),
            Some("AniList request failed: 401 Unauthorized - denied")
        );

        assert!(parse_collection_response(reqwest::StatusCode::OK, "{")
            .err()
            .expect("invalid json should fail")
            .starts_with("Failed to parse AniList response:"));

        assert_eq!(
            parse_collection_response(reqwest::StatusCode::OK, r#"{"data":{}}"#)
                .err()
                .as_deref(),
            Some("AniList response missing MediaListCollection")
        );
    }

    #[test]
    fn parse_collection_response_surfaces_graphql_errors() {
        assert_eq!(
            parse_collection_response(
                reqwest::StatusCode::OK,
                r#"{"errors":[{"message":"forbidden"},{"message":"rate limited"}]}"#,
            )
            .err()
            .as_deref(),
            Some("AniList GraphQL error: forbidden; rate limited")
        );
    }

    #[test]
    fn parse_viewer_response_maps_user_info_and_statistics() {
        let user = parse_viewer_response(
            reqwest::StatusCode::OK,
            r#"{
                "data": {
                    "Viewer": {
                        "id": 7,
                        "name": "robert",
                        "avatar": {
                            "large": "https://img.example/avatar.png"
                        },
                        "statistics": {
                            "anime": {
                                "count": 15,
                                "episodesWatched": 24,
                                "meanScore": 81.5,
                                "minutesWatched": 1440,
                                "statuses": [
                                    {"count": 1, "minutesWatched": 120, "status": "CURRENT"},
                                    {"count": 2, "minutesWatched": 240, "status": "COMPLETED"},
                                    {"count": 3, "minutesWatched": 360, "status": "PAUSED"},
                                    {"count": 4, "minutesWatched": 480, "status": "DROPPED"},
                                    {"count": 5, "minutesWatched": 0, "status": "PLANNING"}
                                ]
                            }
                        }
                    }
                }
            }"#,
        )
        .expect("viewer should parse");

        let statistics = user.statistics.expect("statistics should be present");

        assert_eq!(user.id, 7);
        assert_eq!(user.name, "robert");
        assert_eq!(
            user.picture.as_deref(),
            Some("https://img.example/avatar.png")
        );
        assert_eq!(statistics.num_items, 15);
        assert_eq!(statistics.num_items_watching, 1);
        assert_eq!(statistics.num_items_completed, 2);
        assert_eq!(statistics.num_items_on_hold, 3);
        assert_eq!(statistics.num_items_dropped, 4);
        assert_eq!(statistics.num_items_plan_to_watch, 5);
        assert_eq!(statistics.num_episodes, 24);
        assert!((statistics.num_days_watched - 1.0).abs() < f64::EPSILON);
        assert_eq!(statistics.mean_score, 81.5);
    }

    #[test]
    fn parse_viewer_response_rejects_http_errors_invalid_json_missing_viewer_and_graphql_errors() {
        assert_eq!(
            parse_viewer_response(reqwest::StatusCode::FORBIDDEN, "blocked")
                .err()
                .as_deref(),
            Some("AniList request failed: 403 Forbidden - blocked")
        );

        assert!(parse_viewer_response(reqwest::StatusCode::OK, "{")
            .err()
            .expect("invalid json should fail")
            .starts_with("Failed to parse AniList response:"));

        assert_eq!(
            parse_viewer_response(reqwest::StatusCode::OK, r#"{"data":{}}"#)
                .err()
                .as_deref(),
            Some("AniList response missing Viewer")
        );

        assert_eq!(
            parse_viewer_response(
                reqwest::StatusCode::OK,
                r#"{"errors":[{"message":"session expired"}]}"#,
            )
            .err()
            .as_deref(),
            Some("AniList GraphQL error: session expired")
        );
    }

    #[test]
    fn parse_save_media_list_entry_response_validates_success_and_error_paths() {
        parse_save_media_list_entry_response(
            reqwest::StatusCode::OK,
            r#"{"data":{"SaveMediaListEntry":{"id":123}}}"#,
        )
        .expect("successful mutation should be accepted");

        assert_eq!(
            parse_save_media_list_entry_response(reqwest::StatusCode::BAD_REQUEST, "invalid")
                .err()
                .as_deref(),
            Some("AniList update failed: 400 Bad Request - invalid")
        );

        assert!(
            parse_save_media_list_entry_response(reqwest::StatusCode::OK, "{")
                .unwrap_err()
                .starts_with("Failed to parse AniList update response:")
        );

        assert_eq!(
            parse_save_media_list_entry_response(
                reqwest::StatusCode::OK,
                r#"{"errors":[{"message":"mutation denied"}]}"#,
            )
            .err()
            .as_deref(),
            Some("AniList GraphQL error: mutation denied")
        );

        assert_eq!(
            parse_save_media_list_entry_response(reqwest::StatusCode::OK, r#"{"data":null}"#)
                .err()
                .as_deref(),
            Some("AniList update response missing data")
        );

        assert_eq!(
            parse_save_media_list_entry_response(reqwest::StatusCode::OK, r#"{"data":{}}"#)
                .err()
                .as_deref(),
            Some("AniList update response missing SaveMediaListEntry")
        );

        assert_eq!(
            parse_save_media_list_entry_response(
                reqwest::StatusCode::OK,
                r#"{"data":{"SaveMediaListEntry":{"id":0}}}"#,
            )
            .err()
            .as_deref(),
            Some("AniList update returned an invalid entry id")
        );
    }
}
