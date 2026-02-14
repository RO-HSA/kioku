use std::collections::HashSet;

use super::parser::{
    is_url_source, looks_like_windows_path, normalize_source_arg, parse_anime_from_source,
};
use super::processes::{list_processes, ProcessSnapshot};
use super::types::{AnimePlaybackDetection, DetectPlayingAnimeRequest, SupportedPlayer};
use super::util::{dedup_players, split_command_line};

pub(crate) struct DetectionCandidate {
    pub process_id: u32,
    pub detection: AnimePlaybackDetection,
}

pub(crate) struct DetectionCycleResult {
    pub detections: Vec<DetectionCandidate>,
    pub matched_player_pids: HashSet<u32>,
}

#[tauri::command]
pub async fn detect_playing_anime(
    request: Option<DetectPlayingAnimeRequest>,
) -> Result<Option<AnimePlaybackDetection>, String> {
    let selected_players = resolve_selected_players(request);
    let cycle_result = collect_detection_cycle_result(&selected_players)?;

    let mut best: Option<(u8, u32, AnimePlaybackDetection)> = None;
    for candidate in cycle_result.detections {
        let score = score_detection(&candidate.detection);
        if let Some((best_score, best_process_id, _)) = &best {
            if score < *best_score
                || (score == *best_score && candidate.process_id <= *best_process_id)
            {
                continue;
            }
        }

        best = Some((score, candidate.process_id, candidate.detection));
    }

    Ok(best.map(|(_, _, detection)| detection))
}

pub(crate) fn collect_detection_cycle_result(
    selected_players: &[SupportedPlayer],
) -> Result<DetectionCycleResult, String> {
    let processes = list_processes()?;
    let mut detections = Vec::new();
    let mut matched_player_pids = HashSet::new();

    for process in processes {
        let Some(player) = match_process_to_player(&process, selected_players) else {
            continue;
        };

        matched_player_pids.insert(process.pid);

        let Some(source) = extract_media_source(player, &process.args, &process.command_line)
        else {
            continue;
        };

        let Some(parsed) = parse_anime_from_source(&source) else {
            continue;
        };

        detections.push(DetectionCandidate {
            process_id: process.pid,
            detection: AnimePlaybackDetection {
                player,
                anime_title: parsed.anime_title,
                episode: parsed.episode,
            },
        });
    }

    detections.sort_by_key(|candidate| candidate.process_id);

    Ok(DetectionCycleResult {
        detections,
        matched_player_pids,
    })
}

fn resolve_selected_players(request: Option<DetectPlayingAnimeRequest>) -> Vec<SupportedPlayer> {
    let players = request
        .and_then(|payload| payload.players)
        .filter(|items| !items.is_empty())
        .unwrap_or_else(SupportedPlayer::all);

    let unique_players = dedup_players(players);
    if unique_players.is_empty() {
        SupportedPlayer::all()
    } else {
        unique_players
    }
}

fn score_detection(detection: &AnimePlaybackDetection) -> u8 {
    let mut score = 0;
    if detection.episode.is_some() {
        score += 3;
    }

    let title_len = detection.anime_title.chars().count();
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
    let executable = process.args.first().cloned().unwrap_or_else(|| {
        split_command_line(&process.command_line)
            .into_iter()
            .next()
            .unwrap_or_default()
    });

    for player in selected_players {
        if player.matches_process_name(&process.name)
            || player.matches_process_name(&executable)
            || process
                .args
                .iter()
                .any(|arg| player.matches_process_name(arg))
        {
            return Some(*player);
        }
    }

    None
}

fn extract_media_source(
    player: SupportedPlayer,
    args: &[String],
    command_line: &str,
) -> Option<String> {
    let mut args = if args.is_empty() {
        split_command_line(command_line)
    } else {
        args.to_vec()
    };

    if args.is_empty() {
        return None;
    }

    if player.matches_process_name(&args[0]) {
        args.remove(0);
    }

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
