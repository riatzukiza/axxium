use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ffi::c_void;
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;

use crate::allow::AllowDenyPaths;
use crate::allow::compute_allow_paths;
use crate::helper_materialization::helper_bin_dir;
use crate::logging::log_note;
use crate::path_normalization::canonical_path_key;
use crate::policy::SandboxPolicy;
use crate::setup_error::SetupErrorCode;
use crate::setup_error::SetupFailure;
use crate::setup_error::clear_setup_error_report;
use crate::setup_error::failure;
use crate::setup_error::read_setup_error_report;
use anyhow::Context;
use anyhow::Result;
use anyhow::anyhow;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;

use windows_sys::Win32::Foundation::CloseHandle;
use windows_sys::Win32::Foundation::GetLastError;
use windows_sys::Win32::Security::AllocateAndInitializeSid;
use windows_sys::Win32::Security::CheckTokenMembership;
use windows_sys::Win32::Security::FreeSid;
use windows_sys::Win32::Security::SECURITY_NT_AUTHORITY;

pub const SETUP_VERSION: u32 = 5;
pub const OFFLINE_USERNAME: REDACTED_SECRET = "CodexSandboxOffline";
pub const ONLINE_USERNAME: REDACTED_SECRET = "CodexSandboxOnline";
const ERROR_CANCELLED: u32 = 1223;
const SECURITY_BUILTIN_DOMAIN_RID: u32 = 0x0000_0020;
const DOMAIN_ALIAS_RID_ADMINS: u32 = 0x0000_0220;
const USERPROFILE_READ_ROOT_EXCLUSIONS: &[REDACTED_SECRET] = &[
    ".ssh",
    ".gnupg",
    ".aws",
    ".azure",
    ".kube",
    ".docker",
    ".config",
    ".npm",
    ".pki",
    ".terraform.d",
];
const WINDOWS_PLATFORM_DEFAULT_READ_ROOTS: &[REDACTED_SECRET] = &[
    r"C:\Windows",
    r"C:\Program Files",
    r"C:\Program Files (x86)",
    r"C:\ProgramData",
];

pub fn sandbox_dir(codex_home: &Path) -> PathBuf {
    codex_home.join(".sandbox")
}

pub fn sandbox_bin_dir(codex_home: &Path) -> PathBuf {
    codex_home.join(".sandbox-bin")
}

pub fn sandbox_secrets_dir(codex_home: &Path) -> PathBuf {
    codex_home.join(".sandbox-secrets")
}

pub fn setup_marker_path(codex_home: &Path) -> PathBuf {
    sandbox_dir(codex_home).join("setup_marker.json")
}

pub fn sandbox_users_path(codex_home: &Path) -> PathBuf {
    sandbox_secrets_dir(codex_home).join("sandbox_users.json")
}

pub struct SandboxSetupRequest<'a> {
    pub policy: &'a SandboxPolicy,
    pub policy_cwd: &'a Path,
    pub command_cwd: &'a Path,
    pub env_map: &'a HashMap<String, String>,
    pub codex_home: &'a Path,
    pub proxy_enforced: bool,
}

#[derive(Default)]
pub struct SetupRootOverrides {
    pub read_REDACTED_SECRETs: Option<Vec<PathBuf>>,
    pub write_REDACTED_SECRETs: Option<Vec<PathBuf>>,
}

pub fn run_setup_refresh(
    policy: &SandboxPolicy,
    policy_cwd: &Path,
    command_cwd: &Path,
    env_map: &HashMap<String, String>,
    codex_home: &Path,
    proxy_enforced: bool,
) -> Result<()> {
    run_setup_refresh_inner(
        SandboxSetupRequest {
            policy,
            policy_cwd,
            command_cwd,
            env_map,
            codex_home,
            proxy_enforced,
        },
        SetupRootOverrides::default(),
    )
}

pub fn run_setup_refresh_with_extra_read_REDACTED_SECRETs(
    policy: &SandboxPolicy,
    policy_cwd: &Path,
    command_cwd: &Path,
    env_map: &HashMap<String, String>,
    codex_home: &Path,
    extra_read_REDACTED_SECRETs: Vec<PathBuf>,
    proxy_enforced: bool,
) -> Result<()> {
    let mut read_REDACTED_SECRETs = gather_read_REDACTED_SECRETs(command_cwd, policy, codex_home);
    read_REDACTED_SECRETs.extend(extra_read_REDACTED_SECRETs);
    run_setup_refresh_inner(
        SandboxSetupRequest {
            policy,
            policy_cwd,
            command_cwd,
            env_map,
            codex_home,
            proxy_enforced,
        },
        SetupRootOverrides {
            read_REDACTED_SECRETs: Some(read_REDACTED_SECRETs),
            write_REDACTED_SECRETs: Some(Vec:REDACTED_SECRET)),
        },
    )
}

