use std::time::Duration;

use tauri_plugin_http::reqwest;
use tauri_plugin_zustand::ManagerExt;

use crate::auth::mal::PROVIDER_ID as MAL_PROVIDER_ID;
use crate::auth::token_manager::get_access_token;
use crate::services::anime_list_updates::AnimeListUpdateRequest;

use super::mapping::{
    map_anime_entry_to_domain, map_mal_statistics, map_manga_entry_to_domain,
    map_user_status_to_mal,
};
use super::{
    MalListEntry, MalListResponse, MyAnimeListListType, MyAnimeListUserInfo, SynchronizedAnimeList,
    SynchronizedListResult, SynchronizedMangaList, UserStatusKey, ANIME_UPDATE_BASE_URL, BASE_URL,
    LIMIT, MANGA_UPDATE_BASE_URL, USER_INFO_FIELDS,
};

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
            .timeout(Duration::from_secs(15))
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
                params.push(("num_volumes_read".to_string(), volumes.to_string()));
            }

            if let Some(chapters) = update.user_chapters_read {
                params.push(("num_chapters_read".to_string(), chapters.to_string()));
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
        .timeout(Duration::from_secs(15))
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
        .json::<super::MalUserInfoResponse>()
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn build_user_list_url_uses_expected_path_and_query_values() {
        let url = build_user_list_url("robert", MyAnimeListListType::Anime, 300)
            .expect("url should build");
        let parsed = reqwest::Url::parse(&url).expect("built url should parse");
        let segments = parsed
            .path_segments()
            .expect("path segments should exist")
            .collect::<Vec<_>>();
        let query = parsed
            .query_pairs()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect::<HashMap<_, _>>();

        assert_eq!(parsed.domain(), Some("api.myanimelist.net"));
        assert_eq!(segments, vec!["v2", "users", "robert", "animelist"]);
        assert_eq!(
            query.get("fields"),
            Some(&MyAnimeListListType::Anime.fields().to_string())
        );
        assert_eq!(query.get("nsfw"), Some(&"true".to_string()));
        assert_eq!(query.get("limit"), Some(&LIMIT.to_string()));
        assert_eq!(query.get("offset"), Some(&"300".to_string()));
    }

    #[test]
    fn build_user_list_url_keeps_username_in_a_single_encoded_path_segment() {
        let username = "../evil?admin=true";
        let url =
            build_user_list_url(username, MyAnimeListListType::Manga, 0).expect("url should build");
        let parsed = reqwest::Url::parse(&url).expect("built url should parse");
        let segments = parsed
            .path_segments()
            .expect("path segments should exist")
            .collect::<Vec<_>>();

        assert_eq!(parsed.domain(), Some("api.myanimelist.net"));
        assert_eq!(
            segments,
            vec!["v2", "users", "..%2Fevil%3Fadmin=true", "mangalist"]
        );
    }

    #[test]
    fn parse_next_offset_extracts_numeric_offsets_only() {
        assert_eq!(
            parse_next_offset("https://api.myanimelist.net/v2/users/@me/animelist?offset=2000"),
            Some(2000)
        );
        assert_eq!(
            parse_next_offset("https://api.myanimelist.net/v2/users/@me/animelist?offset=abc"),
            None
        );
        assert_eq!(parse_next_offset("javascript:alert(1)"), None);
        assert_eq!(
            parse_next_offset("https://api.myanimelist.net/v2/users/@me/animelist"),
            None
        );
    }
}
