use regex::Regex;
use std::path::Path;
use std::sync::OnceLock;

const VIDEO_EXTENSIONS: &[&str] = &[
    "mkv", "mp4", "avi", "mov", "wmv", "flv", "webm", "m4v", "ts", "m2ts", "mpg", "mpeg", "ogm",
];

// Numeric tokens that commonly represent video resolution, not episode number.
// This is used by fallback numeric parsing to avoid false positives.
// Keep this list updated as new distributions become common.
const NUMBER_TOKENS_TO_IGNORE: &[u32] = &[
    360, 480, 540, 576, 720, 1080, 1440, 2160, 2880, 3840, 4096, 4320, 5120, 7680,
];

#[derive(Debug, Clone)]
pub(crate) struct ParsedAnime {
    pub anime_title: String,
    pub episode: Option<u32>,
}

pub(crate) fn parse_anime_from_source(source: &str) -> Option<ParsedAnime> {
    let raw = extract_source_title(source)?;
    let normalized_title = normalize_title_tokens(&raw);
    if normalized_title.is_empty() {
        return None;
    }

    let (episode, title_without_episode) = extract_episode_from_title(&normalized_title);

    let mut cleaned_title = leading_group_regex()
        .replace_all(&title_without_episode, " ")
        .to_string();
    cleaned_title = bracket_group_regex()
        .replace_all(&cleaned_title, " ")
        .to_string();
    cleaned_title = noise_token_regex()
        .replace_all(&cleaned_title, " ")
        .to_string();
    cleaned_title = hyphen_separator_regex()
        .replace_all(&cleaned_title, " ")
        .to_string();
    cleaned_title = collapse_whitespace_regex()
        .replace_all(cleaned_title.trim(), " ")
        .to_string();

    let title = cleaned_title
        .trim()
        .trim_matches(|ch: char| matches!(ch, '-' | '_' | '.' | ' '))
        .to_string();

    if title.is_empty() {
        return None;
    }

    Some(ParsedAnime {
        anime_title: title,
        episode,
    })
}

pub(crate) fn normalize_source_arg(value: &str) -> Option<String> {
    let trimmed = value.trim().trim_matches('"').trim_matches('\'').trim();
    if trimmed.is_empty() {
        return None;
    }

    if is_url_source(trimmed) {
        return Some(trimmed.to_string());
    }

    let sanitized = trimmed
        .split('?')
        .next()
        .unwrap_or(trimmed)
        .split('#')
        .next()
        .unwrap_or(trimmed);

    let extension = Path::new(sanitized)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_ascii_lowercase());

    if extension
        .as_deref()
        .map(|ext| VIDEO_EXTENSIONS.contains(&ext))
        .unwrap_or(false)
    {
        Some(trimmed.to_string())
    } else {
        None
    }
}

pub(crate) fn is_url_source(value: &str) -> bool {
    let normalized = value.to_ascii_lowercase();
    normalized.starts_with("http://")
        || normalized.starts_with("https://")
        || normalized.starts_with("ftp://")
        || normalized.starts_with("rtsp://")
        || normalized.starts_with("file://")
}

pub(crate) fn looks_like_windows_path(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() >= 3
        && bytes[1] == b':'
        && (bytes[2] == b'\\' || bytes[2] == b'/')
        && bytes[0].is_ascii_alphabetic()
}

fn extract_source_title(source: &str) -> Option<String> {
    let trimmed = source.trim().trim_matches('"').trim_matches('\'').trim();
    if trimmed.is_empty() {
        return None;
    }

    let without_query = trimmed
        .split('?')
        .next()
        .unwrap_or(trimmed)
        .split('#')
        .next()
        .unwrap_or(trimmed);

    let path = without_query.replace('\\', "/");
    let filename = path.rsplit('/').next().unwrap_or(path.as_str());
    let stem = Path::new(filename)
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or(filename);

    let decoded = decode_percent_encoded(stem);
    let normalized = decoded.trim();

    if normalized.is_empty() {
        None
    } else {
        Some(normalized.to_string())
    }
}

fn normalize_title_tokens(value: &str) -> String {
    collapse_whitespace_regex()
        .replace_all(&value.replace(['.', '_'], " "), " ")
        .to_string()
}