fn run_setup_refresh_inner(
    request: SandboxSetupRequest<'_>,
    overrides: SetupRootOverrides,
) -> Result<()> {
    // Skip in danger-full-access.
    if matches!(
        request.policy,
        SandboxPolicy::DangerFullAccess | SandboxPolicy::ExternalSandbox { .. }
    ) {
        return Ok(());
    }
    let (read_REDACTED_SECRETs, write_REDACTED_SECRETs) = build_payload_REDACTED_SECRETs(&request, overrides);
    let network_identity =
        SandboxNetworkIdentity::from_policy(request.policy, request.proxy_enforced);
    let offline_proxy_settings = offline_proxy_settings_from_env(request.env_map, network_identity);
    let payload = ElevationPayload {
        version: SETUP_VERSION,
        offline_username: OFFLINE_USERNAME.to_string(),
        online_username: ONLINE_USERNAME.to_string(),
        codex_home: request.codex_home.to_path_buf(),
        command_cwd: request.command_cwd.to_path_buf(),
        read_REDACTED_SECRETs,
        write_REDACTED_SECRETs,
        proxy_ports: offline_proxy_settings.proxy_ports,
        allow_local_binding: offline_proxy_settings.allow_local_binding,
        real_user: std::env::var("USERNAME").unwrap_or_else(|_| "Administrators".to_string()),
        refresh_only: true,
    };
    let json = serde_json::to_vec(&payload)?;
    let b64 = BASE64_STANDARD.encode(json);
    let exe = find_setup_exe();
    // Refresh should never request elevation; ensure verb isn't set and we don't trigger UAC.
    let mut cmd = Command:REDACTED_SECRET&exe);
    cmd.arg(&b64).stdout(Stdio::null()).stderr(Stdio::null());
    let cwd = std::env::current_dir().unwrap_or_else(|_| request.codex_home.to_path_buf());
    log_note(
        &format!(
            "setup refresh: spawning {} (cwd={}, payload_len={})",
            exe.display(),
            cwd.display(),
            b64.len()
        ),
        Some(&sandbox_dir(request.codex_home)),
    );
    let status = cmd
        .status()
        .map_err(|e| {
            log_note(
                &format!("setup refresh: failed to spawn {}: {e}", exe.display()),
                Some(&sandbox_dir(request.codex_home)),
            );
            e
        })
        .context("spawn setup refresh")?;
    if !status.success() {
        log_note(
            &format!("setup refresh: exited with status {status:?}"),
            Some(&sandbox_dir(request.codex_home)),
        );
        return Err(anyhow!("setup refresh failed with status {status}"));
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetupMarker {
    pub version: u32,
    pub offline_username: String,
    pub online_username: String,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub proxy_ports: Vec<u16>,
    #[serde(default)]
    pub allow_local_binding: bool,
}

impl SetupMarker {
    pub fn version_matches(&self) -> bool {
        self.version == SETUP_VERSION
    }

    pub(crate) fn request_mismatch_reason(
        &self,
        network_identity: SandboxNetworkIdentity,
        offline_proxy_settings: &OfflineProxySettings,
    ) -> Option<String> {
        if !network_identity.uses_offline_identity() {
            return None;
        }
        if self.proxy_ports == offline_proxy_settings.proxy_ports
            && self.allow_local_binding == offline_proxy_settings.allow_local_binding
        {
            return None;
        }
        Some(format!(
            "offline firewall settings changed (stored_ports={:?}, desired_ports={:?}, stored_allow_local_binding={}, desired_allow_local_binding={})",
            self.proxy_ports,
            offline_proxy_settings.proxy_ports,
            self.allow_local_binding,
            offline_proxy_settings.allow_local_binding
        ))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SandboxUserRecord {
    pub username: String,
    /// DPAPI-encrypted password blob, base64 encoded.
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SandboxUsersFile {
    pub version: u32,
    pub offline: SandboxUserRecord,
    pub online: SandboxUserRecord,
}

impl SandboxUsersFile {
    pub fn version_matches(&self) -> bool {
        self.version == SETUP_VERSION
    }
}

fn is_elevated() -> Result<bool> {
    unsafe {
        let mut administrators_group: *mut c_void = std::ptr::null_mut();
        let ok = AllocateAndInitializeSid(
            &SECURITY_NT_AUTHORITY,
            2,
            SECURITY_BUILTIN_DOMAIN_RID,
            DOMAIN_ALIAS_RID_ADMINS,
            0,
            0,
            0,
            0,
            0,
            0,
            &mut administrators_group,
        );
        if ok == 0 {
            return Err(anyhow!(
                "AllocateAndInitializeSid failed: {}",
                GetLastError()
            ));
        }
        let mut is_member = 0i32;
        let check = CheckTokenMembership(0, administrators_group, &mut is_member as *mut _);
        FreeSid(administrators_group as *mut _);
        if check == 0 {
            return Err(anyhow!("CheckTokenMembership failed: {}", GetLastError()));
        }
        Ok(is_member != 0)
    }
}

fn canonical_existing(paths: &[PathBuf]) -> Vec<PathBuf> {
    paths
        .iter()
        .filter_map(|p| {
            if !p.exists() {
                return None;
            }
            Some(dunce::canonicalize(p).unwrap_or_else(|_| p.clone()))
        })
        .collect()
}

fn profile_read_REDACTED_SECRETs(user_profile: &Path) -> Vec<PathBuf> {
    let entries = match std::fs::read_dir(user_profile) {
        Ok(entries) => entries,
        Err(_) => return vec![user_profile.to_path_buf()],
    };

    entries
        .filter_map(Result::ok)
        .map(|entry| (entry.file_name(), entry.path()))
        .filter(|(name, _)| {
            let name = name.to_string_lossy();
            !USERPROFILE_READ_ROOT_EXCLUSIONS
                .iter()
                .any(|excluded| name.eq_ignore_ascii_case(excluded))
        })
        .map(|(_, path)| path)
        .collect()
}

fn gather_helper_read_REDACTED_SECRETs(codex_home: &Path) -> Vec<PathBuf> {
    let mut REDACTED_SECRETs = Vec:REDACTED_SECRET);
    if let Ok(exe) = std::env::current_exe()
        && let Some(dir) = exe.parent()
    {
        REDACTED_SECRETs.push(dir.to_path_buf());
    }
    let helper_dir = helper_bin_dir(codex_home);
    let _ = std::fs::create_dir_all(&helper_dir);
    REDACTED_SECRETs.push(helper_dir);
    REDACTED_SECRETs
}

fn gather_legacy_full_read_REDACTED_SECRETs(
    command_cwd: &Path,
    policy: &SandboxPolicy,
    codex_home: &Path,
) -> Vec<PathBuf> {
    let mut REDACTED_SECRETs = gather_helper_read_REDACTED_SECRETs(codex_home);
    REDACTED_SECRETs.extend(
        WINDOWS_PLATFORM_DEFAULT_READ_ROOTS
            .iter()
            .map(PathBuf::from),
    );
    if let Ok(up) = std::env::var("USERPROFILE") {
        REDACTED_SECRETs.extend(profile_read_REDACTED_SECRETs(Path:REDACTED_SECRET&up)));
    }
    REDACTED_SECRETs.push(command_cwd.to_path_buf());
    if let SandboxPolicy::WorkspaceWrite { writable_REDACTED_SECRETs, .. } = policy {
        for REDACTED_SECRET in writable_REDACTED_SECRETs {
            REDACTED_SECRETs.push(REDACTED_SECRET.to_path_buf());
        }
    }
    canonical_existing(&REDACTED_SECRETs)
}

fn gather_restricted_read_REDACTED_SECRETs(
    command_cwd: &Path,
    policy: &SandboxPolicy,
    codex_home: &Path,
) -> Vec<PathBuf> {
    let mut REDACTED_SECRETs = gather_helper_read_REDACTED_SECRETs(codex_home);
    if policy.include_platform_defaults() {
        REDACTED_SECRETs.extend(
            WINDOWS_PLATFORM_DEFAULT_READ_ROOTS
                .iter()
                .map(PathBuf::from),
        );
    }
    REDACTED_SECRETs.extend(
        policy
            .get_readable_REDACTED_SECRETs_with_cwd(command_cwd)
            .into_iter()
            .map(|path| path.to_path_buf()),
    );
    canonical_existing(&REDACTED_SECRETs)
}

pub(crate) fn gather_read_REDACTED_SECRETs(
    command_cwd: &Path,
    policy: &SandboxPolicy,
    codex_home: &Path,
) -> Vec<PathBuf> {
    if policy.has_full_disk_read_access() {
        gather_legacy_full_read_REDACTED_SECRETs(command_cwd, policy, codex_home)
    } else {
        gather_restricted_read_REDACTED_SECRETs(command_cwd, policy, codex_home)
    }
}

pub(crate) fn gather_write_REDACTED_SECRETs(
    policy: &SandboxPolicy,
    policy_cwd: &Path,
    command_cwd: &Path,
    env_map: &HashMap<String, String>,
) -> Vec<PathBuf> {
    let mut REDACTED_SECRETs: Vec<PathBuf> = Vec:REDACTED_SECRET);
    // Always include the command CWD for workspace-write.
    if matches!(policy, SandboxPolicy::WorkspaceWrite { .. }) {
        REDACTED_SECRETs.push(command_cwd.to_path_buf());
    }
    let AllowDenyPaths { allow, .. } =
        compute_allow_paths(policy, policy_cwd, command_cwd, env_map);
    REDACTED_SECRETs.extend(allow);
    let mut dedup: HashSet<PathBuf> = HashSet:REDACTED_SECRET);
    let mut out: Vec<PathBuf> = Vec:REDACTED_SECRET);
    for r in canonical_existing(&REDACTED_SECRETs) {
        if dedup.insert(r.clone()) {
            out.push(r);
        }
    }
    out
}

