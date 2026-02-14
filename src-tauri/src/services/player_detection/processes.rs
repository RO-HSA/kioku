use serde::Deserialize;
#[cfg(any(windows, target_os = "macos"))]
use std::process::Command;

use super::util::split_command_line;

#[derive(Debug, Clone)]
pub(crate) struct ProcessSnapshot {
    pub pid: u32,
    pub name: String,
    pub command_line: String,
    pub args: Vec<String>,
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
pub(crate) fn list_processes() -> Result<Vec<ProcessSnapshot>, String> {
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
        .map(|process| {
            let command_line = process.command_line.unwrap_or_default();
            let args = split_command_line(&command_line);

            ProcessSnapshot {
                pid: process.process_id,
                name: process.name,
                command_line,
                args,
            }
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

#[cfg(target_os = "linux")]
pub(crate) fn list_processes() -> Result<Vec<ProcessSnapshot>, String> {
    let entries = std::fs::read_dir("/proc")
        .map_err(|error| format!("Failed to list /proc entries: {error}"))?;

    let mut processes = Vec::new();

    for entry in entries {
        let Ok(entry) = entry else {
            continue;
        };

        let file_name = entry.file_name();
        let pid_raw = file_name.to_string_lossy();
        let Ok(pid) = pid_raw.parse::<u32>() else {
            continue;
        };

        let name = match std::fs::read_to_string(format!("/proc/{pid}/comm")) {
            Ok(value) => value.trim().to_string(),
            Err(_) => continue,
        };

        let args = match std::fs::read(format!("/proc/{pid}/cmdline")) {
            Ok(bytes) => parse_linux_cmdline(&bytes),
            Err(_) => continue,
        };

        let command_line = if args.is_empty() {
            name.clone()
        } else {
            args.join(" ")
        };

        processes.push(ProcessSnapshot {
            pid,
            name,
            command_line,
            args,
        });
    }

    Ok(processes)
}

#[cfg(target_os = "linux")]
fn parse_linux_cmdline(bytes: &[u8]) -> Vec<String> {
    bytes
        .split(|byte| *byte == 0)
        .filter(|chunk| !chunk.is_empty())
        .map(|chunk| String::from_utf8_lossy(chunk).to_string())
        .collect()
}

#[cfg(target_os = "macos")]
pub(crate) fn list_processes() -> Result<Vec<ProcessSnapshot>, String> {
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
        let args = split_command_line(&command_line);

        processes.push(ProcessSnapshot {
            pid,
            name,
            command_line,
            args,
        });
    }

    Ok(processes)
}

#[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
pub(crate) fn list_processes() -> Result<Vec<ProcessSnapshot>, String> {
    Ok(Vec::new())
}
