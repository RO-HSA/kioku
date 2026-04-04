use std::sync::Mutex;

use super::ipc::DiscordIpcConnection;
use super::model::{DiscordActivity, DiscordPresenceRequest};
use super::sanitize::normalize_client_id;

const DISCORD_CLIENT_ID_ENV: &str = "DISCORD_CLIENT_ID";
const DISCORD_CLIENT_ID_FALLBACK: Option<&str> = Some("1475090774516568215");

pub struct DiscordRpcState {
    client: Mutex<DiscordRpcClient>,
}

impl DiscordRpcState {
    pub fn from_env() -> Self {
        let client_id = std::env::var(DISCORD_CLIENT_ID_ENV)
            .ok()
            .or_else(|| DISCORD_CLIENT_ID_FALLBACK.map(str::to_string));

        Self {
            client: Mutex::new(DiscordRpcClient::new(client_id)),
        }
    }

    pub fn configure_client_id(&self, client_id: Option<String>) -> Result<bool, String> {
        let mut client = self.lock_client()?;
        Ok(client.set_client_id(client_id))
    }

    pub fn set_presence(&self, request: DiscordPresenceRequest) -> Result<(), String> {
        let mut client = self.lock_client()?;
        client.set_presence(request)
    }

    pub fn clear_presence(&self) -> Result<(), String> {
        let mut client = self.lock_client()?;
        client.clear_presence()
    }

    fn lock_client(&self) -> Result<std::sync::MutexGuard<'_, DiscordRpcClient>, String> {
        self.client
            .lock()
            .map_err(|_| "Discord RPC state is unavailable".to_string())
    }
}

#[derive(Default)]
struct DiscordRpcClient {
    client_id: Option<String>,
    connection: Option<DiscordIpcConnection>,
}

fn missing_client_id_error() -> String {
    format!(
        "Discord RPC client id is not configured. Set {}.",
        DISCORD_CLIENT_ID_ENV
    )
}

fn update_client_id_state<T>(
    current_client_id: &mut Option<String>,
    connection: &mut Option<T>,
    client_id: Option<String>,
) -> bool {
    let normalized = normalize_client_id(client_id);
    if *current_client_id != normalized {
        *connection = None;
        *current_client_id = normalized;
    }

    current_client_id.is_some()
}

fn send_activity_with_connection<T, Connect, Send>(
    client_id: Option<&str>,
    connection: &mut Option<T>,
    activity: Option<DiscordActivity>,
    mut connect: Connect,
    mut send: Send,
) -> Result<(), String>
where
    Connect: FnMut(&str) -> Result<T, String>,
    Send: FnMut(&mut T, Option<DiscordActivity>) -> Result<(), String>,
{
    let Some(client_id) = client_id else {
        return Err(missing_client_id_error());
    };

    if connection.is_none() {
        *connection = Some(connect(client_id)?);
    }

    let retry_activity = activity.clone();
    if let Some(active_connection) = connection.as_mut() {
        if send(active_connection, activity).is_ok() {
            return Ok(());
        }
    }

    *connection = None;
    *connection = Some(connect(client_id)?);

    if let Some(active_connection) = connection.as_mut() {
        return send(active_connection, retry_activity);
    }

    Err("Discord RPC connection is unavailable".to_string())
}

impl DiscordRpcClient {
    fn new(client_id: Option<String>) -> Self {
        Self {
            client_id: normalize_client_id(client_id),
            connection: None,
        }
    }

    fn set_client_id(&mut self, client_id: Option<String>) -> bool {
        update_client_id_state(&mut self.client_id, &mut self.connection, client_id)
    }

    fn set_presence(&mut self, request: DiscordPresenceRequest) -> Result<(), String> {
        let activity = request.sanitize().into_activity();
        self.send_activity(activity)
    }

    fn clear_presence(&mut self) -> Result<(), String> {
        self.send_activity(None)
    }

    fn send_activity(&mut self, activity: Option<DiscordActivity>) -> Result<(), String> {
        send_activity_with_connection(
            self.client_id.as_deref(),
            &mut self.connection,
            activity,
            DiscordIpcConnection::connect,
            |connection, payload| connection.send_activity(payload),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_client_id_state_normalizes_values_and_clears_stale_connections() {
        let mut client_id = Some("123".to_string());
        let mut connection = Some(1_u8);

        assert!(update_client_id_state(
            &mut client_id,
            &mut connection,
            Some("123".to_string())
        ));
        assert_eq!(client_id.as_deref(), Some("123"));
        assert_eq!(connection, Some(1));

        assert!(!update_client_id_state(
            &mut client_id,
            &mut connection,
            Some("invalid".to_string())
        ));
        assert_eq!(client_id, None);
        assert_eq!(connection, None);
    }

    #[test]
    fn send_activity_with_connection_requires_a_client_id() {
        let mut connection: Option<u8> = None;
        let error =
            send_activity_with_connection(None, &mut connection, None, |_| Ok(1), |_, _| Ok(()))
                .unwrap_err();

        assert_eq!(error, missing_client_id_error());
    }

    #[test]
    fn send_activity_with_connection_uses_existing_connection_when_send_succeeds() {
        let mut connection = Some(7_u8);
        let mut connect_calls = 0;
        let mut send_calls = 0;

        let result = send_activity_with_connection(
            Some("123"),
            &mut connection,
            None,
            |_| {
                connect_calls += 1;
                Ok(9_u8)
            },
            |active_connection, _| {
                send_calls += 1;
                assert_eq!(*active_connection, 7);
                Ok(())
            },
        );

        assert!(result.is_ok());
        assert_eq!(connect_calls, 0);
        assert_eq!(send_calls, 1);
        assert_eq!(connection, Some(7));
    }

    #[test]
    fn send_activity_with_connection_retries_after_send_failure() {
        let mut connection = Some(1_u8);
        let mut connect_calls = 0;
        let mut send_attempts = 0;

        let result = send_activity_with_connection(
            Some("123"),
            &mut connection,
            None,
            |_| {
                connect_calls += 1;
                Ok(2_u8)
            },
            |active_connection, _| {
                send_attempts += 1;
                if send_attempts == 1 {
                    assert_eq!(*active_connection, 1);
                    Err("first send failed".to_string())
                } else {
                    assert_eq!(*active_connection, 2);
                    Ok(())
                }
            },
        );

        assert!(result.is_ok());
        assert_eq!(connect_calls, 1);
        assert_eq!(send_attempts, 2);
        assert_eq!(connection, Some(2));
    }

    #[test]
    fn send_activity_with_connection_returns_second_connection_error() {
        let mut connection = Some(1_u8);
        let mut connect_calls = 0;

        let error = send_activity_with_connection(
            Some("123"),
            &mut connection,
            None,
            |_| {
                connect_calls += 1;
                Err("reconnect failed".to_string())
            },
            |_, _| Err("send failed".to_string()),
        )
        .unwrap_err();

        assert_eq!(error, "reconnect failed");
        assert_eq!(connect_calls, 1);
        assert_eq!(connection, None);
    }
}