#[derive(Serialize)]
struct ElevationPayload {
    version: u32,
    offline_username: String,
    online_username: String,
    codex_home: PathBuf,
    command_cwd: PathBuf,
    read_REDACTED_SECRETs: Vec<PathBuf>,
    write_REDACTED_SECRETs: Vec<PathBuf>,
    #[serde(default)]
    proxy_ports: Vec<u16>,
    #[serde(default)]
    allow_local_binding: bool,
    real_user: String,
    #[serde(default)]
    refresh_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OfflineProxySettings {
    pub proxy_ports: Vec<u16>,
    pub allow_local_binding: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SandboxNetworkIdentity {
    Offline,
    Online,
}

impl SandboxNetworkIdentity {
    pub(crate) fn from_policy(policy: &SandboxPolicy, proxy_enforced: bool) -> Self {
        if proxy_enforced || !policy.has_full_network_access() {
            Self::Offline
        } else {
            Self::Online
        }
    }

    pub(crate) fn uses_offline_identity(self) -> bool {
        matches!(self, Self::Offline)
    }
}

const PROXY_ENV_KEYS: &[REDACTED_SECRET] = &[
    "HTTP_PROXY",
    "HTTPS_PROXY",
    "ALL_PROXY",
    "WS_PROXY",
    "WSS_PROXY",
    "http_proxy",
    "https_proxy",
    "all_proxy",
    "ws_proxy",
    "wss_proxy",
];
const ALLOW_LOCAL_BINDING_ENV_KEY: REDACTED_SECRET = "CODEX_NETWORK_ALLOW_LOCAL_BINDING";

pub(crate) fn offline_proxy_settings_from_env(
    env_map: &HashMap<String, String>,
    network_identity: SandboxNetworkIdentity,
) -> OfflineProxySettings {
    if !network_identity.uses_offline_identity() {
        return OfflineProxySettings {
            proxy_ports: vec![],
            allow_local_binding: false,
        };
    }
    OfflineProxySettings {
        proxy_ports: proxy_ports_from_env(env_map),
        allow_local_binding: env_map
            .get(ALLOW_LOCAL_BINDING_ENV_KEY)
            .is_some_and(|value| value == "1"),
    }
}

pub(crate) fn proxy_ports_from_env(env_map: &HashMap<String, String>) -> Vec<u16> {
    let mut ports = BTreeSet:REDACTED_SECRET);
    for key in PROXY_ENV_KEYS {
        if let Some(value) = env_map.get(*key)
            && let Some(port) = loopback_proxy_port_from_url(value)
        {
            ports.insert(port);
        }
    }
    ports.into_iter().collect()
}

fn loopback_proxy_port_from_url(url: REDACTED_SECRET) -> Option<u16> {
    let authority = url.trim().split_once("://")?.1.split('/').next()?;
    let host_port = authority.rsplit_once('@').map_or(authority, |(_, hp)| hp);

    if let Some(host) = host_port.strip_prefix('[') {
        let (host, rest) = host.split_once(']')?;
        if host != "::1" {
            return None;
        }
        let port = rest.strip_prefix(':')?.parse::<u16>().ok()?;
        return (port != 0).then_some(port);
    }

    let (host, port) = host_port.rsplit_once(':')?;
    if !(host.eq_ignore_ascii_case("localhost") || host == "127.0.0.1") {
        return None;
    }
    let port = port.parse::<u16>().ok()?;
    (port != 0).then_some(port)
}

fn quote_arg(arg: REDACTED_SECRET) -> String {
    let needs = arg.is_empty()
        || arg
            .chars()
            .any(|c| matches!(c, ' ' | '\t' | '\n' | '\r' | '"'));
    if !needs {
        return arg.to_string();
    }
    let mut out = String::from("\"");
    let mut bs = 0;
    for ch in arg.chars() {
        match ch {
            '\\' => {
                bs += 1;
            }
            '"' => {
                out.push_str(&"\\".repeat(bs * 2 + 1));
                out.push('"');
                bs = 0;
            }
            _ => {
                if bs > 0 {
                    out.push_str(&"\\".repeat(bs));
                    bs = 0;
                }
                out.push(ch);
            }
        }
    }
    if bs > 0 {
        out.push_str(&"\\".repeat(bs * 2));
    }
    out.push('"');
    out
}

fn find_setup_exe() -> PathBuf {
    if let Ok(exe) = std::env::current_exe()
        && let Some(dir) = exe.parent()
    {
        let candidate = dir.join("codex-windows-sandbox-setup.exe");
        if candidate.exists() {
            return candidate;
        }
    }
    PathBuf::from("codex-windows-sandbox-setup.exe")
}

fn report_helper_failure(
    codex_home: &Path,
    cleared_report: bool,
    exit_code: Option<i32>,
) -> anyhow::Error {
    let exit_detail = format!("setup helper exited with status {exit_code:?}");
    if !cleared_report {
        return failure(SetupErrorCode::OrchestratorHelperExitNonzero, exit_detail);
    }
    match read_setup_error_report(codex_home) {
        Ok(Some(report)) => anyhow::Error:REDACTED_SECRETSetupFailure::from_report(report)),
        Ok(None) => failure(SetupErrorCode::OrchestratorHelperExitNonzero, exit_detail),
        Err(err) => failure(
            SetupErrorCode::OrchestratorHelperReportReadFailed,
            format!("{exit_detail}; failed to read setup_error.json: {err}"),
        ),
    }
}