fn extract_episode_from_title(title: &str) -> (Option<u32>, String) {
    for regex in [
        episode_sxe_regex(),
        episode_explicit_regex(),
        episode_japanese_regex(),
        episode_dash_regex(),
        episode_bracket_regex(),
    ] {
        if let Some((episode, range)) = match_episode_with_regex(regex, title) {
            let mut remaining = title.to_string();
            remaining.replace_range(range, " ");
            return (Some(episode), remaining);
        }
    }

    if let Some((episode, range)) = fallback_episode_from_numbers(title) {
        let mut remaining = title.to_string();
        remaining.replace_range(range, " ");
        return (Some(episode), remaining);
    }

    (None, title.to_string())
}

fn match_episode_with_regex(regex: &Regex, title: &str) -> Option<(u32, std::ops::Range<usize>)> {
    let captures = regex.captures(title)?;
    let full_match = captures.get(0)?;
    let episode_raw = captures.name("episode")?.as_str();
    let episode = episode_raw.parse::<u32>().ok()?;

    if !is_plausible_episode(episode) {
        return None;
    }

    Some((episode, full_match.range()))
}

fn fallback_episode_from_numbers(title: &str) -> Option<(u32, std::ops::Range<usize>)> {
    let captures: Vec<_> = fallback_numeric_regex().captures_iter(title).collect();
    for capture in captures.into_iter().rev() {
        let Some(full_match) = capture.get(0) else {
            continue;
        };
        let Some(episode_match) = capture.name("episode") else {
            continue;
        };

        if !is_token_boundary(title, full_match.start(), full_match.end()) {
            continue;
        }

        let episode = match episode_match.as_str().parse::<u32>() {
            Ok(value) => value,
            Err(_) => continue,
        };

        if !is_plausible_episode(episode) {
            continue;
        }

        return Some((episode, full_match.range()));
    }

    None
}

fn is_token_boundary(value: &str, start: usize, end: usize) -> bool {
    let prev = value[..start].chars().last();
    let next = value[end..].chars().next();

    if prev.is_some_and(|ch| ch.is_ascii_digit()) || next.is_some_and(|ch| ch.is_ascii_digit()) {
        return false;
    }

    if prev.is_some_and(|ch| ch.is_ascii_alphabetic())
        || next.is_some_and(|ch| ch.is_ascii_alphabetic())
    {
        return false;
    }

    true
}

fn is_plausible_episode(value: u32) -> bool {
    value > 0
        && value <= 5000
        && !NUMBER_TOKENS_TO_IGNORE.contains(&value)
        && !(1900..=2100).contains(&value)
}

fn decode_percent_encoded(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut index = 0;

    while index < bytes.len() {
        if bytes[index] == b'%' && index + 2 < bytes.len() {
            let first = hex_to_u8(bytes[index + 1]);
            let second = hex_to_u8(bytes[index + 2]);
            if let (Some(a), Some(b)) = (first, second) {
                out.push((a << 4) | b);
                index += 3;
                continue;
            }
        }

        if bytes[index] == b'+' {
            out.push(b' ');
        } else {
            out.push(bytes[index]);
        }
        index += 1;
    }

    String::from_utf8_lossy(&out).to_string()
}

fn hex_to_u8(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        b'A'..=b'F' => Some(value - b'A' + 10),
        _ => None,
    }
}

fn leading_group_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"^(?:\[[^\]]+\]\s*)+").expect("valid leading group regex"))
}

fn bracket_group_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| {
        Regex::new(r"[\[\(\{][^\]\)\}]*[\]\)\}]").expect("valid bracket group regex")
    })
}

fn noise_token_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| {
        Regex::new(
            r"(?i)\b(?:\d{3,4}p(?:[- ]?main(?:8|10))?|main(?:8|10)|10bit|8bit|x264(?:-[a-z0-9]+)?|x265(?:-[a-z0-9]+)?|h264|h265|hevc|av1|aac(?:[- ]?2[- ]?0)?|flac|opus|ddp?(?:[- ]?(?:2[- ]?0|5[- ]?1|7[- ]?1))?|ac[- ]?3|e[- ]?ac[- ]?3|dts(?:[- ]?hd)?|truehd|atmos|blu[- ]?ray|bdrip|webrip|web[- ]?dl|dvdrip|remux|proper|repack|vostfr|raw|sub(?:bed|s)?|multi|dual[- ]?audio)\b",
        )
        .expect("valid noise token regex")
    })
}

