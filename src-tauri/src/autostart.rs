#[cfg(target_os = "linux")]
use std::env::current_exe;
#[cfg(target_os = "linux")]
use std::env::{self, split_paths};
#[cfg(target_os = "linux")]
use std::fs::OpenOptions;
#[cfg(target_os = "linux")]
use std::io::Write;
#[cfg(target_os = "linux")]
use std::path::{Path, PathBuf};
#[cfg(target_os = "linux")]
use std::sync::Arc;

use tauri::{AppHandle, Runtime};
#[cfg(not(target_os = "linux"))]
use tauri_plugin_autostart::ManagerExt;

#[tauri::command]
pub fn is_auto_start_enabled(app: AppHandle) -> Result<bool, String> {
    is_auto_start_enabled_impl(&app)
}

#[tauri::command]
pub fn set_auto_start_enabled(enabled: bool, app: AppHandle) -> Result<bool, String> {
    set_auto_start_enabled_impl(&app, enabled)?;
    is_auto_start_enabled_impl(&app)
}

pub fn sync_auto_start<R: Runtime>(app: &AppHandle<R>, enabled: bool) -> Result<(), String> {
    set_auto_start_enabled_impl(app, enabled)
}

fn is_auto_start_enabled_impl<R: Runtime>(app: &AppHandle<R>) -> Result<bool, String> {
    #[cfg(target_os = "linux")]
    {
        let entry_path = linux_autostart_entry_path(app)?;
        if !entry_path.exists() {
            return Ok(false);
        }

        is_linux_autostart_entry_enabled(&entry_path)
    }

    #[cfg(not(target_os = "linux"))]
    {
        app.autolaunch().is_enabled().map_err(|err| err.to_string())
    }
}

fn set_auto_start_enabled_impl<R: Runtime>(
    app: &AppHandle<R>,
    enabled: bool,
) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        if enabled {
            write_linux_autostart_entry(app)
        } else {
            remove_linux_autostart_entry(app)
        }
    }

    #[cfg(not(target_os = "linux"))]
    {
        let autostart = app.autolaunch();
        if enabled {
            autostart.enable().map_err(|err| err.to_string())
        } else {
            autostart.disable().map_err(|err| err.to_string())
        }
    }
}

#[cfg(target_os = "linux")]
fn write_linux_autostart_entry<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    let entry_path = linux_autostart_entry_path(app)?;
    if let Some(parent) = entry_path.parent() {
        std::fs::create_dir_all(parent).map_err(|err| err.to_string())?;
    }

    let exec = desktop_exec_argument(
        &resolve_launch_path(app)?
            .into_os_string()
            .to_string_lossy()
            .into_owned(),
    );
    let app_name = app.package_info().name.clone();
    let entry = render_linux_desktop_entry(&app_name, &exec);

    let mut options = OpenOptions::new();
    options.write(true).create(true).truncate(true);

    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o644);
    }

    let mut file = options.open(&entry_path).map_err(|err| err.to_string())?;
    file.write_all(entry.as_bytes())
        .map_err(|err| err.to_string())
}

#[cfg(target_os = "linux")]
#[derive(Debug, Default, PartialEq, Eq)]
struct LinuxAutostartEntry {
    hidden: bool,
    only_show_in: Option<Vec<String>>,
    not_show_in: Option<Vec<String>>,
    try_exec: Option<String>,
    x_gnome_autostart_enabled: Option<bool>,
}

#[cfg(target_os = "linux")]
fn remove_linux_autostart_entry<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    let entry_path = linux_autostart_entry_path(app)?;
    if !entry_path.exists() {
        return Ok(());
    }

    std::fs::remove_file(entry_path).map_err(|err| err.to_string())
}

#[cfg(target_os = "linux")]
fn is_linux_autostart_entry_enabled(path: &Path) -> Result<bool, String> {
    let contents = std::fs::read_to_string(path).map_err(|err| err.to_string())?;
    let entry = parse_linux_autostart_entry(&contents);
    let current_desktops = current_desktop_names();
    let try_exec_exists = Arc::new(|command: &str| command_exists(command));

    Ok(evaluate_linux_autostart_entry(
        &entry,
        &current_desktops,
        &try_exec_exists,
    ))
}

#[cfg(target_os = "linux")]
fn parse_linux_autostart_entry(contents: &str) -> LinuxAutostartEntry {
    let mut entry = LinuxAutostartEntry::default();
    let mut in_desktop_entry_group = false;

    for raw_line in contents.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            in_desktop_entry_group = line == "[Desktop Entry]";
            continue;
        }

        if !in_desktop_entry_group {
            continue;
        }

        let Some((key, value)) = line.split_once('=') else {
            continue;
        };

        let key = key.trim();
        let value = value.trim();

        match key {
            "Hidden" => {
                if let Some(parsed) = parse_desktop_entry_bool(value) {
                    entry.hidden = parsed;
                }
            }
            "OnlyShowIn" => {
                entry.only_show_in = parse_desktop_entry_list(value);
            }
            "NotShowIn" => {
                entry.not_show_in = parse_desktop_entry_list(value);
            }
            "TryExec" => {
                if !value.is_empty() {
                    entry.try_exec = Some(value.to_string());
                }
            }
            "X-GNOME-Autostart-enabled" => {
                entry.x_gnome_autostart_enabled = parse_desktop_entry_bool(value);
            }
            _ => {}
        }
    }

    entry
}