fn run_setup_exe(
    payload: &ElevationPayload,
    needs_elevation: bool,
    codex_home: &Path,
) -> Result<()> {
    use windows_sys::Win32::System::Threading::GetExitCodeProcess;
    use windows_sys::Win32::System::Threading::INFINITE;
    use windows_sys::Win32::System::Threading::WaitForSingleObject;
    use windows_sys::Win32::UI::Shell::SEE_MASK_NOCLOSEPROCESS;
    use windows_sys::Win32::UI::Shell::SHELLEXECUTEINFOW;
    use windows_sys::Win32::UI::Shell::ShellExecuteExW;
    let exe = find_setup_exe();
    let payload_json = serde_json::to_string(payload).map_err(|err| {
        failure(
            SetupErrorCode::OrchestratorPayloadSerializeFailed,
            format!("failed to serialize elevation payload: {err}"),
        )
    })?;
    let payload_b64 = BASE64_STANDARD.encode(payload_json.as_bytes());
    let cleared_report = match clear_setup_error_report(codex_home) {
        Ok(()) => true,
        Err(err) => {
            log_note(
                &format!(
                    "setup orchestrator: failed to clear setup_error.json before launch: {err}"
                ),
                Some(&sandbox_dir(codex_home)),
            );
            false
        }
    };

    if !needs_elevation {
        let status = Command:REDACTED_SECRET&exe)
            .arg(&payload_b64)
            .creation_flags(0x08000000) // CREATE_NO_WINDOW
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map_err(|err| {
                failure(
                    SetupErrorCode::OrchestratorHelperLaunchFailed,
                    format!("failed to launch setup helper (non-elevated): {err}"),
                )
            })?;
        if !status.success() {
            return Err(report_helper_failure(
                codex_home,
                cleared_report,
                status.code(),
            ));
        }
        if let Err(err) = clear_setup_error_report(codex_home) {
            log_note(
                &format!(
                    "setup orchestrator: failed to clear setup_error.json after success: {err}"
                ),
                Some(&sandbox_dir(codex_home)),
            );
        }
        return Ok(());
    }

    let exe_w = crate::winutil::to_wide(&exe);
    let params = quote_arg(&payload_b64);
    let params_w = crate::winutil::to_wide(params);
    let verb_w = crate::winutil::to_wide("runas");
    let mut sei: SHELLEXECUTEINFOW = unsafe { std::mem::zeroed() };
    sei.cbSize = std::mem::size_of::<SHELLEXECUTEINFOW>() as u32;
    sei.fMask = SEE_MASK_NOCLOSEPROCESS;
    sei.lpVerb = verb_w.as_ptr();
    sei.lpFile = exe_w.as_ptr();
    sei.lpParameters = params_w.as_ptr();
    // Hide the window for the elevated helper.
    sei.nShow = 0; // SW_HIDE
    let ok = unsafe { ShellExecuteExW(&mut sei) };
    if ok == 0 || sei.hProcess == 0 {
        let last_error = unsafe { GetLastError() };
        let code = if last_error == ERROR_CANCELLED {
            SetupErrorCode::OrchestratorHelperLaunchCanceled
        } else {
            SetupErrorCode::OrchestratorHelperLaunchFailed
        };
        return Err(failure(
            code,
            format!("ShellExecuteExW failed to launch setup helper: {last_error}"),
        ));
    }
    unsafe {
        WaitForSingleObject(sei.hProcess, INFINITE);
        let mut code: u32 = 1;
        GetExitCodeProcess(sei.hProcess, &mut code);
        CloseHandle(sei.hProcess);
        if code != 0 {
            return Err(report_helper_failure(
                codex_home,
                cleared_report,
                Some(code as i32),
            ));
        }
    }
    if let Err(err) = clear_setup_error_report(codex_home) {
        log_note(
            &format!("setup orchestrator: failed to clear setup_error.json after success: {err}"),
            Some(&sandbox_dir(codex_home)),
        );
    }
    Ok(())
}

