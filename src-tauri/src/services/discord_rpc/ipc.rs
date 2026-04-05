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

fn connect_over_endpoints<T, Connect, Handshake>(
    client_id: &str,
    endpoints: Vec<String>,
    mut connect: Connect,
    mut send_handshake: Handshake,
) -> Result<T, String>
where
    Connect: FnMut(&str) -> Result<T, String>,
    Handshake: FnMut(&mut T, &str) -> Result<(), String>,
{
    if endpoints.is_empty() {
        return Err("Current platform does not support Discord IPC".to_string());
    }

    let mut last_error: Option<String> = None;
    for endpoint in endpoints {
        let mut connection = match connect(&endpoint) {
            Ok(connection) => connection,
            Err(error) => {
                last_error = Some(format!("{endpoint}: {error}"));
                continue;
            }
        };

        if let Err(error) = send_handshake(&mut connection, client_id) {
            last_error = Some(format!("{endpoint}: {error}"));
            continue;
        }

        return Ok(connection);
    }

    Err(last_error.unwrap_or_else(|| "Unable to connect to Discord IPC".to_string()))
}

fn encode_packet_bytes<T: Serialize>(opcode: i32, payload: &T) -> Result<Vec<u8>, String> {
    let body = serde_json::to_vec(payload)
        .map_err(|error| format!("Failed to encode Discord IPC payload: {error}"))?;
    let payload_len =
        i32::try_from(body.len()).map_err(|_| "Discord IPC payload is too large".to_string())?;

    let mut packet = Vec::with_capacity(8 + body.len());
    packet.extend_from_slice(&opcode.to_le_bytes());
    packet.extend_from_slice(&payload_len.to_le_bytes());
    packet.extend_from_slice(&body);
    Ok(packet)
}

fn write_packet<W: Write, T: Serialize>(
    writer: &mut W,
    opcode: i32,
    payload: &T,
) -> Result<(), String> {
    let packet = encode_packet_bytes(opcode, payload)?;

    writer
        .write_all(&packet)
        .map_err(|error| format!("Failed to write Discord IPC payload: {error}"))?;
    writer
        .flush()
        .map_err(|error| format!("Failed to flush Discord IPC stream: {error}"))?;

    Ok(())
}

fn build_activity_command(
    pid: u32,
    activity: Option<DiscordActivity>,
    nonce: String,
) -> DiscordIpcCommand {
    DiscordIpcCommand {
        cmd: DISCORD_COMMAND_SET_ACTIVITY,
        args: DiscordIpcCommandArgs { pid, activity },
        nonce,
    }
}

fn build_handshake_payload(client_id: &str) -> serde_json::Value {
    serde_json::json!({
        "v": DISCORD_HANDSHAKE_VERSION,
        "client_id": client_id
    })
}

impl DiscordIpcConnection {
    pub(crate) fn connect(client_id: &str) -> Result<Self, String> {
        connect_over_endpoints(
            client_id,
            discord_ipc_endpoints(),
            |endpoint| {
                DiscordIpcStream::connect(endpoint)
                    .map(|stream| Self { stream })
                    .map_err(|error| error.to_string())
            },
            |connection, client_id| connection.send_handshake(client_id),
        )
    }

    pub(crate) fn send_activity(
        &mut self,
        activity: Option<DiscordActivity>,
    ) -> Result<(), String> {
        let command = build_activity_command(std::process::id(), activity, build_nonce());

        self.send_packet(DISCORD_IPC_OPCODE_FRAME, &command)
    }

    fn send_handshake(&mut self, client_id: &str) -> Result<(), String> {
        let payload = build_handshake_payload(client_id);

        self.send_packet(DISCORD_IPC_OPCODE_HANDSHAKE, &payload)
    }

    fn send_packet<T: Serialize>(&mut self, opcode: i32, payload: &T) -> Result<(), String> {
        write_packet(&mut self.stream, opcode, payload)
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

#[cfg(test)]
mod tests {
    use super::super::model::DiscordPresenceRequest;
    use super::*;
    use std::io::{Error, ErrorKind};

    #[derive(Default)]
    struct RecordingWriter {
        bytes: Vec<u8>,
        flushed: bool,
        fail_on_write: bool,
        fail_on_flush: bool,
    }

    impl Write for RecordingWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            if self.fail_on_write {
                return Err(Error::new(ErrorKind::BrokenPipe, "write failed"));
            }

            self.bytes.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            if self.fail_on_flush {
                return Err(Error::new(ErrorKind::Other, "flush failed"));
            }

            self.flushed = true;
            Ok(())
        }
    }

    #[test]
    fn encode_packet_bytes_writes_opcode_length_and_json_body() {
        let packet = encode_packet_bytes(
            DISCORD_IPC_OPCODE_FRAME,
            &serde_json::json!({
                "cmd": "SET_ACTIVITY"
            }),
        )
        .expect("packet should encode");

        let opcode = i32::from_le_bytes(packet[0..4].try_into().expect("opcode bytes"));
        let length = i32::from_le_bytes(packet[4..8].try_into().expect("length bytes"));
        let body = std::str::from_utf8(&packet[8..]).expect("body should be utf-8");

        assert_eq!(opcode, DISCORD_IPC_OPCODE_FRAME);
        assert_eq!(length as usize, packet.len() - 8);
        assert!(body.contains("\"SET_ACTIVITY\""));
    }