#[cfg(target_os = "linux")]
fn evaluate_linux_autostart_entry(
    entry: &LinuxAutostartEntry,
    current_desktops: &[String],
    try_exec_exists: &Arc<dyn Fn(&str) -> bool + Send + Sync>,
) -> bool {
    if entry.hidden {
        return false;
    }

    if matches!(entry.x_gnome_autostart_enabled, Some(false))
        && current_desktops
            .iter()
            .any(|desktop| desktop.eq_ignore_ascii_case("gnome"))
    {
        return false;
    }

    if let Some(only_show_in) = &entry.only_show_in {
        if current_desktops.is_empty()
            || !desktop_list_contains_current_desktop(only_show_in, current_desktops)
        {
            return false;
        }
    }

    if let Some(not_show_in) = &entry.not_show_in {
        if desktop_list_contains_current_desktop(not_show_in, current_desktops) {
            return false;
        }
    }

    if let Some(try_exec) = entry.try_exec.as_deref() {
        if !try_exec_exists(try_exec) {
            return false;
        }
    }

    true
}

#[cfg(target_os = "linux")]
fn parse_desktop_entry_bool(value: &str) -> Option<bool> {
    if value.eq_ignore_ascii_case("true") {
        Some(true)
    } else if value.eq_ignore_ascii_case("false") {
        Some(false)
    } else {
        None
    }
}

#[cfg(target_os = "linux")]
fn parse_desktop_entry_list(value: &str) -> Option<Vec<String>> {
    let values: Vec<String> = value
        .split(';')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(|item| item.to_string())
        .collect();

    if values.is_empty() {
        None
    } else {
        Some(values)
    }
}

#[cfg(target_os = "linux")]
fn current_desktop_names() -> Vec<String> {
    [
        "XDG_CURRENT_DESKTOP",
        "XDG_SESSION_DESKTOP",
        "DESKTOP_SESSION",
    ]
    .into_iter()
    .filter_map(|key| env::var(key).ok())
    .flat_map(|value| {
        value
            .split(':')
            .map(str::trim)
            .filter(|item| !item.is_empty())
            .map(|item| item.to_string())
            .collect::<Vec<_>>()
    })
    .collect()
}

#[cfg(target_os = "linux")]
fn desktop_list_contains_current_desktop(
    configured_desktops: &[String],
    current_desktops: &[String],
) -> bool {
    configured_desktops.iter().any(|configured| {
        current_desktops
            .iter()
            .any(|current| configured.eq_ignore_ascii_case(current))
    })
}

#[cfg(target_os = "linux")]
fn command_exists(command: &str) -> bool {
    let command_path = Path::new(command);

    if command_path.is_absolute() {
        return is_executable_file(command_path);
    }

    if command_path.components().count() > 1 {
        return false;
    }

    env::var_os("PATH")
        .map(|path| {
            split_paths(&path).any(|directory| is_executable_file(&directory.join(command)))
        })
        .unwrap_or(false)
}

#[cfg(target_os = "linux")]
fn is_executable_file(path: &Path) -> bool {
    let Ok(metadata) = std::fs::metadata(path) else {
        return false;
    };

    if !metadata.is_file() {
        return false;
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        return metadata.permissions().mode() & 0o111 != 0;
    }

    #[allow(unreachable_code)]
    true
}

#[cfg(target_os = "linux")]
fn resolve_launch_path<R: Runtime>(app: &AppHandle<R>) -> Result<PathBuf, String> {
    if let Some(appimage_path) = app.env().appimage.as_ref() {
        return Ok(canonicalize_fallback(appimage_path));
    }

    current_exe()
        .map(|path| canonicalize_fallback(&path))
        .map_err(|err| err.to_string())
}

#[cfg(target_os = "linux")]
fn canonicalize_fallback(path: &Path) -> PathBuf {
    std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}

#[cfg(target_os = "linux")]
fn linux_autostart_entry_path<R: Runtime>(app: &AppHandle<R>) -> Result<PathBuf, String> {
    let config_dir = app.path().config_dir().map_err(|err| err.to_string())?;
    Ok(config_dir
        .join("autostart")
        .join(format!("{}.desktop", app.package_info().name)))
}

#[cfg(target_os = "linux")]
fn render_linux_desktop_entry(app_name: &str, exec: &str) -> String {
    format!(
        "[Desktop Entry]\n\
Type=Application\n\
Version=1.0\n\
Name={app_name}\n\
Comment=Start {app_name} automatically on login\n\
Exec={exec}\n\
Terminal=false\n\
StartupNotify=false\n\
X-GNOME-Autostart-enabled=true\n\
X-KDE-autostart-after=panel\n"
    )
}

