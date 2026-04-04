use serde::{Deserialize, Serialize};

use super::sanitize::{normalize_optional_string, normalize_public_url};

const MAX_DETAILS_LEN: usize = 128;
const MAX_STATE_LEN: usize = 128;
const MAX_ASSET_TEXT_LEN: usize = 128;
const MAX_ASSET_URL_LEN: usize = 512;
const MAX_BUTTON_LABEL_LEN: usize = 32;
const MAX_BUTTON_URL_LEN: usize = 512;
const MAX_BUTTONS: usize = 2;
const MAX_ACTIVITY_TYPE: u8 = 5;
const MAX_STATUS_DISPLAY_TYPE: u8 = 2;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscordPresenceButton {
    pub label: String,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DiscordPresenceRequest {
    pub details: Option<String>,
    pub state: Option<String>,
    pub r#type: Option<u8>,
    pub status_display_type: Option<u8>,
    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub large_url: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
    pub start_timestamp: Option<i64>,
    pub end_timestamp: Option<i64>,
    pub buttons: Option<Vec<DiscordPresenceButton>>,
}

impl DiscordPresenceRequest {
    pub(crate) fn sanitize(self) -> Self {
        Self {
            details: normalize_optional_string(self.details, MAX_DETAILS_LEN),
            state: normalize_optional_string(self.state, MAX_STATE_LEN),
            r#type: sanitize_activity_type(self.r#type),
            status_display_type: sanitize_status_display_type(self.status_display_type),
            large_image: normalize_optional_string(self.large_image, MAX_DETAILS_LEN),
            large_text: normalize_optional_string(self.large_text, MAX_ASSET_TEXT_LEN),
            large_url: normalize_public_url(self.large_url, MAX_ASSET_URL_LEN),
            small_image: normalize_optional_string(self.small_image, MAX_DETAILS_LEN),
            small_text: normalize_optional_string(self.small_text, MAX_ASSET_TEXT_LEN),
            start_timestamp: self.start_timestamp,
            end_timestamp: self.end_timestamp,
            buttons: sanitize_buttons(self.buttons),
        }
    }

    pub(crate) fn into_activity(self) -> Option<DiscordActivity> {
        let assets = if self.large_image.is_some()
            || self.large_text.is_some()
            || self.large_url.is_some()
            || self.small_image.is_some()
            || self.small_text.is_some()
        {
            Some(DiscordAssets {
                large_image: self.large_image,
                large_text: self.large_text,
                large_url: self.large_url,
                small_image: self.small_image,
                small_text: self.small_text,
            })
        } else {
            None
        };

        let timestamps = if self.start_timestamp.is_some() || self.end_timestamp.is_some() {
            Some(DiscordTimestamps {
                start: self.start_timestamp,
                end: self.end_timestamp,
            })
        } else {
            None
        };

        if self.details.is_none()
            && self.state.is_none()
            && self.r#type.is_none()
            && self.status_display_type.is_none()
            && assets.is_none()
            && timestamps.is_none()
            && self.buttons.is_none()
        {
            return None;
        }

        Some(DiscordActivity {
            details: self.details,
            state: self.state,
            r#type: self.r#type,
            status_display_type: self.status_display_type,
            assets,
            timestamps,
            buttons: self.buttons,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct DiscordActivity {
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    r#type: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_display_type: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assets: Option<DiscordAssets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamps: Option<DiscordTimestamps>,
    #[serde(skip_serializing_if = "Option::is_none")]
    buttons: Option<Vec<DiscordPresenceButton>>,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct DiscordAssets {
    #[serde(skip_serializing_if = "Option::is_none")]
    large_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    large_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    large_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    small_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    small_text: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct DiscordTimestamps {
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<i64>,
}

fn sanitize_buttons(
    buttons: Option<Vec<DiscordPresenceButton>>,
) -> Option<Vec<DiscordPresenceButton>> {
    buttons
        .map(|items| {
            items
                .into_iter()
                .filter_map(|item| {
                    let label = normalize_optional_string(Some(item.label), MAX_BUTTON_LABEL_LEN)?;
                    let url = normalize_public_url(Some(item.url), MAX_BUTTON_URL_LEN)?;
                    Some(DiscordPresenceButton { label, url })
                })
                .take(MAX_BUTTONS)
                .collect::<Vec<_>>()
        })
        .filter(|items| !items.is_empty())
}

fn sanitize_activity_type(value: Option<u8>) -> Option<u8> {
    value.filter(|item| *item <= MAX_ACTIVITY_TYPE)
}

fn sanitize_status_display_type(value: Option<u8>) -> Option<u8> {
    value.filter(|item| *item <= MAX_STATUS_DISPLAY_TYPE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_discards_invalid_values_and_limits_buttons() {
        let sanitized = DiscordPresenceRequest {
            details: Some("  Watching Frieren  ".to_string()),
            state: Some("   ".to_string()),
            r#type: Some(MAX_ACTIVITY_TYPE + 1),
            status_display_type: Some(MAX_STATUS_DISPLAY_TYPE + 1),
            large_url: Some("javascript:alert(1)".to_string()),
            buttons: Some(vec![
                DiscordPresenceButton {
                    label: "   ".to_string(),
                    url: "https://example.com/ignored".to_string(),
                },
                DiscordPresenceButton {
                    label: "Unsafe".to_string(),
                    url: "file:///etc/passwd".to_string(),
                },
                DiscordPresenceButton {
                    label: "Open AniList".to_string(),
                    url: "https://anilist.co/anime/1".to_string(),
                },
                DiscordPresenceButton {
                    label: "Open MAL".to_string(),
                    url: "https://myanimelist.net/anime/1".to_string(),
                },
                DiscordPresenceButton {
                    label: "Overflow".to_string(),
                    url: "https://example.com/overflow".to_string(),
                },
            ]),
            ..Default::default()
        }
        .sanitize();

        assert_eq!(sanitized.details.as_deref(), Some("Watching Frieren"));
        assert_eq!(sanitized.state, None);
        assert_eq!(sanitized.r#type, None);
        assert_eq!(sanitized.status_display_type, None);
        assert_eq!(sanitized.large_url, None);

        let buttons = sanitized.buttons.expect("buttons should remain");
        assert_eq!(buttons.len(), 2);
        assert_eq!(buttons[0].label, "Open AniList");
        assert_eq!(buttons[1].label, "Open MAL");
    }

    #[test]
    fn into_activity_returns_none_when_every_field_is_removed() {
        let activity = DiscordPresenceRequest {
            details: Some("   ".to_string()),
            large_url: Some("javascript:alert(1)".to_string()),
            buttons: Some(vec![DiscordPresenceButton {
                label: "Open".to_string(),
                url: "file:///secret".to_string(),
            }]),
            ..Default::default()
        }
        .sanitize()
        .into_activity();

        assert!(activity.is_none());
    }
}