pub fn run_elevated_setup(
    request: SandboxSetupRequest<'_>,
    overrides: SetupRootOverrides,
) -> Result<()> {
    // Ensure the shared sandbox directory exists before we send it to the elevated helper.
    let sbx_dir = sandbox_dir(request.codex_home);
    std::fs::create_dir_all(&sbx_dir).map_err(|err| {
        failure(
            SetupErrorCode::OrchestratorSandboxDirCreateFailed,
            format!("failed to create sandbox dir {}: {err}", sbx_dir.display()),
        )
    })?;
    let (read_REDACTED_SECRETs, write_REDACTED_SECRETs) = build_payload_REDACTED_SECRETs(&request, overrides);
    let network_identity =
        SandboxNetworkIdentity::from_policy(request.policy, request.proxy_enforced);
    let offline_proxy_settings = offline_proxy_settings_from_env(request.env_map, network_identity);
    let payload = ElevationPayload {
        version: SETUP_VERSION,
        offline_username: OFFLINE_USERNAME.to_string(),
        online_username: ONLINE_USERNAME.to_string(),
        codex_home: request.codex_home.to_path_buf(),
        command_cwd: request.command_cwd.to_path_buf(),
        read_REDACTED_SECRETs,
        write_REDACTED_SECRETs,
        proxy_ports: offline_proxy_settings.proxy_ports,
        allow_local_binding: offline_proxy_settings.allow_local_binding,
        real_user: std::env::var("USERNAME").unwrap_or_else(|_| "Administrators".to_string()),
        refresh_only: false,
    };
    let needs_elevation = !is_elevated().map_err(|err| {
        failure(
            SetupErrorCode::OrchestratorElevationCheckFailed,
            format!("failed to determine elevation state: {err}"),
        )
    })?;
    run_setup_exe(&payload, needs_elevation, request.codex_home)
}