#[cfg(target_os = "linux")]
fn desktop_exec_argument(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '"' => escaped.push_str("\\\""),
            '`' => escaped.push_str("\\`"),
            '$' => escaped.push_str("\\$"),
            '\\' => escaped.push_str("\\\\\\\\"),
            '%' => escaped.push_str("%%"),
            _ => escaped.push(character),
        }
    }

    if value.is_empty() || value.chars().any(is_reserved_exec_character) {
        format!("\"{escaped}\"")
    } else {
        escaped
    }
}

#[cfg(target_os = "linux")]
fn is_reserved_exec_character(character: char) -> bool {
    matches!(
        character,
        ' ' | '\t'
            | '\n'
            | '"'
            | '\''
            | '\\'
            | '>'
            | '<'
            | '~'
            | '|'
            | '&'
            | ';'
            | '$'
            | '*'
            | '?'
            | '#'
            | '('
            | ')'
            | '`'
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "linux")]
    #[test]
    fn desktop_exec_argument_quotes_reserved_characters_and_escapes_literals() {
        assert_eq!(
            desktop_exec_argument("/opt/kioku.AppImage"),
            "/opt/kioku.AppImage"
        );
        assert_eq!(
            desktop_exec_argument("/home/user/Kioku AppImage/kioku$beta%.AppImage"),
            "\"/home/user/Kioku AppImage/kioku\\$beta%%.AppImage\""
        );
        assert_eq!(desktop_exec_argument(""), "\"\"");
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn rendered_linux_desktop_entry_includes_kde_and_gnome_hints() {
        let entry = render_linux_desktop_entry("kioku", "\"/opt/kioku.AppImage\"");

        assert!(entry.contains("Exec=\"/opt/kioku.AppImage\""));
        assert!(entry.contains("X-GNOME-Autostart-enabled=true"));
        assert!(entry.contains("X-KDE-autostart-after=panel"));
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn parse_linux_autostart_entry_reads_effective_keys() {
        let entry = parse_linux_autostart_entry(
            "[Desktop Entry]\n\
Hidden=true\n\
OnlyShowIn=GNOME;KDE;\n\
NotShowIn=XFCE;\n\
TryExec=kioku\n\
X-GNOME-Autostart-enabled=false\n",
        );

        assert_eq!(
            entry,
            LinuxAutostartEntry {
                hidden: true,
                only_show_in: Some(vec!["GNOME".to_string(), "KDE".to_string()]),
                not_show_in: Some(vec!["XFCE".to_string()]),
                try_exec: Some("kioku".to_string()),
                x_gnome_autostart_enabled: Some(false),
            }
        );
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn evaluate_linux_autostart_entry_honors_hidden_only_show_in_not_show_in_and_try_exec() {
        let current_desktops = vec!["GNOME".to_string()];
        let try_exec_exists = Arc::new(|command: &str| command == "kioku");

        assert!(!evaluate_linux_autostart_entry(
            &LinuxAutostartEntry {
                hidden: true,
                ..Default::default()
            },
            &current_desktops,
            &try_exec_exists,
        ));

        assert!(!evaluate_linux_autostart_entry(
            &LinuxAutostartEntry {
                only_show_in: Some(vec!["KDE".to_string()]),
                ..Default::default()
            },
            &current_desktops,
            &try_exec_exists,
        ));

        assert!(!evaluate_linux_autostart_entry(
            &LinuxAutostartEntry {
                not_show_in: Some(vec!["GNOME".to_string()]),
                ..Default::default()
            },
            &current_desktops,
            &try_exec_exists,
        ));

        assert!(!evaluate_linux_autostart_entry(
            &LinuxAutostartEntry {
                try_exec: Some("missing".to_string()),
                ..Default::default()
            },
            &current_desktops,
            &try_exec_exists,
        ));

        assert!(evaluate_linux_autostart_entry(
            &LinuxAutostartEntry {
                only_show_in: Some(vec!["GNOME".to_string()]),
                try_exec: Some("kioku".to_string()),
                ..Default::default()
            },
            &current_desktops,
            &try_exec_exists,
        ));
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn evaluate_linux_autostart_entry_treats_gnome_flag_as_gnome_specific() {
        let try_exec_exists = Arc::new(|_: &str| true);

        assert!(!evaluate_linux_autostart_entry(
            &LinuxAutostartEntry {
                x_gnome_autostart_enabled: Some(false),
                ..Default::default()
            },
            &["GNOME".to_string()],
            &try_exec_exists,
        ));

        assert!(evaluate_linux_autostart_entry(
            &LinuxAutostartEntry {
                x_gnome_autostart_enabled: Some(false),
                ..Default::default()
            },
            &["KDE".to_string()],
            &try_exec_exists,
        ));
    }
}