    #[test]
    fn write_packet_writes_bytes_and_surfaces_writer_errors() {
        let mut writer = RecordingWriter::default();
        write_packet(
            &mut writer,
            DISCORD_IPC_OPCODE_FRAME,
            &serde_json::json!({ "cmd": "SET_ACTIVITY" }),
        )
        .expect("packet should write");
        assert!(writer.flushed);
        assert!(!writer.bytes.is_empty());

        let mut write_error = RecordingWriter {
            fail_on_write: true,
            ..Default::default()
        };
        assert_eq!(
            write_packet(
                &mut write_error,
                DISCORD_IPC_OPCODE_FRAME,
                &serde_json::json!({ "cmd": "SET_ACTIVITY" }),
            )
            .err()
            .as_deref(),
            Some("Failed to write Discord IPC payload: write failed")
        );

        let mut flush_error = RecordingWriter {
            fail_on_flush: true,
            ..Default::default()
        };
        assert_eq!(
            write_packet(
                &mut flush_error,
                DISCORD_IPC_OPCODE_FRAME,
                &serde_json::json!({ "cmd": "SET_ACTIVITY" }),
            )
            .err()
            .as_deref(),
            Some("Failed to flush Discord IPC stream: flush failed")
        );
    }

    #[test]
    fn discord_ipc_endpoints_returns_expected_windows_pipe_names() {
        let endpoints = discord_ipc_endpoints();

        #[cfg(windows)]
        {
            assert_eq!(endpoints.len(), 10);
            assert_eq!(
                endpoints.first().map(String::as_str),
                Some(r"\\?\pipe\discord-ipc-0")
            );
            assert_eq!(
                endpoints.last().map(String::as_str),
                Some(r"\\?\pipe\discord-ipc-9")
            );
        }
    }

    #[test]
    fn connect_over_endpoints_retries_until_handshake_succeeds() {
        let mut connect_calls = Vec::new();
        let mut handshakes = Vec::new();

        let connection = connect_over_endpoints(
            "123",
            vec![
                "first".to_string(),
                "second".to_string(),
                "third".to_string(),
            ],
            |endpoint| {
                connect_calls.push(endpoint.to_string());
                if endpoint == "first" {
                    Err("connect failed".to_string())
                } else {
                    Ok(endpoint.to_string())
                }
            },
            |connection, client_id| {
                handshakes.push((connection.clone(), client_id.to_string()));
                if connection == "second" {
                    Err("handshake failed".to_string())
                } else {
                    Ok(())
                }
            },
        )
        .expect("third endpoint should succeed");

        assert_eq!(connection, "third".to_string());
        assert_eq!(
            connect_calls,
            vec![
                "first".to_string(),
                "second".to_string(),
                "third".to_string()
            ]
        );
        assert_eq!(
            handshakes,
            vec![
                ("second".to_string(), "123".to_string()),
                ("third".to_string(), "123".to_string())
            ]
        );
    }

    #[test]
    fn connect_over_endpoints_reports_unsupported_and_last_error() {
        assert_eq!(
            connect_over_endpoints("123", Vec::<String>::new(), |_| Ok(()), |_, _| Ok(()))
                .err()
                .as_deref(),
            Some("Current platform does not support Discord IPC")
        );

        assert_eq!(
            connect_over_endpoints(
                "123",
                vec!["first".to_string(), "second".to_string()],
                |endpoint| Ok(endpoint.to_string()),
                |connection, _| Err(format!("{connection} handshake failed")),
            )
            .err()
            .as_deref(),
            Some("second: second handshake failed")
        );
    }

    #[test]
    fn command_builders_produce_expected_shapes() {
        let activity = DiscordPresenceRequest {
            details: Some("Watching Frieren".to_string()),
            ..Default::default()
        }
        .sanitize()
        .into_activity();

        let command = build_activity_command(123, activity, "kioku-test".to_string());
        let command_json = serde_json::to_value(&command).expect("command should serialize");
        let handshake_json = build_handshake_payload("456");

        assert_eq!(command_json["cmd"], "SET_ACTIVITY");
        assert_eq!(command_json["args"]["pid"], 123);
        assert_eq!(
            command_json["args"]["activity"]["details"],
            "Watching Frieren"
        );
        assert_eq!(command_json["nonce"], "kioku-test");
        assert_eq!(handshake_json["v"], DISCORD_HANDSHAKE_VERSION);
        assert_eq!(handshake_json["client_id"], "456");
    }

    #[test]
    fn send_activity_packet_contains_command_and_activity_payload() {
        let activity = DiscordPresenceRequest {
            details: Some("Watching Frieren".to_string()),
            ..Default::default()
        }
        .sanitize()
        .into_activity();

        let packet = encode_packet_bytes(
            DISCORD_IPC_OPCODE_FRAME,
            &DiscordIpcCommand {
                cmd: DISCORD_COMMAND_SET_ACTIVITY,
                args: DiscordIpcCommandArgs { pid: 123, activity },
                nonce: "kioku-test".to_string(),
            },
        )
        .expect("packet should encode");
        let body = std::str::from_utf8(&packet[8..]).expect("body should be utf-8");

        assert!(body.contains("\"cmd\":\"SET_ACTIVITY\""));
        assert!(body.contains("\"details\":\"Watching Frieren\""));
        assert!(body.contains("\"nonce\":\"kioku-test\""));
    }
}
