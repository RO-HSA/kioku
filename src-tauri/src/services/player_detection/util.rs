use std::collections::HashSet;

use super::types::SupportedPlayer;

pub(crate) const DEFAULT_OBSERVER_POLL_INTERVAL_MS: u64 = 2_000;
pub(crate) const MIN_OBSERVER_POLL_INTERVAL_MS: u64 = 500;
pub(crate) const MAX_OBSERVER_POLL_INTERVAL_MS: u64 = 60_000;

pub(crate) fn dedup_players(players: Vec<SupportedPlayer>) -> Vec<SupportedPlayer> {
    let mut seen = HashSet::new();
    players
        .into_iter()
        .filter(|player| seen.insert(*player))
        .collect()
}

pub(crate) fn normalize_poll_interval_ms(value: u64) -> u64 {
    value.clamp(MIN_OBSERVER_POLL_INTERVAL_MS, MAX_OBSERVER_POLL_INTERVAL_MS)
}

pub(crate) fn normalize_process_name(value: &str) -> String {
    value
        .trim_matches('"')
        .trim()
        .rsplit(['\\', '/'])
        .next()
        .unwrap_or(value)
        .to_ascii_lowercase()
}

pub(crate) fn split_command_line(value: &str) -> Vec<String> {
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