fn hyphen_separator_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"[-]{2,}").expect("valid separator regex"))
}

fn collapse_whitespace_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"\s+").expect("valid whitespace regex"))
}

fn episode_sxe_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| {
        Regex::new(r"(?i)\bS\d{1,2}E(?P<episode>\d{1,4})\b").expect("valid SxE regex")
    })
}

fn episode_explicit_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| {
        Regex::new(r"(?i)\bE(?:P|PISODE)?[ ._-]?(?P<episode>\d{1,4})\b")
            .expect("valid explicit episode regex")
    })
}

fn episode_japanese_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| {
        Regex::new(r"(?i)\x{7B2C}\s*(?P<episode>\d{1,4})\s*[\x{8A71}\x{8BDD}]")
            .expect("valid JP episode regex")
    })
}

fn episode_dash_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| {
        Regex::new(r"(?i)\s-\s(?P<episode>\d{1,4})(?:v\d+)?\b").expect("valid dash regex")
    })
}

fn episode_bracket_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| {
        Regex::new(r"(?i)\[(?P<episode>\d{1,4})(?:v\d+)?\]").expect("valid bracket regex")
    })
}

fn fallback_numeric_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX
        .get_or_init(|| Regex::new(r"(?P<episode>\d{1,4})(?:v\d+)?").expect("valid fallback regex"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_percent_encoded_urls_and_strips_release_noise() {
        let parsed = parse_anime_from_source(
            "https://cdn.example.com/%5BSubsPlease%5D%20Sousou_no_Frieren_-_03_%281080p%29_%5BABCD1234%5D.mkv?download=1#fragment",
        )
        .expect("source should parse");

        assert_eq!(parsed.anime_title, "Sousou no Frieren");
        assert_eq!(parsed.episode, Some(3));
    }

    #[test]
    fn parses_japanese_episode_markers() {
        let parsed =
            parse_anime_from_source("[Erai-raws] Kusuriya no Hitorigoto 第12話 [1080p].mkv")
                .expect("source should parse");

        assert_eq!(parsed.anime_title, "Kusuriya no Hitorigoto");
        assert_eq!(parsed.episode, Some(12));
    }

    #[test]
    fn fallback_numeric_episode_ignores_years_and_resolution_tokens() {
        let parsed = parse_anime_from_source("Anime.Name.2024.12.1080p.WEB-DL.mkv")
            .expect("source should parse");

        assert_eq!(parsed.episode, Some(12));
        assert_eq!(parsed.anime_title, "Anime Name 2024");
    }

    #[test]
    fn strips_release_profile_audio_and_group_suffix_from_detected_title() {
        let parsed = parse_anime_from_source(
            "Ghost.in.the.Shell.Stand.Alone.Complex.S01E08.1080p-Main10.BluRay.DD5.1.x265-CTR.mkv",
        )
        .expect("source should parse");

        assert_eq!(parsed.anime_title, "Ghost in the Shell Stand Alone Complex");
        assert_eq!(parsed.episode, Some(8));
    }

    #[test]
    fn returns_none_when_source_only_contains_noise_tokens() {
        assert!(parse_anime_from_source("[SubsPlease] [1080p] [AAC] [1234ABCD].mkv").is_none());
    }

    #[test]
    fn normalize_source_arg_accepts_video_sources_and_rejects_unsafe_or_irrelevant_values() {
        assert_eq!(
            normalize_source_arg("  \"C:\\Anime\\Frieren - 01.MKV\"  "),
            Some("C:\\Anime\\Frieren - 01.MKV".to_string())
        );
        assert_eq!(
            normalize_source_arg("https://example.com/watch?id=1#frag"),
            Some("https://example.com/watch?id=1#frag".to_string())
        );
        assert_eq!(normalize_source_arg("/fullscreen"), None);
        assert_eq!(normalize_source_arg("javascript:alert(1)"), None);
        assert_eq!(normalize_source_arg("C:\\Anime\\readme.txt"), None);
    }

    #[test]
    fn url_and_windows_path_detection_distinguish_sources_from_switches() {
        assert!(is_url_source("HTTPS://example.com/video.mkv"));
        assert!(!is_url_source("javascript:alert(1)"));
        assert!(looks_like_windows_path(r"C:\Anime\Frieren - 01.mkv"));
        assert!(!looks_like_windows_path("/play"));
    }
}
