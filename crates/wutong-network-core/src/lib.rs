// SPDX-License-Identifier: AGPL-3.0-only
//! Wutong Network engine.
//! Client side of the `wutong-networkd` JSON-lines IPC (version-negotiated, optional).

use serde_json::{Value, json};

/// Daemon state as seen by the GUI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DaemonState {
    /// Not running / not installed: GUI keeps working on local cache.
    Offline,
    /// Connected with the given IPC version.
    Connected(u64),
    /// Incompatible IPC version.
    Incompatible(u64),
}

/// IPC version this GUI speaks.
pub const IPC_VERSION: u64 = 1;

/// Build a request line.
pub fn request(id: u64, method: &str) -> String {
    json!({"id": id, "method": method}).to_string()
}

/// Interpret a `hello` response line.
pub fn parse_hello(line: &str) -> DaemonState {
    match wutong_core::wire::strict::parse(line.as_bytes())
        .ok()
        .and_then(|v: Value| v.pointer("/result/ipc_version").and_then(Value::as_u64))
    {
        Some(IPC_VERSION) => DaemonState::Connected(IPC_VERSION),
        Some(v) => DaemonState::Incompatible(v),
        None => DaemonState::Offline,
    }
}

/// Probe the daemon socket (Unix). Never blocks the GUI for long.
pub fn probe(socket: &str) -> DaemonState {
    #[cfg(unix)]
    {
        use std::io::{BufRead, BufReader, Write};
        let Ok(mut s) = std::os::unix::net::UnixStream::connect(socket) else {
            return DaemonState::Offline;
        };
        let _ = s.set_read_timeout(Some(std::time::Duration::from_millis(500)));
        if writeln!(s, "{}", request(1, "hello")).is_err() {
            return DaemonState::Offline;
        }
        let mut line = String::new();
        if BufReader::new(s).read_line(&mut line).is_err() {
            return DaemonState::Offline;
        }
        parse_hello(&line)
    }
    #[cfg(not(unix))]
    {
        let _ = socket;
        DaemonState::Offline
    }
}

/// One-line description used by the GUI.
pub fn describe_path(socket: &str) -> String {
    match probe(socket) {
        DaemonState::Offline => "wutong-networkd: offline (browsing local cache only)".into(),
        DaemonState::Connected(v) => format!("wutong-networkd: connected (IPC v{v})"),
        DaemonState::Incompatible(v) => format!("wutong-networkd: incompatible IPC v{v}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_parsing_and_offline() {
        assert_eq!(
            parse_hello(r#"{"id":1,"result":{"ipc_version":1}}"#),
            DaemonState::Connected(1)
        );
        assert_eq!(
            parse_hello(r#"{"id":1,"result":{"ipc_version":9}}"#),
            DaemonState::Incompatible(9)
        );
        assert_eq!(probe("/nonexistent/wutong.sock"), DaemonState::Offline);
    }
}
