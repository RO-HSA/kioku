use std::io;
use std::io::Write;

use serde::Serialize;

use super::model::DiscordActivity;
use super::sanitize::build_nonce;

const DISCORD_HANDSHAKE_VERSION: u32 = 1;
const DISCORD_IPC_OPCODE_HANDSHAKE: i32 = 0;
const DISCORD_IPC_OPCODE_FRAME: i32 = 1;
const DISCORD_COMMAND_SET_ACTIVITY: &str = "SET_ACTIVITY";

#[derive(Debug, Serialize)]
struct DiscordIpcCommand {
    cmd: &'static str,
    args: DiscordIpcCommandArgs,
    nonce: String,
}

#[derive(Debug, Serialize)]
struct DiscordIpcCommandArgs {
    pid: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    activity: Option<DiscordActivity>,
}

pub(crate) struct DiscordIpcConnection {
    stream: DiscordIpcStream,
}

impl DiscordIpcConnection {
    pub(crate) fn connect(client_id: &str) -> Result<Self, String> {
        let endpoints = discord_ipc_endpoints();

        if endpoints.is_empty() {
            return Err("Current platform does not support Discord IPC".to_string());
        }

        let mut last_error: Option<String> = None;
        for endpoint in endpoints {
            let stream = match DiscordIpcStream::connect(&endpoint) {
                Ok(stream) => stream,
                Err(error) => {
                    last_error = Some(format!("{endpoint}: {error}"));
                    continue;
                }
            };

            let mut connection = Self { stream };
            if let Err(error) = connection.send_handshake(client_id) {
                last_error = Some(format!("{endpoint}: {error}"));
                continue;
            }

            return Ok(connection);
        }

        Err(last_error.unwrap_or_else(|| "Unable to connect to Discord IPC".to_string()))
    }

    pub(crate) fn send_activity(
        &mut self,
        activity: Option<DiscordActivity>,
    ) -> Result<(), String> {
        let command = DiscordIpcCommand {
            cmd: DISCORD_COMMAND_SET_ACTIVITY,
            args: DiscordIpcCommandArgs {
                pid: std::process::id(),
                activity,
            },
            nonce: build_nonce(),
        };

        self.send_packet(DISCORD_IPC_OPCODE_FRAME, &command)
    }

    fn send_handshake(&mut self, client_id: &str) -> Result<(), String> {
        let payload = serde_json::json!({
            "v": DISCORD_HANDSHAKE_VERSION,
            "client_id": client_id
        });

        self.send_packet(DISCORD_IPC_OPCODE_HANDSHAKE, &payload)
    }

    fn send_packet<T: Serialize>(&mut self, opcode: i32, payload: &T) -> Result<(), String> {
        let body = serde_json::to_vec(payload)
            .map_err(|error| format!("Failed to encode Discord IPC payload: {error}"))?;
        let payload_len = i32::try_from(body.len())
            .map_err(|_| "Discord IPC payload is too large".to_string())?;

        let mut header = [0_u8; 8];
        header[..4].copy_from_slice(&opcode.to_le_bytes());
        header[4..].copy_from_slice(&payload_len.to_le_bytes());

        self.stream
            .write_all(&header)
            .map_err(|error| format!("Failed to write Discord IPC header: {error}"))?;
        self.stream
            .write_all(&body)
            .map_err(|error| format!("Failed to write Discord IPC body: {error}"))?;
        self.stream
            .flush()
            .map_err(|error| format!("Failed to flush Discord IPC stream: {error}"))?;

        Ok(())
    }
}

enum DiscordIpcStream {
    #[cfg(windows)]
    Windows(std::fs::File),
    #[cfg(unix)]
    Unix(std::os::unix::net::UnixStream),
}

impl DiscordIpcStream {
    fn connect(path: &str) -> io::Result<Self> {
        #[cfg(windows)]
        {
            return std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open(path)
                .map(Self::Windows);
        }

        #[cfg(unix)]
        {
            return std::os::unix::net::UnixStream::connect(path).map(Self::Unix);
        }

        #[allow(unreachable_code)]
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "Discord IPC is not supported on this platform",
        ))
    }
}

impl Write for DiscordIpcStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            #[cfg(windows)]
            Self::Windows(stream) => stream.write(buf),
            #[cfg(unix)]
            Self::Unix(stream) => stream.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            #[cfg(windows)]
            Self::Windows(stream) => stream.flush(),
            #[cfg(unix)]
            Self::Unix(stream) => stream.flush(),
        }
    }
}

fn discord_ipc_endpoints() -> Vec<String> {
    #[cfg(windows)]
    {
        return (0..10)
            .map(|index| format!(r"\\?\pipe\discord-ipc-{}", index))
            .collect();
    }

    #[cfg(unix)]
    {
        let mut roots: Vec<String> = ["XDG_RUNTIME_DIR", "TMPDIR", "TMP", "TEMP"]
            .iter()
            .filter_map(|name| std::env::var(name).ok())
            .filter(|value| !value.trim().is_empty())
            .collect();

        roots.push("/tmp".to_string());
        roots.sort();
        roots.dedup();

        return roots
            .into_iter()
            .flat_map(|root| (0..10).map(move |index| format!("{root}/discord-ipc-{index}")))
            .collect();
    }

    #[allow(unreachable_code)]
    Vec::new()
}