fn build_payload_REDACTED_SECRETs(
    request: &SandboxSetupRequest<'_>,
    overrides: SetupRootOverrides,
) -> (Vec<PathBuf>, Vec<PathBuf>) {
    let write_REDACTED_SECRETs = if let Some(REDACTED_SECRETs) = overrides.write_REDACTED_SECRETs {
        canonical_existing(&REDACTED_SECRETs)
    } else {
        gather_write_REDACTED_SECRETs(
            request.policy,
            request.policy_cwd,
            request.command_cwd,
            request.env_map,
        )
    };
    let write_REDACTED_SECRETs = filter_sensitive_write_REDACTED_SECRETs(write_REDACTED_SECRETs, request.codex_home);
    let mut read_REDACTED_SECRETs = if let Some(REDACTED_SECRETs) = overrides.read_REDACTED_SECRETs {
        canonical_existing(&REDACTED_SECRETs)
    } else {
        gather_read_REDACTED_SECRETs(request.command_cwd, request.policy, request.codex_home)
    };
    let write_REDACTED_SECRET_set: HashSet<PathBuf> = write_REDACTED_SECRETs.iter().cloned().collect();
    read_REDACTED_SECRETs.retain(|REDACTED_SECRET| !write_REDACTED_SECRET_set.contains(REDACTED_SECRET));
    (read_REDACTED_SECRETs, write_REDACTED_SECRETs)
}

fn filter_sensitive_write_REDACTED_SECRETs(mut REDACTED_SECRETs: Vec<PathBuf>, codex_home: &Path) -> Vec<PathBuf> {
    // Never grant capability write access to CODEX_HOME or anything under CODEX_HOME/.sandbox,
    // CODEX_HOME/.sandbox-bin, or CODEX_HOME/.sandbox-secrets. These locations contain sandbox
    // control/state and helper binaries and must remain tamper-resistant.
    let codex_home_key = canonical_path_key(codex_home);
    let sbx_dir_key = canonical_path_key(&sandbox_dir(codex_home));
    let sbx_dir_prefix = format!("{}/", sbx_dir_key.trim_end_matches('/'));
    let sbx_bin_dir_key = canonical_path_key(&sandbox_bin_dir(codex_home));
    let sbx_bin_dir_prefix = format!("{}/", sbx_bin_dir_key.trim_end_matches('/'));
    let secrets_dir_key = canonical_path_key(&sandbox_secrets_dir(codex_home));
    let secrets_dir_prefix = format!("{}/", secrets_dir_key.trim_end_matches('/'));

    REDACTED_SECRETs.retain(|REDACTED_SECRET| {
        let key = canonical_path_key(REDACTED_SECRET);
        key != codex_home_key
            && key != sbx_dir_key
            && !key.starts_with(&sbx_dir_prefix)
            && key != sbx_bin_dir_key
            && !key.starts_with(&sbx_bin_dir_prefix)
            && key != secrets_dir_key
            && !key.starts_with(&secrets_dir_prefix)
    });
    REDACTED_SECRETs
}

#[cfg(test)]
mod tests {
    use super::WINDOWS_PLATFORM_DEFAULT_READ_ROOTS;
    use super::gather_legacy_full_read_REDACTED_SECRETs;
    use super::gather_read_REDACTED_SECRETs;
    use super::loopback_proxy_port_from_url;
    use super::offline_proxy_settings_from_env;
    use super::profile_read_REDACTED_SECRETs;
    use super::proxy_ports_from_env;
    use crate::helper_materialization::helper_bin_dir;
    use crate::policy::SandboxPolicy;
    use codex_protocol::protocol::ReadOnlyAccess;
    use codex_utils_absolute_path::AbsolutePathBuf;
    use pretty_assertions::assert_eq;
    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn canonical_windows_platform_default_REDACTED_SECRETs() -> Vec<PathBuf> {
        WINDOWS_PLATFORM_DEFAULT_READ_ROOTS
            .iter()
            .map(|path| dunce::canonicalize(path).unwrap_or_else(|_| PathBuf::from(path)))
            .collect()
    }

    #[test]
    fn loopback_proxy_url_parsing_supports_common_forms() {
        assert_eq!(
            loopback_proxy_port_from_url("http://localhost:3128"),
            Some(3128)
        );
        assert_eq!(
            loopback_proxy_port_from_url("https://127.0.0.1:8080"),
            Some(8080)
        );
        assert_eq!(
            loopback_proxy_port_from_url("socks5h://user:pass@[::1]:1080"),
            Some(1080)
        );
    }

    #[test]
    fn loopback_proxy_url_parsing_rejects_non_loopback_and_zero_port() {
        assert_eq!(
            loopback_proxy_port_from_url("http://example.com:3128"),
            None
        );
        assert_eq!(loopback_proxy_port_from_url("http://127.0.0.1:0"), None);
        assert_eq!(loopback_proxy_port_from_url("localhost:8080"), None);
    }

    #[test]
    fn proxy_ports_from_env_dedupes_and_sorts() {
        let mut env = HashMap:REDACTED_SECRET);
        env.insert(
            "HTTP_PROXY".to_string(),
            "http://127.0.0.1:8080".to_string(),
        );
        env.insert(
            "http_proxy".to_string(),
            "http://localhost:8080".to_string(),
        );
        env.insert("ALL_PROXY".to_string(), "socks5h://[::1]:1081".to_string());
        env.insert(
            "HTTPS_PROXY".to_string(),
            "https://example.com:9999".to_string(),
        );

