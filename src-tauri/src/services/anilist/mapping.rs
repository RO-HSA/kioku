use crate::services::anime_list_updates::ListType;

use super::{
    AniListAnimeStatistics, AniListFuzzyDate, AniListMedia, AniListMediaListEntry, AniListStaff,
    AniListStudios, AniListTitle, AnimeListBroadcast, AnimeListItem, FuzzyDateInput,
    MangaListItem, UserStatistics, UserStatusKey,
};

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

    normalize_text(title.romaji.as_deref())
        .or_else(|| normalize_text(title.english.as_deref()))
        .or_else(|| normalize_text(title.native_title.as_deref()))
        .unwrap_or_else(|| "Unknown".to_string())
}

fn build_alternative_titles(title: Option<&AniListTitle>, primary: &str) -> String {
    let Some(title) = title else {
        return "Unknown".to_string();
    };

    let mut parts: Vec<String> = Vec::new();

    for value in [title.english.as_deref(), title.native_title.as_deref()] {
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

fn map_status(list_type: ListType, status: Option<String>) -> String {
    match status {
        Some(value) => match list_type {
            ListType::Anime => match value.as_str() {
                "FINISHED" => "Finished Airing".to_string(),
                "NOT_YET_RELEASED" => "Not Yet Aired".to_string(),
                "RELEASING" => "Currently Airing".to_string(),
                _ => format_upper_snake(&value),
            },
            ListType::Manga => match value.as_str() {
                "FINISHED" => "Finished".to_string(),
                "NOT_YET_RELEASED" => "Not Yet Published".to_string(),
                "RELEASING" => "Currently Publishing".to_string(),
                "HIATUS" => "On Hiatus".to_string(),
                "CANCELLED" => "Discontinued".to_string(),
                _ => format_upper_snake(&value),
            },
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

pub(super) fn map_user_status_to_anilist(
    list_type: ListType,
    status: &str,
) -> Result<&'static str, String> {
    match list_type {
        ListType::Anime => match status {
            "watching" => Ok("CURRENT"),
            "completed" => Ok("COMPLETED"),
            "onHold" | "on_hold" => Ok("PAUSED"),
            "dropped" => Ok("DROPPED"),
            "planToWatch" | "plan_to_watch" => Ok("PLANNING"),
            _ => Err(format!("Invalid AniList status: {status}")),
        },
        ListType::Manga => match status {
            "reading" => Ok("CURRENT"),
            "completed" => Ok("COMPLETED"),
            "onHold" | "on_hold" => Ok("PAUSED"),
            "dropped" => Ok("DROPPED"),
            "planToRead" | "plan_to_read" => Ok("PLANNING"),
            _ => Err(format!("Invalid AniList status: {status}")),
        },
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

pub(super) fn parse_fuzzy_date_input(
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

fn join_author_names(staff: Option<AniListStaff>) -> String {
    let Some(staff) = staff else {
        return "Unknown".to_string();
    };

    let mut names: Vec<String> = Vec::new();

    for edge in staff.edges {
        let _role = edge.role.as_deref();
        let Some(node) = edge.node else {
            continue;
        };
        let Some(name) = node.name else {
            continue;
        };
        let Some(full_name) = normalize_text(name.full.as_deref()) else {
            continue;
        };

        if !names.iter().any(|existing| existing == &full_name) {
            names.push(full_name);
        }
    }

    if names.is_empty() {
        "Unknown".to_string()
    } else {
        names.join(", ")
    }
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

fn minutes_to_days(minutes: u64) -> f64 {
    minutes as f64 / 1440.0
}

pub(super) fn map_anilist_statistics(statistics: AniListAnimeStatistics) -> UserStatistics {
    let mut num_items_watching = 0_u32;
    let mut num_items_completed = 0_u32;
    let mut num_items_on_hold = 0_u32;
    let mut num_items_dropped = 0_u32;
    let mut num_items_plan_to_watch = 0_u32;

    let mut minutes_watching = 0_u64;
    let mut minutes_completed = 0_u64;
    let mut minutes_on_hold = 0_u64;
    let mut minutes_dropped = 0_u64;

    for status in statistics.statuses {
        let count = status.count.unwrap_or(0);
        let minutes = status.minutes_watched.unwrap_or(0);
        let key = status.status.unwrap_or_default();

        match key.as_str() {
            "CURRENT" | "REPEATING" => {
                num_items_watching = num_items_watching.saturating_add(count);
                minutes_watching = minutes_watching.saturating_add(minutes);
            }
            "COMPLETED" => {
                num_items_completed = num_items_completed.saturating_add(count);
                minutes_completed = minutes_completed.saturating_add(minutes);
            }
            "PAUSED" => {
                num_items_on_hold = num_items_on_hold.saturating_add(count);
                minutes_on_hold = minutes_on_hold.saturating_add(minutes);
            }
            "DROPPED" => {
                num_items_dropped = num_items_dropped.saturating_add(count);
                minutes_dropped = minutes_dropped.saturating_add(minutes);
            }
            "PLANNING" => {
                num_items_plan_to_watch = num_items_plan_to_watch.saturating_add(count);
            }
            _ => {}
        }
    }

    let total_minutes = statistics.minutes_watched.unwrap_or_else(|| {
        minutes_watching
            .saturating_add(minutes_completed)
            .saturating_add(minutes_on_hold)
            .saturating_add(minutes_dropped)
    });

    UserStatistics {
        num_items_watching,
        num_items_completed,
        num_items_on_hold,
        num_items_dropped,
        num_items_plan_to_watch,
        num_items: statistics.count.unwrap_or(
            num_items_watching
                .saturating_add(num_items_completed)
                .saturating_add(num_items_on_hold)
                .saturating_add(num_items_dropped)
                .saturating_add(num_items_plan_to_watch),
        ),
        num_days_watched: minutes_to_days(total_minutes),
        num_days_watching: minutes_to_days(minutes_watching),
        num_days_completed: minutes_to_days(minutes_completed),
        num_days_on_hold: minutes_to_days(minutes_on_hold),
        num_days_dropped: minutes_to_days(minutes_dropped),
        num_days: minutes_to_days(total_minutes),
        num_episodes: statistics.episodes_watched.unwrap_or(0),
        num_times_rewatched: 0,
        mean_score: statistics.mean_score.unwrap_or(0.0),
    }
}

pub(super) fn map_anime_to_domain(
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
        status: map_status(ListType::Anime, media.status),
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

pub(super) fn map_manga_to_domain(
    media: AniListMedia,
    media_list_entry: AniListMediaListEntry,
    status_key: UserStatusKey,
) -> MangaListItem {
    let title = pick_title(media.title.as_ref());
    let alternative_titles = build_alternative_titles(media.title.as_ref(), &title);
    let image_url = media
        .cover_image
        .and_then(|cover| cover.extra_large.or(cover.large))
        .unwrap_or_default();
    let repeat = media_list_entry.repeat.unwrap_or(0);

    MangaListItem {
        id: media.id,
        entry_id: media_list_entry.id,
        title,
        image_url,
        synopsis: media
            .description
            .unwrap_or_else(|| "No synopsis available.".to_string()),
        alternative_titles,
        score: media.mean_score.unwrap_or(0) as f64,
        status: map_status(ListType::Manga, media.status),
        total_volumes: media.volumes.unwrap_or(0),
        total_chapters: media.chapters.unwrap_or(0),
        genres: join_genres(media.genres),
        start_date: format_fuzzy_date(media.start_date),
        end_date: format_fuzzy_date(media.end_date),
        authors: join_author_names(media.staff),
        serialization: "Unknown".to_string(),
        media_type: map_media_type(media.format.or(media.r#type)),
        user_status: status_key.as_user_status_str().to_string(),
        user_score: normalize_score(media_list_entry.score),
        user_volumes_read: media_list_entry.progress_volumes.unwrap_or(0),
        user_chapters_read: media_list_entry.progress.unwrap_or(0),
        is_rereading: repeat > 0,
        user_comments: media_list_entry.notes.unwrap_or_default(),
        user_num_times_reread: repeat,
        user_start_date: format_fuzzy_date(media_list_entry.started_at),
        user_finish_date: format_fuzzy_date(media_list_entry.completed_at),
        updated_at: None,
    }
}
