#[cfg(target_os = "linux")]
use std::env::current_exe;
#[cfg(target_os = "linux")]
use std::fs::OpenOptions;
#[cfg(target_os = "linux")]
use std::io::Write;
#[cfg(target_os = "linux")]
use std::path::{Path, PathBuf};

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
        Ok(linux_autostart_entry_path(app)?.exists())
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
fn remove_linux_autostart_entry<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    let entry_path = linux_autostart_entry_path(app)?;
    if !entry_path.exists() {
        return Ok(());
    }

    std::fs::remove_file(entry_path).map_err(|err| err.to_string())
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
    let home_dir = app.path().home_dir().map_err(|err| err.to_string())?;
    Ok(home_dir
        .join(".config")
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
}
