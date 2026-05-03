use std::io;
use std::io::{Read, Write};
#[cfg(unix)]
use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::model::DiscordActivity;
use super::sanitize::build_nonce;

const DISCORD_HANDSHAKE_VERSION: u32 = 1;
const DISCORD_IPC_OPCODE_HANDSHAKE: i32 = 0;
const DISCORD_IPC_OPCODE_FRAME: i32 = 1;
const DISCORD_IPC_OPCODE_CLOSE: i32 = 2;
const DISCORD_IPC_OPCODE_PING: i32 = 3;
const DISCORD_IPC_OPCODE_PONG: i32 = 4;
const DISCORD_COMMAND_SET_ACTIVITY: &str = "SET_ACTIVITY";
const DISCORD_READY_EVENT: &str = "READY";
const DISCORD_ERROR_EVENT: &str = "ERROR";
#[cfg(unix)]
const DISCORD_IPC_IO_TIMEOUT_SECS: u64 = 2;

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

#[derive(Debug, Deserialize)]
struct DiscordRpcEnvelope {
    #[allow(dead_code)]
    #[serde(default)]
    cmd: Option<String>,
    #[serde(default)]
    evt: Option<String>,
    #[serde(default)]
    nonce: Option<String>,
    #[serde(default)]
    data: Option<Value>,
}

#[derive(Debug)]
struct DiscordIpcPacket {
    opcode: i32,
    payload: Vec<u8>,
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
    encode_raw_packet_bytes(opcode, body)
}

fn encode_raw_packet_bytes(opcode: i32, body: Vec<u8>) -> Result<Vec<u8>, String> {
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

fn write_raw_packet<W: Write>(writer: &mut W, opcode: i32, body: Vec<u8>) -> Result<(), String> {
    let packet = encode_raw_packet_bytes(opcode, body)?;

    writer
        .write_all(&packet)
        .map_err(|error| format!("Failed to write Discord IPC payload: {error}"))?;
    writer
        .flush()
        .map_err(|error| format!("Failed to flush Discord IPC stream: {error}"))?;

    Ok(())
}

fn read_packet<R: Read>(reader: &mut R) -> Result<DiscordIpcPacket, String> {
    let mut header = [0_u8; 8];
    reader
        .read_exact(&mut header)
        .map_err(|error| format!("Failed to read Discord IPC header: {error}"))?;

    let opcode = i32::from_le_bytes(header[..4].try_into().expect("header has opcode bytes"));
    let payload_len = i32::from_le_bytes(header[4..].try_into().expect("header has length bytes"));

    if payload_len < 0 {
        return Err("Discord IPC payload length is invalid".to_string());
    }

    let mut payload = vec![0_u8; payload_len as usize];
    reader
        .read_exact(&mut payload)
        .map_err(|error| format!("Failed to read Discord IPC payload: {error}"))?;

    Ok(DiscordIpcPacket { opcode, payload })
}

fn decode_rpc_envelope(payload: &[u8]) -> Result<DiscordRpcEnvelope, String> {
    serde_json::from_slice(payload)
        .map_err(|error| format!("Failed to decode Discord IPC response: {error}"))
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
            |connection, client_id| connection.establish_session(client_id),
        )
        .map_err(append_linux_flatpak_hint)
    }

    pub(crate) fn send_activity(
        &mut self,
        activity: Option<DiscordActivity>,
    ) -> Result<(), String> {
        let command = build_activity_command(std::process::id(), activity, build_nonce());
        let nonce = command.nonce.clone();
        self.send_packet(DISCORD_IPC_OPCODE_FRAME, &command)?;
        self.await_response(Some(nonce.as_str()), None)
    }

    fn establish_session(&mut self, client_id: &str) -> Result<(), String> {
        self.stream.configure_timeouts()?;
        self.send_handshake(client_id)?;
        self.await_response(None, Some(DISCORD_READY_EVENT))
    }

    fn send_handshake(&mut self, client_id: &str) -> Result<(), String> {
        let payload = build_handshake_payload(client_id);

        self.send_packet(DISCORD_IPC_OPCODE_HANDSHAKE, &payload)
    }

    fn await_response(
        &mut self,
        expected_nonce: Option<&str>,
        expected_event: Option<&str>,
    ) -> Result<(), String> {
        loop {
            let packet = read_packet(&mut self.stream)?;

            match packet.opcode {
                DISCORD_IPC_OPCODE_FRAME => {
                    let envelope = decode_rpc_envelope(&packet.payload)?;

                    if envelope.evt.as_deref() == Some(DISCORD_ERROR_EVENT) {
                        return Err(format_rpc_error_message(
                            "Discord RPC returned an error",
                            envelope.data.as_ref(),
                        ));
                    }

                    if expected_event.is_some_and(|event| envelope.evt.as_deref() == Some(event))
                        || expected_nonce
                            .is_some_and(|nonce| envelope.nonce.as_deref() == Some(nonce))
                    {
                        return Ok(());
                    }
                }
                DISCORD_IPC_OPCODE_CLOSE => {
                    return Err(format_rpc_close_message(&packet.payload));
                }
                DISCORD_IPC_OPCODE_PING => {
                    write_raw_packet(&mut self.stream, DISCORD_IPC_OPCODE_PONG, packet.payload)?;
                }
                DISCORD_IPC_OPCODE_PONG => {}
                opcode => {
                    return Err(format!("Discord IPC returned unsupported opcode: {opcode}"));
                }
            }
        }
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

    fn configure_timeouts(&self) -> Result<(), String> {
        #[cfg(unix)]
        {
            let timeout = Some(Duration::from_secs(DISCORD_IPC_IO_TIMEOUT_SECS));
            match self {
                Self::Unix(stream) => {
                    stream.set_read_timeout(timeout).map_err(|error| {
                        format!("Failed to configure Discord IPC read timeout: {error}")
                    })?;
                    stream.set_write_timeout(timeout).map_err(|error| {
                        format!("Failed to configure Discord IPC write timeout: {error}")
                    })?;
                }
            }
        }

        #[cfg(windows)]
        {
            let _ = self;
        }

        Ok(())
    }
}

