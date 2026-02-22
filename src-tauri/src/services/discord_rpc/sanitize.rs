pub(crate) fn normalize_client_id(client_id: Option<String>) -> Option<String> {
    client_id.and_then(|value| {
        let normalized = value.trim().to_string();
        if normalized.is_empty() {
            None
        } else {
            Some(normalized)
        }
    })
}

pub(crate) fn normalize_optional_string(value: Option<String>, max_len: usize) -> Option<String> {
    value.and_then(|raw| {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return None;
        }

        let clipped = truncate_to_chars(trimmed, max_len);
        if clipped.is_empty() {
            None
        } else {
            Some(clipped)
        }
    })
}

fn truncate_to_chars(value: &str, max_len: usize) -> String {
    value.chars().take(max_len).collect()
}

pub(crate) fn build_nonce() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);
    format!("kioku-{}-{}", std::process::id(), nanos)
}