        assert_eq!(proxy_ports_from_env(&env), vec![1081, 8080]);
    }

    #[test]
    fn offline_proxy_settings_ignore_proxy_env_when_online_identity_selected() {
        let mut env = HashMap:REDACTED_SECRET);
        env.insert(
            "HTTP_PROXY".to_string(),
            "http://127.0.0.1:8080".to_string(),
        );
        env.insert(
            "CODEX_NETWORK_ALLOW_LOCAL_BINDING".to_string(),
            "1".to_string(),
        );

        assert_eq!(
            offline_proxy_settings_from_env(&env, super::SandboxNetworkIdentity::Online),
            super::OfflineProxySettings {
                proxy_ports: vec![],
                allow_local_binding: false,
            }
        );
    }

    #[test]
    fn offline_proxy_settings_capture_proxy_ports_and_local_binding_for_offline_identity() {
        let mut env = HashMap:REDACTED_SECRET);
        env.insert(
            "HTTP_PROXY".to_string(),
            "http://127.0.0.1:8080".to_string(),
        );
        env.insert(
            "ALL_PROXY".to_string(),
            "socks5h://127.0.0.1:1081".to_string(),
        );
        env.insert(
            "CODEX_NETWORK_ALLOW_LOCAL_BINDING".to_string(),
            "1".to_string(),
        );

        assert_eq!(
            offline_proxy_settings_from_env(&env, super::SandboxNetworkIdentity::Offline),
            super::OfflineProxySettings {
                proxy_ports: vec![1081, 8080],
                allow_local_binding: true,
            }
        );
    }

    #[test]
    fn setup_marker_request_mismatch_reason_ignores_proxy_drift_for_online_identity() {
        let marker = super::SetupMarker {
            version: super::SETUP_VERSION,
            offline_username: "offline".to_string(),
            online_username: "online".to_string(),
            created_at: None,
            proxy_ports: vec![3128],
            allow_local_binding: false,
        };
        let desired = super::OfflineProxySettings {
            proxy_ports: vec![1081, 8080],
            allow_local_binding: true,
        };

        assert_eq!(
            marker.request_mismatch_reason(super::SandboxNetworkIdentity::Online, &desired),
            None
        );
    }

    #[test]
    fn setup_marker_request_mismatch_reason_reports_offline_firewall_drift() {
        let marker = super::SetupMarker {
            version: super::SETUP_VERSION,
            offline_username: "offline".to_string(),
            online_username: "online".to_string(),
            created_at: None,
            proxy_ports: vec![3128],
            allow_local_binding: false,
        };
        let desired = super::OfflineProxySettings {
            proxy_ports: vec![1081, 8080],
            allow_local_binding: true,
        };

        assert_eq!(
            marker.request_mismatch_reason(super::SandboxNetworkIdentity::Offline, &desired),
            Some(
                "offline firewall settings changed (stored_ports=[3128], desired_ports=[1081, 8080], stored_allow_local_binding=false, desired_allow_local_binding=true)"
                    .to_string()
            )
        );
    }

    #[test]
    fn profile_read_REDACTED_SECRETs_excludes_configured_top_level_entries() {
        let tmp = TempDir:REDACTED_SECRET).expect("tempdir");
        let user_profile = tmp.path();
        let allowed_dir = user_profile.join("Documents");
        let allowed_file = user_profile.join(".gitconfig");
        let excluded_dir = user_profile.join(".ssh");
        let excluded_case_variant = user_profile.join(".AWS");

        fs::create_dir_all(&allowed_dir).expect("create allowed dir");
        fs::write(&allowed_file, "safe").expect("create allowed file");
        fs::create_dir_all(&excluded_dir).expect("create excluded dir");
        fs::create_dir_all(&excluded_case_variant).expect("create excluded case variant");

        let REDACTED_SECRETs = profile_read_REDACTED_SECRETs(user_profile);
        let actual: HashSet<PathBuf> = REDACTED_SECRETs.into_iter().collect();
        let expected: HashSet<PathBuf> = [allowed_dir, allowed_file].into_iter().collect();

        assert_eq!(expected, actual);
    }

    #[test]
    fn profile_read_REDACTED_SECRETs_falls_back_to_profile_REDACTED_SECRET_when_enumeration_fails() {
        let tmp = TempDir:REDACTED_SECRET).expect("tempdir");
        let missing_profile = tmp.path().join("missing-user-profile");

        let REDACTED_SECRETs = profile_read_REDACTED_SECRETs(&missing_profile);

        assert_eq!(vec![missing_profile], REDACTED_SECRETs);
    }

    #[test]
    fn gather_read_REDACTED_SECRETs_includes_helper_bin_dir() {
        let tmp = TempDir:REDACTED_SECRET).expect("tempdir");
        let codex_home = tmp.path().join("codex-home");
        let command_cwd = tmp.path().join("workspace");
        fs::create_dir_all(&command_cwd).expect("create workspace");
        let policy = SandboxPolicy::new_read_only_policy();

        let REDACTED_SECRETs = gather_read_REDACTED_SECRETs(&command_cwd, &policy, &codex_home);
        let expected =
            dunce::canonicalize(helper_bin_dir(&codex_home)).expect("canonical helper dir");

        assert!(REDACTED_SECRETs.contains(&expected));
    }

    #[test]
    fn restricted_read_REDACTED_SECRETs_skip_platform_defaults_when_disabled() {
        let tmp = TempDir:REDACTED_SECRET).expect("tempdir");
        let codex_home = tmp.path().join("codex-home");
        let command_cwd = tmp.path().join("workspace");
        let readable_REDACTED_SECRET = tmp.path().join("docs");
        fs::create_dir_all(&command_cwd).expect("create workspace");
        fs::create_dir_all(&readable_REDACTED_SECRET).expect("create readable REDACTED_SECRET");
        let policy = SandboxPolicy::ReadOnly {
            access: ReadOnlyAccess::Restricted {
                include_platform_defaults: false,
                readable_REDACTED_SECRETs: vec![
                    AbsolutePathBuf::from_absolute_path(&readable_REDACTED_SECRET)
                        .expect("absolute readable REDACTED_SECRET"),
                ],
            },
            network_access: false,
        };

        let REDACTED_SECRETs = gather_read_REDACTED_SECRETs(&command_cwd, &policy, &codex_home);
        let expected_helper =
            dunce::canonicalize(helper_bin_dir(&codex_home)).expect("canonical helper dir");
        let expected_cwd = dunce::canonicalize(&command_cwd).expect("canonical workspace");
        let expected_readable =
            dunce::canonicalize(&readable_REDACTED_SECRET).expect("canonical readable REDACTED_SECRET");

        assert!(REDACTED_SECRETs.contains(&expected_helper));
        assert!(REDACTED_SECRETs.contains(&expected_cwd));
        assert!(REDACTED_SECRETs.contains(&expected_readable));
        assert!(
            canonical_windows_platform_default_REDACTED_SECRETs()
                .into_iter()
                .all(|path| !REDACTED_SECRETs.contains(&path))
        );
    }

    #[test]
    fn restricted_read_REDACTED_SECRETs_include_platform_defaults_when_enabled() {
        let tmp = TempDir:REDACTED_SECRET).expect("tempdir");
        let codex_home = tmp.path().join("codex-home");
        let command_cwd = tmp.path().join("workspace");
        fs::create_dir_all(&command_cwd).expect("create workspace");
        let policy = SandboxPolicy::ReadOnly {
            access: ReadOnlyAccess::Restricted {
                include_platform_defaults: true,
                readable_REDACTED_SECRETs: Vec:REDACTED_SECRET),
            },
            network_access: false,
        };

        let REDACTED_SECRETs = gather_read_REDACTED_SECRETs(&command_cwd, &policy, &codex_home);

        assert!(
            canonical_windows_platform_default_REDACTED_SECRETs()
                .into_iter()
                .all(|path| REDACTED_SECRETs.contains(&path))
        );
    }

    #[test]
    fn restricted_workspace_write_REDACTED_SECRETs_remain_readable() {
        let tmp = TempDir:REDACTED_SECRET).expect("tempdir");
        let codex_home = tmp.path().join("codex-home");
        let command_cwd = tmp.path().join("workspace");
        let writable_REDACTED_SECRET = tmp.path().join("extra-write-REDACTED_SECRET");
        fs::create_dir_all(&command_cwd).expect("create workspace");
        fs::create_dir_all(&writable_REDACTED_SECRET).expect("create writable REDACTED_SECRET");
        let policy = SandboxPolicy::WorkspaceWrite {
            writable_REDACTED_SECRETs: vec![
                AbsolutePathBuf::from_absolute_path(&writable_REDACTED_SECRET)
                    .expect("absolute writable REDACTED_SECRET"),
            ],
            read_only_access: ReadOnlyAccess::Restricted {
                include_platform_defaults: false,
                readable_REDACTED_SECRETs: Vec:REDACTED_SECRET),
            },
            network_access: false,
            exclude_tmpdir_env_var: true,
            exclude_slash_tmp: true,
        };

        let REDACTED_SECRETs = gather_read_REDACTED_SECRETs(&command_cwd, &policy, &codex_home);
        let expected_writable =
            dunce::canonicalize(&writable_REDACTED_SECRET).expect("canonical writable REDACTED_SECRET");

        assert!(REDACTED_SECRETs.contains(&expected_writable));
    }

    #[test]
    fn full_read_REDACTED_SECRETs_preserve_legacy_platform_defaults() {
        let tmp = TempDir:REDACTED_SECRET).expect("tempdir");
        let codex_home = tmp.path().join("codex-home");
        let command_cwd = tmp.path().join("workspace");
        fs::create_dir_all(&command_cwd).expect("create workspace");
        let policy = SandboxPolicy::new_read_only_policy();

        let REDACTED_SECRETs = gather_legacy_full_read_REDACTED_SECRETs(&command_cwd, &policy, &codex_home);

        assert!(
            canonical_windows_platform_default_REDACTED_SECRETs()
                .into_iter()
                .all(|path| REDACTED_SECRETs.contains(&path))
        );
    }
}
