use tauri_plugin_http::reqwest::Url;

pub(crate) fn normalize_client_id(client_id: Option<String>) -> Option<String> {
    client_id.and_then(|value| {
        let normalized = value.trim();
        if normalized.is_empty() || !normalized.bytes().all(|byte| byte.is_ascii_digit()) {
            None
        } else {
            Some(normalized.to_string())
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

pub(crate) fn normalize_public_url(value: Option<String>, max_len: usize) -> Option<String> {
    let normalized = normalize_optional_string(value, max_len)?;
    let parsed = Url::parse(&normalized).ok()?;

    match parsed.scheme() {
        "http" | "https" if parsed.has_host() => Some(parsed.to_string()),
        _ => None,
    }
}

pub(crate) fn build_nonce() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);
    format!("kioku-{}-{}", std::process::id(), nanos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_client_id_trims_numeric_values_and_rejects_invalid_input() {
        assert_eq!(
            normalize_client_id(Some(" 1475090774516568215 ".to_string())),
            Some("1475090774516568215".to_string())
        );
        assert_eq!(normalize_client_id(Some("discord-app".to_string())), None);
        assert_eq!(normalize_client_id(Some("   ".to_string())), None);
    }

    #[test]
    fn normalize_optional_string_trims_and_truncates_without_breaking_utf8() {
        assert_eq!(
            normalize_optional_string(Some("  héllö world  ".to_string()), 4),
            Some("héll".to_string())
        );
        assert_eq!(normalize_optional_string(Some("   ".to_string()), 10), None);
    }

    #[test]
    fn normalize_public_url_allows_only_http_and_https_urls() {
        assert_eq!(
            normalize_public_url(Some(" HTTPS://Example.com/watch?q=1 ".to_string()), 512),
            Some("https://example.com/watch?q=1".to_string())
        );
        assert_eq!(
            normalize_public_url(Some("javascript:alert(1)".to_string()), 512),
            None
        );
        assert_eq!(
            normalize_public_url(Some("file:///C:/Windows/System32".to_string()), 512),
            None
        );
    }

    #[test]
    fn build_nonce_uses_expected_prefix_and_changes_per_call() {
        let first = build_nonce();
        let second = build_nonce();

        assert!(first.starts_with("kioku-"));
        assert!(second.starts_with("kioku-"));
        assert_ne!(first, second);
    }
}
