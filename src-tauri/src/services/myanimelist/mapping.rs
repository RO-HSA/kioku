use super::{
    AnimeListBroadcast, AnimeListItem, MalAlternativeTitles, MalAuthorRole, MalGenre, MalListEntry,
    MalSerialization, MalStartSeason, MalStudio, MangaListItem, MyAnimeListListType,
    UserStatistics, UserStatusKey,
};

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

pub(super) fn map_user_status_to_mal(
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

fn join_names<T, F>(items: Vec<T>, mut f: F) -> String
where
    F: FnMut(T) -> String,
{
    if items.is_empty() {
        return "Unknown".to_string();
    }

    let mut result = String::new();
    for item in items {
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

pub(super) fn map_anime_entry_to_domain(
    entry: MalListEntry,
    status_key: UserStatusKey,
) -> AnimeListItem {
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
    let genres = join_names(node.genres, |genre: MalGenre| genre.name);
    let studios = join_names(node.studios, |studio: MalStudio| studio.name);
    let start_season = format_start_season(node.start_season);
    let broadcast = node.broadcast.unwrap_or(super::MalBroadcast {
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

pub(super) fn map_manga_entry_to_domain(
    entry: MalListEntry,
    status_key: UserStatusKey,
) -> MangaListItem {
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
        genres: join_names(node.genres, |genre: MalGenre| genre.name),
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

pub(super) fn map_mal_statistics(statistics: super::MalAnimeStatistics) -> UserStatistics {
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

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::super::MalAuthor;
    use super::*;

    fn sample_anime_entry() -> MalListEntry {
        serde_json::from_value(json!({
            "node": {
                "id": 1,
                "title": "Frieren",
                "main_picture": {
                    "medium": "https://img.example/medium.jpg",
                    "large": "https://img.example/large.jpg"
                },
                "synopsis": "A journey.",
                "alternative_titles": {
                    "synonyms": [" Beyond Journey's End ", " "],
                    "en": " Frieren: Beyond Journey's End ",
                    "ja": " 葬送のフリーレン "
                },
                "mean": 9.1,
                "source": "light_novel",
                "num_episodes": 28,
                "status": "currently_airing",
                "genres": [{"name": "Adventure"}, {"name": "Fantasy"}],
                "start_season": {"season": "Fall", "year": 2023},
                "broadcast": {"day_of_the_week": "friday", "start_time": "23:00"},
                "start_date": "2023-09-29",
                "media_type": "tv",
                "studios": [{"name": "Madhouse"}],
                "authors": [],
                "serialization": null
            },
            "list_status": {
                "status": "watching",
                "score": 8,
                "num_episodes_watched": 12,
                "is_rewatching": true,
                "comments": "great",
                "num_times_rewatched": 1,
                "updated_at": "2024-01-01T00:00:00+00:00",
                "start_date": "2023-10-01",
                "finish_date": null
            }
        }))
        .expect("sample anime entry should deserialize")
    }

    fn sample_manga_entry() -> MalListEntry {
        serde_json::from_value(json!({
            "node": {
                "id": 2,
                "title": "Vagabond",
                "main_picture": {
                    "medium": "https://img.example/vagabond-medium.jpg"
                },
                "synopsis": null,
                "alternative_titles": {
                    "synonyms": [" Takehiko Inoue "],
                    "en": "",
                    "ja": " バガボンド "
                },
                "mean": 8.7,
                "num_volumes": 37,
                "num_chapters": 327,
                "status": "on_hiatus",
                "genres": [{"name": "Action"}, {"name": ""}],
                "start_date": "1998-09-03",
                "end_date": null,
                "media_type": "manhwa",
                "authors": [
                    {"node": {"first_name": "Takehiko", "last_name": "Inoue"}},
                    {"node": {"first_name": "", "last_name": "Yoshikawa"}}
                ],
                "serialization": {"name": " Morning "}
            },
            "list_status": {
                "status": "reading",
                "score": 10,
                "num_volumes_read": 20,
                "num_chapters_read": 200,
                "is_rereading": false,
                "comments": null,
                "num_times_reread": 4,
                "updated_at": null,
                "start_date": "2020-01-01",
                "finish_date": null
            }
        }))
        .expect("sample manga entry should deserialize")
    }

    #[test]
    fn basic_mapping_helpers_convert_known_values_and_preserve_unknowns() {
        assert_eq!(map_source(Some("light_novel".to_string())), "Light Novel");
        assert_eq!(map_source(Some("webtoon".to_string())), "webtoon");
        assert_eq!(map_source(None), "Unknown");

        assert_eq!(
            map_status(Some("finished_airing".to_string())),
            "Finished Airing"
        );
        assert_eq!(map_status(Some("custom".to_string())), "custom");
        assert_eq!(map_status(None), "Unknown");

        assert_eq!(map_media_type(Some("one_shot".to_string())), "One-shot");
        assert_eq!(map_media_type(Some("custom".to_string())), "custom");
        assert_eq!(map_media_type(None), "Unknown");
    }

    #[test]
    fn user_status_mapping_handles_aliases_and_invalid_values() {
        assert_eq!(
            map_user_status_to_mal(MyAnimeListListType::Anime, "planToWatch"),
            Some("plan_to_watch")
        );
        assert_eq!(
            map_user_status_to_mal(MyAnimeListListType::Manga, "on_hold"),
            Some("on_hold")
        );
        assert_eq!(
            map_user_status_to_mal(MyAnimeListListType::Manga, "watching"),
            None
        );
    }

    #[test]
    fn title_author_and_season_helpers_trim_and_default_correctly() {
        assert_eq!(
            build_alternative_titles(Some(MalAlternativeTitles {
                synonyms: Some(vec![" Alias ".to_string(), "".to_string()]),
                en: Some(" English ".to_string()),
                ja: Some(" ".to_string()),
            })),
            "English, Alias"
        );
        assert_eq!(build_alternative_titles(None), "Unknown");

        assert_eq!(
            format_start_season(Some(MalStartSeason {
                season: Some("Fall".to_string()),
                year: Some(2023),
            })),
            "Fall 2023"
        );
        assert_eq!(
            format_start_season(Some(MalStartSeason {
                season: None,
                year: Some(2023),
            })),
            "Unknown 2023"
        );
        assert_eq!(
            format_start_season(Some(MalStartSeason {
                season: Some("".to_string()),
                year: None,
            })),
            "Unknown"
        );

        assert_eq!(
            build_author_names(vec![
                MalAuthorRole {
                    node: Some(MalAuthor {
                        first_name: Some("Hiromu".to_string()),
                        last_name: Some("Arakawa".to_string()),
                    }),
                },
                MalAuthorRole {
                    node: Some(MalAuthor {
                        first_name: None,
                        last_name: Some("Yoshikawa".to_string()),
                    }),
                },
            ]),
            "Hiromu Arakawa, Yoshikawa"
        );
        assert_eq!(build_author_names(Vec::new()), "Unknown");
        assert_eq!(
            build_serialization_label(Some(MalSerialization {
                name: Some(" Weekly Shonen Jump ".to_string()),
            })),
            "Weekly Shonen Jump"
        );
        assert_eq!(build_serialization_label(None), "Unknown");
    }

    #[test]
    fn map_anime_entry_to_domain_applies_defaults_and_field_mappings() {
        let mapped = map_anime_entry_to_domain(sample_anime_entry(), UserStatusKey::Watching);

        assert_eq!(mapped.id, 1);
        assert_eq!(mapped.title, "Frieren");
        assert_eq!(mapped.image_url, "https://img.example/large.jpg");
        assert_eq!(
            mapped.alternative_titles,
            "Frieren: Beyond Journey's End, 葬送のフリーレン, Beyond Journey's End"
        );
        assert_eq!(mapped.source, "Light Novel");
        assert_eq!(mapped.status, "Currently Airing");
        assert_eq!(mapped.total_episodes, 28);
        assert_eq!(mapped.genres, "Adventure, Fantasy");
        assert_eq!(mapped.start_season, "Fall 2023");
        assert_eq!(mapped.broadcast.day_of_the_week, "friday");
        assert_eq!(mapped.broadcast.start_time, "23:00");
        assert_eq!(mapped.media_type, "TV");
        assert_eq!(mapped.user_status, "watching");
        assert_eq!(mapped.user_score, 8);
        assert_eq!(mapped.user_episodes_watched, 12);
        assert!(mapped.is_rewatching);
        assert_eq!(mapped.user_num_times_rewatched, 1);
    }

    #[test]
    fn map_manga_entry_to_domain_uses_fallbacks_when_values_are_missing() {
        let mapped = map_manga_entry_to_domain(sample_manga_entry(), UserStatusKey::Reading);

        assert_eq!(mapped.id, 2);
        assert_eq!(mapped.image_url, "https://img.example/vagabond-medium.jpg");
        assert_eq!(mapped.synopsis, "No synopsis available.");
        assert_eq!(mapped.alternative_titles, "バガボンド, Takehiko Inoue");
        assert_eq!(mapped.status, "On Hiatus");
        assert_eq!(mapped.total_volumes, 37);
        assert_eq!(mapped.total_chapters, 327);
        assert_eq!(mapped.genres, "Action");
        assert_eq!(mapped.authors, "Takehiko Inoue, Yoshikawa");
        assert_eq!(mapped.serialization, "Morning");
        assert_eq!(mapped.media_type, "Manhwa");
        assert_eq!(mapped.user_status, "reading");
        assert_eq!(mapped.user_volumes_read, 20);
        assert_eq!(mapped.user_chapters_read, 200);
        assert!(!mapped.is_rereading);
        assert_eq!(mapped.user_num_times_reread, 4);
    }

    #[test]
    fn map_mal_statistics_is_a_direct_projection() {
        let statistics = super::super::MalAnimeStatistics {
            num_items_watching: 1,
            num_items_completed: 2,
            num_items_on_hold: 3,
            num_items_dropped: 4,
            num_items_plan_to_watch: 5,
            num_items: 15,
            num_days_watched: 10.5,
            num_days_watching: 1.5,
            num_days_completed: 2.5,
            num_days_on_hold: 3.5,
            num_days_dropped: 4.5,
            num_days: 10.5,
            num_episodes: 99,
            num_times_rewatched: 7,
            mean_score: 8.9,
        };

        let mapped = map_mal_statistics(statistics);

        assert_eq!(mapped.num_items_watching, 1);
        assert_eq!(mapped.num_items_completed, 2);
        assert_eq!(mapped.num_items_on_hold, 3);
        assert_eq!(mapped.num_items_dropped, 4);
        assert_eq!(mapped.num_items_plan_to_watch, 5);
        assert_eq!(mapped.num_items, 15);
        assert_eq!(mapped.num_episodes, 99);
        assert_eq!(mapped.num_times_rewatched, 7);
        assert_eq!(mapped.mean_score, 8.9);
    }
}