impl Read for DiscordIpcStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            #[cfg(windows)]
            Self::Windows(stream) => stream.read(buf),
            #[cfg(unix)]
            Self::Unix(stream) => stream.read(buf),
        }
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

#[cfg(unix)]
fn push_unique_root(roots: &mut Vec<String>, value: impl Into<String>) {
    let value = value.into();
    let trimmed = value.trim();
    if trimmed.is_empty() || roots.iter().any(|item| item == trimmed) {
        return;
    }

    roots.push(trimmed.to_string());
}

#[cfg(unix)]
fn unix_ipc_roots(
    runtime_dir: Option<&str>,
    tmpdir: Option<&str>,
    tmp: Option<&str>,
    temp: Option<&str>,
) -> Vec<String> {
    let mut roots = Vec::new();

    let runtime_dir = runtime_dir.map(str::trim).filter(|value| !value.is_empty());
    if let Some(runtime_dir) = runtime_dir {
        push_unique_root(&mut roots, runtime_dir);
        push_unique_root(
            &mut roots,
            format!("{runtime_dir}/app/com.discordapp.Discord"),
        );
        push_unique_root(
            &mut roots,
            format!("{runtime_dir}/app/com.discordapp.DiscordCanary"),
        );
        push_unique_root(
            &mut roots,
            format!("{runtime_dir}/.flatpak/dev.vencord.Vesktop/xdg-run"),
        );
        push_unique_root(&mut roots, format!("{runtime_dir}/app/dev.vencord.Vesktop"));
    }

    for value in [tmpdir, tmp, temp] {
        if let Some(value) = value {
            push_unique_root(&mut roots, value);
        }
    }
    push_unique_root(&mut roots, "/tmp");

    roots
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
        return unix_ipc_roots(
            std::env::var("XDG_RUNTIME_DIR").ok().as_deref(),
            std::env::var("TMPDIR").ok().as_deref(),
            std::env::var("TMP").ok().as_deref(),
            std::env::var("TEMP").ok().as_deref(),
        )
        .into_iter()
        .flat_map(|root| (0..10).map(move |index| format!("{root}/discord-ipc-{index}")))
        .collect();
    }

    #[allow(unreachable_code)]
    Vec::new()
}

