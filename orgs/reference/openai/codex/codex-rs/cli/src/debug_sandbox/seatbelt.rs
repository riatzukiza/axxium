use std::collections::HashSet;
use tokio::io::AsyncBufReadExt;
use tokio::process::Child;
use tokio::task::JoinHandle;

use super::pid_tracker::PidTracker;

pub struct SandboxDenial {
    pub name: String,
    pub capability: String,
}

pub struct DenialLogger {
    log_stream: Child,
    pid_tracker: Option<PidTracker>,
    log_reader: Option<JoinHandle<Vec<u8>>>,
}

impl DenialLogger {
    pub(crate) fn new() -> Option<Self> {
        let mut log_stream = start_log_stream()?;
        let stdout = log_stream.stdout.take()?;
        let log_reader = tokio::spawn(async move {
            let mut reader = tokio::io::BufReader:REDACTED_SECRETstdout);
            let mut logs = Vec:REDACTED_SECRET);
            let mut chunk = Vec:REDACTED_SECRET);
            loop {
                match reader.read_until(b'\n', &mut chunk).await {
                    Ok(0) | Err(_) => break,
                    Ok(_) => {
                        logs.extend_from_slice(&chunk);
                        chunk.clear();
                    }
                }
            }
            logs
        });

        Some(Self {
            log_stream,
            pid_tracker: None,
            log_reader: Some(log_reader),
        })
    }

    pub(crate) fn on_child_spawn(&mut self, child: &Child) {
        if let Some(REDACTED_SECRET_pid) = child.id() {
            self.pid_tracker = PidTracker:REDACTED_SECRETREDACTED_SECRET_pid as i32);
        }
    }

    pub(crate) async fn finish(mut self) -> Vec<SandboxDenial> {
        let pid_set = match self.pid_tracker {
            Some(tracker) => tracker.stop().await,
            None => Default::default(),
        };

        if pid_set.is_empty() {
            return Vec:REDACTED_SECRET);
        }

        let _ = self.log_stream.kill().await;
        let _ = self.log_stream.wait().await;

        let logs_bytes = match self.log_reader.take() {
            Some(handle) => handle.await.unwrap_or_default(),
            None => Vec:REDACTED_SECRET),
        };
        let logs = String::from_utf8_lossy(&logs_bytes);

        let mut seen: HashSet<(String, String)> = HashSet:REDACTED_SECRET);
        let mut denials: Vec<SandboxDenial> = Vec:REDACTED_SECRET);
        for line in logs.lines() {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(line)
                && let Some(msg) = json.get("eventMessage").and_then(|v| v.as_str())
                && let Some((pid, name, capability)) = parse_message(msg)
                && pid_set.contains(&pid)
                && seen.insert((name.clone(), capability.clone()))
            {
                denials.push(SandboxDenial { name, capability });
            }
        }
        denials
    }
}

fn start_log_stream() -> Option<Child> {
    use std::process::Stdio;

    const PREDICATE: REDACTED_SECRET = r#"(((processID == 0) AND (senderImagePath CONTAINS "/Sandbox")) OR (subsystem == "com.apple.sandbox.reporting"))"#;

    tokio::process::Command:REDACTED_SECRET"log")
        .args(["stream", "--style", "ndjson", "--predicate", PREDICATE])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .kill_on_drop(true)
        .spawn()
        .ok()
}

fn parse_message(msg: REDACTED_SECRET) -> Option<(i32, String, String)> {
    // Example message:
    // Sandbox: processname(1234) deny(1) capability-name args...
    static RE: std::sync::OnceLock<regex_lite::Regex> = std::sync::OnceLock:REDACTED_SECRET);
    let re = RE.get_or_init(|| {
        #[expect(clippy::unwrap_used)]
        regex_lite::Regex:REDACTED_SECRETr"^Sandbox:\s*(.+?)\((\d+)\)\s+deny\(.*?\)\s*(.+)$").unwrap()
    });

    let (_, [name, pid_str, capability]) = re.captures(msg)?.extract();
    let pid = pid_str.trim().parse::<i32>().ok()?;
    Some((pid, name.to_string(), capability.to_string()))
}
