use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Command;
use std::sync::OnceLock;

const VIDEO_EXTENSIONS: &[&str] = &[
    "mkv", "mp4", "avi", "mov", "wmv", "flv", "webm", "m4v", "ts", "m2ts", "mpg", "mpeg", "ogm",
];
const NUMBER_TOKENS_TO_IGNORE: &[u32] = &[360, 480, 540, 576, 720, 1080, 1440, 2160, 4320];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SupportedPlayer {
    Mpv,
    MpcHc,
    MpcBe,
}

impl SupportedPlayer {
    fn all() -> Vec<Self> {
        vec![Self::Mpv, Self::MpcHc, Self::MpcBe]
    }

    fn process_aliases(self) -> &'static [&'static str] {
        match self {
            Self::Mpv => &["mpv", "mpv.exe", "mpvnet", "mpvnet.exe"],
            Self::MpcHc => &["mpc-hc", "mpc-hc.exe", "mpc-hc64", "mpc-hc64.exe"],
            Self::MpcBe => &["mpc-be", "mpc-be.exe", "mpc-be64", "mpc-be64.exe"],
        }
    }

    fn matches_process_name(self, value: &str) -> bool {
        let normalized = normalize_process_name(value);
        self.process_aliases()
            .iter()
            .any(|alias| normalized == *alias)
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DetectPlayingAnimeRequest {
    pub players: Option<Vec<SupportedPlayer>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AnimePlaybackDetection {
    pub player: SupportedPlayer,
    pub process_id: u32,
    pub source: String,
    pub anime_title: String,
    pub episode: Option<u32>,
}

#[derive(Debug, Clone)]
struct ProcessSnapshot {
    pid: u32,
    name: String,
    command_line: String,
}

#[derive(Debug, Clone)]
struct ParsedAnime {
    anime_title: String,
    episode: Option<u32>,
}

#[tauri::command]
pub async fn detect_playing_anime(
    request: Option<DetectPlayingAnimeRequest>,
) -> Result<Option<AnimePlaybackDetection>, String> {
    let selected_players = resolve_selected_players(request);
    let processes = list_processes()?;

    let mut best: Option<(u8, AnimePlaybackDetection)> = None;

    for process in processes {
        let Some(player) = match_process_to_player(&process, &selected_players) else {
            continue;
        };

        let Some(source) = extract_media_source(player, &process.command_line) else {
            continue;
        };

        let Some(parsed) = parse_anime_from_source(&source) else {
            continue;
        };

        let detection = AnimePlaybackDetection {
            player,
            process_id: process.pid,
            source,
            anime_title: parsed.anime_title.clone(),
            episode: parsed.episode,
        };

        let score = score_detection(&parsed);
        if let Some((best_score, best_detection)) = &best {
            if score < *best_score
                || (score == *best_score && detection.process_id <= best_detection.process_id)
            {
                continue;
            }
        }

        best = Some((score, detection));
    }

    Ok(best.map(|(_, detection)| detection))
}

fn resolve_selected_players(request: Option<DetectPlayingAnimeRequest>) -> Vec<SupportedPlayer> {
    let players = request
        .and_then(|payload| payload.players)
        .filter(|items| !items.is_empty())
        .unwrap_or_else(SupportedPlayer::all);

    let mut unique_players = Vec::new();
    for player in players {
        if !unique_players.contains(&player) {
            unique_players.push(player);
        }
    }

    if unique_players.is_empty() {
        SupportedPlayer::all()
    } else {
        unique_players
    }
}

fn score_detection(parsed: &ParsedAnime) -> u8 {
    let mut score = 0;
    if parsed.episode.is_some() {
        score += 3;
    }

    let title_len = parsed.anime_title.chars().count();
    if title_len >= 6 {
        score += 2;
    } else if title_len >= 3 {
        score += 1;
    }

    score
}

fn match_process_to_player(
    process: &ProcessSnapshot,
    selected_players: &[SupportedPlayer],
) -> Option<SupportedPlayer> {
    let executable = split_command_line(&process.command_line)
        .into_iter()
        .next()
        .unwrap_or_default();

    for player in selected_players {
        if player.matches_process_name(&process.name) || player.matches_process_name(&executable) {
            return Some(*player);
        }
    }

    None
}

fn extract_media_source(player: SupportedPlayer, command_line: &str) -> Option<String> {
    let mut args = split_command_line(command_line);
    if args.is_empty() {
        return None;
    }

    args.remove(0);
    let mut candidate: Option<String> = None;

    for arg in args {
        let value = arg.trim();
        if value.is_empty() {
            continue;
        }

        if is_player_option(player, value) {
            continue;
        }

        if let Some(source) = normalize_source_arg(value) {
            candidate = Some(source);
        }
    }

    candidate
}

fn is_player_option(player: SupportedPlayer, value: &str) -> bool {
    let normalized = value.to_ascii_lowercase();
    if normalized == "--" || normalized.starts_with('-') {
        return true;
    }

    match player {
        SupportedPlayer::Mpv => false,
        SupportedPlayer::MpcHc | SupportedPlayer::MpcBe => {
            normalized.starts_with('/') && !looks_like_windows_path(value) && !is_url_source(value)
        }
    }
}

fn normalize_source_arg(value: &str) -> Option<String> {
    let trimmed = value.trim_matches('"').trim_matches('\'').trim();
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

fn is_url_source(value: &str) -> bool {
    let normalized = value.to_ascii_lowercase();
    normalized.starts_with("http://")
        || normalized.starts_with("https://")
        || normalized.starts_with("ftp://")
        || normalized.starts_with("rtsp://")
        || normalized.starts_with("file://")
}

fn looks_like_windows_path(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() >= 3
        && bytes[1] == b':'
        && (bytes[2] == b'\\' || bytes[2] == b'/')
        && bytes[0].is_ascii_alphabetic()
}

fn split_command_line(value: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut quote: Option<char> = None;

    for ch in value.chars() {
        if let Some(active) = quote {
            if ch == active {
                quote = None;
            } else {
                current.push(ch);
            }
            continue;
        }

        match ch {
            '"' | '\'' => quote = Some(ch),
            _ if ch.is_whitespace() => {
                if !current.is_empty() {
                    parts.push(current.clone());
                    current.clear();
                }
            }
            _ => current.push(ch),
        }
    }

    if !current.is_empty() {
        parts.push(current);
    }

    parts
}

fn parse_anime_from_source(source: &str) -> Option<ParsedAnime> {
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

fn normalize_process_name(value: &str) -> String {
    value
        .trim_matches('"')
        .trim()
        .rsplit(['\\', '/'])
        .next()
        .unwrap_or(value)
        .to_ascii_lowercase()
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
            r"(?i)\b(?:\d{3,4}p|10bit|8bit|x264|x265|h264|h265|hevc|av1|aac(?:2\.0)?|flac|opus|ddp(?:5\.1)?|blu[- ]?ray|bdrip|webrip|web[- ]?dl|dvdrip|remux|proper|repack|vostfr|raw|sub(?:bed|s)?|multi|dual[- ]?audio)\b",
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

#[cfg(windows)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct WindowsProcess {
    process_id: u32,
    name: String,
    command_line: Option<String>,
}

#[cfg(windows)]
fn list_processes() -> Result<Vec<ProcessSnapshot>, String> {
    let script = r#"
    [Console]::OutputEncoding = [System.Text.Encoding]::UTF8
    $targets = @('mpv.exe','mpvnet.exe','mpc-hc.exe','mpc-hc64.exe','mpc-be.exe','mpc-be64.exe')
    $processes = Get-CimInstance Win32_Process |
    Where-Object { $targets -contains $_.Name.ToLower() } |
    Select-Object ProcessId, Name, CommandLine

    if ($null -eq $processes) {
      '[]'
    } else {
      @($processes) | ConvertTo-Json -Compress
    }
    "#;

    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", script])
        .output()
        .map_err(|error| format!("Failed to list running processes: {error}"))?;

    if !output.status.success() {
        return Err(format!(
            "Failed to list running processes: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.trim().is_empty() {
        return Ok(Vec::new());
    }

    let parsed = parse_windows_process_output(stdout.trim())?;

    Ok(parsed
        .into_iter()
        .map(|process| ProcessSnapshot {
            pid: process.process_id,
            name: process.name,
            command_line: process.command_line.unwrap_or_default(),
        })
        .collect())
}

#[cfg(windows)]
fn parse_windows_process_output(payload: &str) -> Result<Vec<WindowsProcess>, String> {
    let value: serde_json::Value = serde_json::from_str(payload)
        .map_err(|error| format!("Invalid process output JSON: {error}"))?;

    match value {
        serde_json::Value::Null => Ok(Vec::new()),
        serde_json::Value::Array(array) => array
            .into_iter()
            .map(|item| serde_json::from_value(item).map_err(|error| error.to_string()))
            .collect(),
        serde_json::Value::Object(_) => {
            let single: WindowsProcess =
                serde_json::from_value(value).map_err(|error| error.to_string())?;
            Ok(vec![single])
        }
        _ => Err("Unexpected process output format".to_string()),
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn list_processes() -> Result<Vec<ProcessSnapshot>, String> {
    let output = Command::new("ps")
        .args(["-axww", "-o", "pid=,comm=,args="])
        .output()
        .map_err(|error| format!("Failed to list running processes: {error}"))?;

    if !output.status.success() {
        return Err(format!(
            "Failed to list running processes: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut processes = Vec::new();

    for line in stdout.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let mut pid_and_rest = trimmed.splitn(2, char::is_whitespace);
        let Some(pid_raw) = pid_and_rest.next() else {
            continue;
        };
        let Some(rest_raw) = pid_and_rest.next() else {
            continue;
        };

        let pid = match pid_raw.trim().parse::<u32>() {
            Ok(value) => value,
            Err(_) => continue,
        };

        let rest = rest_raw.trim_start();
        let mut name_and_args = rest.splitn(2, char::is_whitespace);
        let name = name_and_args.next().unwrap_or_default().to_string();
        let command_line = name_and_args
            .next()
            .map(str::trim_start)
            .filter(|value| !value.is_empty())
            .unwrap_or(rest)
            .to_string();

        processes.push(ProcessSnapshot {
            pid,
            name,
            command_line,
        });
    }

    Ok(processes)
}

#[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
fn list_processes() -> Result<Vec<ProcessSnapshot>, String> {
    Ok(Vec::new())
}