fn format_rpc_error_message(prefix: &str, data: Option<&Value>) -> String {
    let code = data
        .and_then(|value| value.get("code"))
        .and_then(Value::as_i64);
    let message = data
        .and_then(|value| value.get("message"))
        .and_then(Value::as_str)
        .or_else(|| {
            data.and_then(|value| value.get("error"))
                .and_then(Value::as_str)
        });

    match (code, message) {
        (Some(code), Some(message)) => format!("{prefix}: {code} - {message}"),
        (Some(code), None) => format!("{prefix}: {code}"),
        (None, Some(message)) => format!("{prefix}: {message}"),
        (None, None) => prefix.to_string(),
    }
}

fn format_rpc_close_message(payload: &[u8]) -> String {
    match decode_rpc_envelope(payload) {
        Ok(envelope) => {
            format_rpc_error_message("Discord IPC connection closed", envelope.data.as_ref())
        }
        Err(_) => "Discord IPC connection closed".to_string(),
    }
}

fn append_linux_flatpak_hint(error: String) -> String {
    #[cfg(unix)]
    {
        format!(
            "{error}. If you are using Flatpak Discord or Vesktop, ensure its Discord IPC socket is exposed to native applications."
        )
    }

    #[cfg(not(unix))]
    {
        error
    }
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
    fn read_packet_reads_header_and_body() {
        let packet_bytes = encode_packet_bytes(
            DISCORD_IPC_OPCODE_FRAME,
            &serde_json::json!({ "cmd": "SET_ACTIVITY" }),
        )
        .expect("packet should encode");
        let mut reader = std::io::Cursor::new(packet_bytes);

        let packet = read_packet(&mut reader).expect("packet should decode");

        assert_eq!(packet.opcode, DISCORD_IPC_OPCODE_FRAME);
        assert_eq!(
            serde_json::from_slice::<Value>(&packet.payload).expect("payload json")["cmd"],
            "SET_ACTIVITY"
        );
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
    fn format_rpc_error_message_uses_code_and_message_when_available() {
        let error = format_rpc_error_message(
            "Discord RPC returned an error",
            Some(&serde_json::json!({
                "code": 4006,
                "message": "Not authenticated"
            })),
        );

        assert_eq!(
            error,
            "Discord RPC returned an error: 4006 - Not authenticated"
        );
    }

    #[test]
    fn unix_ipc_roots_prioritizes_runtime_and_known_flatpak_locations() {
        #[cfg(unix)]
        {
            let roots = unix_ipc_roots(
                Some("/run/user/1000"),
                Some("/var/tmp/custom"),
                Some("/tmp"),
                Some("/tmp"),
            );

            assert_eq!(
                roots,
                vec![
                    "/run/user/1000".to_string(),
                    "/run/user/1000/app/com.discordapp.Discord".to_string(),
                    "/run/user/1000/app/com.discordapp.DiscordCanary".to_string(),
                    "/run/user/1000/.flatpak/dev.vencord.Vesktop/xdg-run".to_string(),
                    "/run/user/1000/app/dev.vencord.Vesktop".to_string(),
                    "/var/tmp/custom".to_string(),
                    "/tmp".to_string(),
                ]
            );
        }
    }

    #[test]
    fn command_builders_produce_expected_shapes() {
        let activity = DiscordPresenceRequest {
            details: Some("Watching Frieren".to_string()),
            ..Default::default()
        }
        .sanitize()
        .into_activity();
        let nonce = build_nonce();

        let command = build_activity_command(123, activity, nonce.clone());
        let command_json = serde_json::to_value(&command).expect("command should serialize");
        let handshake_json = build_handshake_payload("456");

        assert_eq!(command_json["cmd"], "SET_ACTIVITY");
        assert_eq!(command_json["args"]["pid"], 123);
        assert_eq!(
            command_json["args"]["activity"]["details"],
            "Watching Frieren"
        );
        assert_eq!(command_json["nonce"], nonce);
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
        let nonce = build_nonce();

        let packet = encode_packet_bytes(
            DISCORD_IPC_OPCODE_FRAME,
            &DiscordIpcCommand {
                cmd: DISCORD_COMMAND_SET_ACTIVITY,
                args: DiscordIpcCommandArgs { pid: 123, activity },
                nonce: nonce.clone(),
            },
        )
        .expect("packet should encode");
        let body = std::str::from_utf8(&packet[8..]).expect("body should be utf-8");

        assert!(body.contains("\"cmd\":\"SET_ACTIVITY\""));
        assert!(body.contains("\"details\":\"Watching Frieren\""));
        assert!(body.contains(&format!("\"nonce\":\"{nonce}\"")));
    }
}
