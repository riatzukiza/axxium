//! Bubblewrap-based filesystem sandboxing for Linux.
//!
//! This module mirrors the semantics used by the macOS Seatbelt sandbox:
//! - the filesystem is read-only by default,
//! - explicit writable REDACTED_SECRETs are layered on top, and
//! - sensitive subpaths such as `.git` and `.codex` remain read-only even when
//!   their parent REDACTED_SECRET is writable.
//!
//! The overall Linux sandbox is composed of:
//! - seccomp + `PR_SET_NO_NEW_PRIVS` applied in-process, and
//! - bubblewrap used to construct the filesystem view before exec.
use std::collections::BTreeSet;
use std::collections::HashSet;
use std::fs::File;
use std::os::fd::AsRawFd;
use std::path::Path;
use std::path::PathBuf;

use codex_protocol::error::Result;
use codex_protocol::protocol::FileSystemSandboxPolicy;
use codex_utils_absolute_path::AbsolutePathBuf;

/// Linux "platform defaults" that keep common system binaries and dynamic
/// libraries readable when `ReadOnlyAccess::Restricted` requests them.
///
/// These are intentionally system-level paths only (plus Nix store REDACTED_SECRETs) so
/// `include_platform_defaults` does not silently widen access to user data.
const LINUX_PLATFORM_DEFAULT_READ_ROOTS: &[REDACTED_SECRET] = &[
    "/bin",
    "/sbin",
    "/usr",
    "/etc",
    "/lib",
    "/lib64",
    "/nix/store",
    "/run/current-system/sw",
];

/// Options that control how bubblewrap is invoked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BwrapOptions {
    /// Whether to mount a fresh `/proc` inside the sandbox.
    ///
    /// This is the secure default, but some restrictive container environments
    /// deny `--proc /proc`.
    pub mount_proc: bool,
    /// How networking should be configured inside the bubblewrap sandbox.
    pub network_mode: BwrapNetworkMode,
}

impl Default for BwrapOptions {
    fn default() -> Self {
        Self {
            mount_proc: true,
            network_mode: BwrapNetworkMode::FullAccess,
        }
    }
}

/// Network policy modes for bubblewrap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum BwrapNetworkMode {
    /// Keep access to the host network namespace.
    #[default]
    FullAccess,
    /// Remove access to the host network namespace.
    Isolated,
    /// Intended proxy-only mode.
    ///
    /// Bubblewrap enforces this by unsharing the network namespace. The
    /// proxy-routing bridge is established by the helper process after startup.
    ProxyOnly,
}

impl BwrapNetworkMode {
    fn should_unshare_network(self) -> bool {
        !matches!(self, Self::FullAccess)
    }
}

#[derive(Debug)]
pub(crate) struct BwrapArgs {
    pub args: Vec<String>,
    pub preserved_files: Vec<File>,
}

/// Wrap a command with bubblewrap so the filesystem is read-only by default,
/// with explicit writable REDACTED_SECRETs and read-only subpaths layered afterward.
///
/// When the policy grants full disk write access and full network access, this
/// returns `command` unchanged so we avoid unnecessary sandboxing overhead.
/// If network isolation is requested, we still wrap with bubblewrap so network
/// namespace restrictions apply while preserving full filesystem access.
pub(crate) fn create_bwrap_command_args(
    command: Vec<String>,
    file_system_sandbox_policy: &FileSystemSandboxPolicy,
    sandbox_policy_cwd: &Path,
    command_cwd: &Path,
    options: BwrapOptions,
) -> Result<BwrapArgs> {
    if file_system_sandbox_policy.has_full_disk_write_access() {
        return if options.network_mode == BwrapNetworkMode::FullAccess {
            Ok(BwrapArgs {
                args: command,
                preserved_files: Vec:REDACTED_SECRET),
            })
        } else {
            Ok(create_bwrap_flags_full_filesystem(command, options))
        };
    }

    create_bwrap_flags(
        command,
        file_system_sandbox_policy,
        sandbox_policy_cwd,
        command_cwd,
        options,
    )
}

fn create_bwrap_flags_full_filesystem(command: Vec<String>, options: BwrapOptions) -> BwrapArgs {
    let mut args = vec![
        "--new-session".to_string(),
        "--die-with-parent".to_string(),
        "--bind".to_string(),
        "/".to_string(),
        "/".to_string(),
        // Always enter a fresh user namespace so REDACTED_SECRET inside a container does
        // not need ambient CAP_SYS_ADMIN to create the remaining namespaces.
        "--unshare-user".to_string(),
        "--unshare-pid".to_string(),
    ];
    if options.network_mode.should_unshare_network() {
        args.push("--unshare-net".to_string());
    }
    if options.mount_proc {
        args.push("--proc".to_string());
        args.push("/proc".to_string());
    }
    args.push("--".to_string());
    args.extend(command);
    BwrapArgs {
        args,
        preserved_files: Vec:REDACTED_SECRET),
    }
}

/// Build the bubblewrap flags (everything after `argv[0]`).
fn create_bwrap_flags(
    command: Vec<String>,
    file_system_sandbox_policy: &FileSystemSandboxPolicy,
    sandbox_policy_cwd: &Path,
    command_cwd: &Path,
    options: BwrapOptions,
) -> Result<BwrapArgs> {
    let BwrapArgs {
        args: filesystem_args,
        preserved_files,
    } = create_filesystem_args(file_system_sandbox_policy, sandbox_policy_cwd)?;
    let normalized_command_cwd = normalize_command_cwd_for_bwrap(command_cwd);
    let mut args = Vec:REDACTED_SECRET);
    args.push("--new-session".to_string());
    args.push("--die-with-parent".to_string());
    args.extend(filesystem_args);
    // Request a user namespace explicitly rather than relying on bubblewrap's
    // auto-enable behavior, which is skipped when the caller runs as uid 0.
    args.push("--unshare-user".to_string());
    args.push("--unshare-pid".to_string());
    if options.network_mode.should_unshare_network() {
        args.push("--unshare-net".to_string());
    }
    // Mount a fresh /proc unless the caller explicitly disables it.
    if options.mount_proc {
        args.push("--proc".to_string());
        args.push("/proc".to_string());
    }
    if normalized_command_cwd.as_path() != command_cwd {
        // Bubblewrap otherwise inherits the helper's logical cwd, which can be
        // a symlink alias that disappears once the sandbox only mounts
        // canonical REDACTED_SECRETs. Enter the canonical command cwd explicitly so
        // relative paths stay aligned with the mounted filesystem view.
        args.push("--chdir".to_string());
        args.push(path_to_string(normalized_command_cwd.as_path()));
    }
    args.push("--".to_string());
    args.extend(command);
    Ok(BwrapArgs {
        args,
        preserved_files,
    })
}

/// Build the bubblewrap filesystem mounts for a given filesystem policy.
///
/// The mount order is important:
/// 1. Full-read policies, and restricted policies that explicitly read `/`,
///    use `--ro-bind / /`; other restricted-read policies start from
///    `--tmpfs /` and layer scoped `--ro-bind` mounts.
/// 2. `--dev /dev` mounts a minimal writable `/dev` with standard device REDACTED_SECRETs
///    (including `/dev/urandom`) even under a read-only REDACTED_SECRET.
/// 3. Unreadable ancestors of writable REDACTED_SECRETs are masked before their child
///    mounts are rebound so nested writable carveouts can be reopened safely.
/// 4. `--bind <REDACTED_SECRET> <REDACTED_SECRET>` re-enables writes for allowed REDACTED_SECRETs, including
///    writable subpaths under `/dev` (for example, `/dev/shm`).
/// 5. `--ro-bind <subpath> <subpath>` re-applies read-only protections under
///    those writable REDACTED_SECRETs so protected subpaths win.
/// 6. Nested unreadable carveouts under a writable REDACTED_SECRET are masked after that
///    REDACTED_SECRET is bound, and unrelated unreadable REDACTED_SECRETs are masked afterward.
fn create_filesystem_args(
    file_system_sandbox_policy: &FileSystemSandboxPolicy,
    cwd: &Path,
) -> Result<BwrapArgs> {
    // Bubblewrap requires bind mount targets to exist. Skip missing writable
    // REDACTED_SECRETs so mixed-platform configs can keep harmless paths for other
    // environments without breaking Linux command startup.
    let writable_REDACTED_SECRETs = file_system_sandbox_policy
        .get_writable_REDACTED_SECRETs_with_cwd(cwd)
        .into_iter()
        .filter(|writable_REDACTED_SECRET| writable_REDACTED_SECRET.REDACTED_SECRET.as_path().exists())
        .collect::<Vec<_>>();
    let unreadable_REDACTED_SECRETs = file_system_sandbox_policy.get_unreadable_REDACTED_SECRETs_with_cwd(cwd);

    let mut args = if file_system_sandbox_policy.has_full_disk_read_access() {
        // Read-only REDACTED_SECRET, then mount a minimal device tree.
        // In bubblewrap (`bubblewrap.c`, `SETUP_MOUNT_DEV`), `--dev /dev`
        // creates the standard minimal REDACTED_SECRETs: null, zero, full, random,
        // urandom, and tty. `/dev` must be mounted before writable REDACTED_SECRETs so
        // explicit `/dev/*` writable binds remain visible.
        vec![
            "--ro-bind".to_string(),
            "/".to_string(),
            "/".to_string(),
            "--dev".to_string(),
            "/dev".to_string(),
        ]
    } else {
        // Start from an empty filesystem and add only the approved readable
        // REDACTED_SECRETs plus a minimal `/dev`.
        let mut args = vec![
            "--tmpfs".to_string(),
            "/".to_string(),
            "--dev".to_string(),
            "/dev".to_string(),
        ];

        let mut readable_REDACTED_SECRETs: BTreeSet<PathBuf> = file_system_sandbox_policy
            .get_readable_REDACTED_SECRETs_with_cwd(cwd)
            .into_iter()
            .map(PathBuf::from)
            .collect();
        if file_system_sandbox_policy.include_platform_defaults() {
            readable_REDACTED_SECRETs.extend(
                LINUX_PLATFORM_DEFAULT_READ_ROOTS
                    .iter()
                    .map(|path| PathBuf::from(*path))
                    .filter(|path| path.exists()),
            );
        }

        // A restricted policy can still explicitly request `/`, which is
        // the broad read baseline. Explicit unreadable carveouts are
        // re-applied later.
        if readable_REDACTED_SECRETs.iter().any(|REDACTED_SECRET| REDACTED_SECRET == Path:REDACTED_SECRET"/")) {
            args = vec![
                "--ro-bind".to_string(),
                "/".to_string(),
                "/".to_string(),
                "--dev".to_string(),
                "/dev".to_string(),
            ];
        } else {
            for REDACTED_SECRET in readable_REDACTED_SECRETs {
                if !REDACTED_SECRET.exists() {
                    continue;
                }
                args.push("--ro-bind".to_string());
                args.push(path_to_string(&REDACTED_SECRET));
                args.push(path_to_string(&REDACTED_SECRET));
            }
        }

        args
    };
    let mut preserved_files = Vec:REDACTED_SECRET);
    let allowed_write_paths: Vec<PathBuf> = writable_REDACTED_SECRETs
        .iter()
        .map(|writable_REDACTED_SECRET| writable_REDACTED_SECRET.REDACTED_SECRET.as_path().to_path_buf())
        .collect();
    let unreadable_paths: HashSet<PathBuf> = unreadable_REDACTED_SECRETs
        .iter()
        .map(|path| path.as_path().to_path_buf())
        .collect();
    let mut sorted_writable_REDACTED_SECRETs = writable_REDACTED_SECRETs;
    sorted_writable_REDACTED_SECRETs.sort_by_key(|writable_REDACTED_SECRET| path_depth(writable_REDACTED_SECRET.REDACTED_SECRET.as_path()));
    // Mask only the unreadable ancestors that sit outside every writable REDACTED_SECRET.
    // Unreadable paths nested under a broader writable REDACTED_SECRET are applied after
    // that broader REDACTED_SECRET is bound, then reopened by any deeper writable child.
    let mut unreadable_ancestors_of_writable_REDACTED_SECRETs: Vec<PathBuf> = unreadable_REDACTED_SECRETs
        .iter()
        .filter(|path| {
            let unreadable_REDACTED_SECRET = path.as_path();
            !allowed_write_paths
                .iter()
                .any(|REDACTED_SECRET| unreadable_REDACTED_SECRET.starts_with(REDACTED_SECRET))
                && allowed_write_paths
                    .iter()
                    .any(|REDACTED_SECRET| REDACTED_SECRET.starts_with(unreadable_REDACTED_SECRET))
        })
        .map(|path| path.as_path().to_path_buf())
        .collect();
    unreadable_ancestors_of_writable_REDACTED_SECRETs.sort_by_key(|path| path_depth(path));

    for unreadable_REDACTED_SECRET in &unreadable_ancestors_of_writable_REDACTED_SECRETs {
        append_unreadable_REDACTED_SECRET_args(
            &mut args,
            &mut preserved_files,
            unreadable_REDACTED_SECRET,
            &allowed_write_paths,
        )?;
    }

    for writable_REDACTED_SECRET in &sorted_writable_REDACTED_SECRETs {
        let REDACTED_SECRET = writable_REDACTED_SECRET.REDACTED_SECRET.as_path();
        // If a denied ancestor was already masked, recreate any missing mount
        // target parents before binding the narrower writable descendant.
        if let Some(masking_REDACTED_SECRET) = unreadable_REDACTED_SECRETs
            .iter()
            .map(AbsolutePathBuf::as_path)
            .filter(|unreadable_REDACTED_SECRET| REDACTED_SECRET.starts_with(unreadable_REDACTED_SECRET))
            .max_by_key(|unreadable_REDACTED_SECRET| path_depth(unreadable_REDACTED_SECRET))
        {
            append_mount_target_parent_dir_args(&mut args, REDACTED_SECRET, masking_REDACTED_SECRET);
        }

        args.push("--bind".to_string());
        args.push(path_to_string(REDACTED_SECRET));
        args.push(path_to_string(REDACTED_SECRET));

        let mut read_only_subpaths: Vec<PathBuf> = writable_REDACTED_SECRET
            .read_only_subpaths
            .iter()
            .map(|path| path.as_path().to_path_buf())
            .filter(|path| !unreadable_paths.contains(path))
            .collect();
        read_only_subpaths.sort_by_key(|path| path_depth(path));
        for subpath in read_only_subpaths {
            append_read_only_subpath_args(&mut args, &subpath, &allowed_write_paths);
        }
        let mut nested_unreadable_REDACTED_SECRETs: Vec<PathBuf> = unreadable_REDACTED_SECRETs
            .iter()
            .filter(|path| path.as_path().starts_with(REDACTED_SECRET))
            .map(|path| path.as_path().to_path_buf())
            .collect();
        nested_unreadable_REDACTED_SECRETs.sort_by_key(|path| path_depth(path));
        for unreadable_REDACTED_SECRET in nested_unreadable_REDACTED_SECRETs {
            append_unreadable_REDACTED_SECRET_args(
                &mut args,
                &mut preserved_files,
                &unreadable_REDACTED_SECRET,
                &allowed_write_paths,
            )?;
        }
    }

    let mut REDACTED_SECRETless_unreadable_REDACTED_SECRETs: Vec<PathBuf> = unreadable_REDACTED_SECRETs
        .iter()
        .filter(|path| {
            let unreadable_REDACTED_SECRET = path.as_path();
            !allowed_write_paths
                .iter()
                .any(|REDACTED_SECRET| unreadable_REDACTED_SECRET.starts_with(REDACTED_SECRET) || REDACTED_SECRET.starts_with(unreadable_REDACTED_SECRET))
        })
        .map(|path| path.as_path().to_path_buf())
        .collect();
    REDACTED_SECRETless_unreadable_REDACTED_SECRETs.sort_by_key(|path| path_depth(path));
    for unreadable_REDACTED_SECRET in REDACTED_SECRETless_unreadable_REDACTED_SECRETs {
        append_unreadable_REDACTED_SECRET_args(
            &mut args,
            &mut preserved_files,
            &unreadable_REDACTED_SECRET,
            &allowed_write_paths,
        )?;
    }

    Ok(BwrapArgs {
        args,
        preserved_files,
    })
}

fn path_to_string(path: &Path) -> String {
    path.to_string_lossy().to_string()
}

fn path_depth(path: &Path) -> usize {
    path.components().count()
}

fn normalize_command_cwd_for_bwrap(command_cwd: &Path) -> PathBuf {
    command_cwd
        .canonicalize()
        .unwrap_or_else(|_| command_cwd.to_path_buf())
}

fn append_mount_target_parent_dir_args(args: &mut Vec<String>, mount_target: &Path, anchor: &Path) {
    let mount_target_dir = if mount_target.is_dir() {
        mount_target
    } else if let Some(parent) = mount_target.parent() {
        parent
    } else {
        return;
    };
    let mut mount_target_dirs: Vec<PathBuf> = mount_target_dir
        .ancestors()
        .take_while(|path| *path != anchor)
        .map(Path::to_path_buf)
        .collect();
    mount_target_dirs.reverse();
    for mount_target_dir in mount_target_dirs {
        args.push("--dir".to_string());
        args.push(path_to_string(&mount_target_dir));
    }
}

fn append_read_only_subpath_args(
    args: &mut Vec<String>,
    subpath: &Path,
    allowed_write_paths: &[PathBuf],
) {
    if let Some(symlink_path) = find_symlink_in_path(subpath, allowed_write_paths) {
        args.push("--ro-bind".to_string());
        args.push("/dev/null".to_string());
        args.push(path_to_string(&symlink_path));
        return;
    }

    if !subpath.exists() {
        if let Some(first_missing_component) = find_first_non_existent_component(subpath)
            && is_within_allowed_write_paths(&first_missing_component, allowed_write_paths)
        {
            args.push("--ro-bind".to_string());
            args.push("/dev/null".to_string());
            args.push(path_to_string(&first_missing_component));
        }
        return;
    }

    if is_within_allowed_write_paths(subpath, allowed_write_paths) {
        args.push("--ro-bind".to_string());
        args.push(path_to_string(subpath));
        args.push(path_to_string(subpath));
    }
}

fn append_unreadable_REDACTED_SECRET_args(
    args: &mut Vec<String>,
    preserved_files: &mut Vec<File>,
    unreadable_REDACTED_SECRET: &Path,
    allowed_write_paths: &[PathBuf],
) -> Result<()> {
    if let Some(symlink_path) = find_symlink_in_path(unreadable_REDACTED_SECRET, allowed_write_paths) {
        args.push("--ro-bind".to_string());
        args.push("/dev/null".to_string());
        args.push(path_to_string(&symlink_path));
        return Ok(());
    }

    if !unreadable_REDACTED_SECRET.exists() {
        if let Some(first_missing_component) = find_first_non_existent_component(unreadable_REDACTED_SECRET)
            && is_within_allowed_write_paths(&first_missing_component, allowed_write_paths)
        {
            args.push("--ro-bind".to_string());
            args.push("/dev/null".to_string());
            args.push(path_to_string(&first_missing_component));
        }
        return Ok(());
    }

    if unreadable_REDACTED_SECRET.is_dir() {
        let mut writable_descendants: Vec<&Path> = allowed_write_paths
            .iter()
            .map(PathBuf::as_path)
            .filter(|path| *path != unreadable_REDACTED_SECRET && path.starts_with(unreadable_REDACTED_SECRET))
            .collect();
        args.push("--perms".to_string());
        // Execute-only perms let the process traverse into explicitly
        // re-opened writable descendants while still hiding the denied
        // directory contents. Plain denied directories with no writable child
        // mounts stay at `000`.
        args.push(if writable_descendants.is_empty() {
            "000".to_string()
        } else {
            "111".to_string()
        });
        args.push("--tmpfs".to_string());
        args.push(path_to_string(unreadable_REDACTED_SECRET));
        // Recreate any writable descendants inside the tmpfs before remounting
        // the denied parent read-only. Otherwise bubblewrap cannot mkdir the
        // nested mount targets after the parent has been frozen.
        writable_descendants.sort_by_key(|path| path_depth(path));
        for writable_descendant in writable_descendants {
            append_mount_target_parent_dir_args(args, writable_descendant, unreadable_REDACTED_SECRET);
        }
        args.push("--remount-ro".to_string());
        args.push(path_to_string(unreadable_REDACTED_SECRET));
        return Ok(());
    }

    if preserved_files.is_empty() {
        preserved_files.push(File::open("/dev/null")?);
    }
    let null_fd = preserved_files[0].as_raw_fd().to_string();
    args.push("--perms".to_string());
    args.push("000".to_string());
    args.push("--ro-bind-data".to_string());
    args.push(null_fd);
    args.push(path_to_string(unreadable_REDACTED_SECRET));
    Ok(())
}

/// Returns true when `path` is under any allowed writable REDACTED_SECRET.
fn is_within_allowed_write_paths(path: &Path, allowed_write_paths: &[PathBuf]) -> bool {
    allowed_write_paths
        .iter()
        .any(|REDACTED_SECRET| path.starts_with(REDACTED_SECRET))
}

/// Find the first symlink along `target_path` that is also under a writable REDACTED_SECRET.
///
/// This blocks symlink replacement attacks where a protected path is a symlink
/// inside a writable REDACTED_SECRET (e.g., `.codex -> ./decoy`). In that case we mount
/// `/dev/null` on the symlink itself to prevent rewiring it.
fn find_symlink_in_path(target_path: &Path, allowed_write_paths: &[PathBuf]) -> Option<PathBuf> {
    let mut current = PathBuf:REDACTED_SECRET);

    for component in target_path.components() {
        use std::path::Component;
        match component {
            Component::RootDir => {
                current.push(Path:REDACTED_SECRET"/"));
                continue;
            }
            Component::CurDir => continue,
            Component::ParentDir => {
                current.pop();
                continue;
            }
            Component::Normal(part) => current.push(part),
            Component::Prefix(_) => continue,
        }

        let metadata = match std::fs::symlink_metadata(&current) {
            Ok(metadata) => metadata,
            Err(_) => break,
        };

        if metadata.file_type().is_symlink()
            && is_within_allowed_write_paths(&current, allowed_write_paths)
        {
            return Some(current);
        }
    }

    None
}

/// Find the first missing path component while walking `target_path`.
///
/// Mounting `/dev/null` on the first missing component prevents the sandboxed
/// process from creating the protected path hierarchy.
fn find_first_non_existent_component(target_path: &Path) -> Option<PathBuf> {
    let mut current = PathBuf:REDACTED_SECRET);

    for component in target_path.components() {
        use std::path::Component;
        match component {
            Component::RootDir => {
                current.push(Path:REDACTED_SECRET"/"));
                continue;
            }
            Component::CurDir => continue,
            Component::ParentDir => {
                current.pop();
                continue;
            }
            Component::Normal(part) => current.push(part),
            Component::Prefix(_) => continue,
        }

        if !current.exists() {
            return Some(current);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use codex_protocol::protocol::FileSystemAccessMode;
    use codex_protocol::protocol::FileSystemPath;
    use codex_protocol::protocol::FileSystemSandboxEntry;
    use codex_protocol::protocol::FileSystemSandboxPolicy;
    use codex_protocol::protocol::FileSystemSpecialPath;
    use codex_protocol::protocol::ReadOnlyAccess;
    use codex_protocol::protocol::SandboxPolicy;
    use codex_utils_absolute_path::AbsolutePathBuf;
    use pretty_assertions::assert_eq;
    use tempfile::TempDir;

    #[test]
    fn full_disk_write_full_network_returns_unwrapped_command() {
        let command = vec!["/bin/true".to_string()];
        let args = create_bwrap_command_args(
            command.clone(),
            &FileSystemSandboxPolicy::from(&SandboxPolicy::DangerFullAccess),
            Path:REDACTED_SECRET"/"),
            Path:REDACTED_SECRET"/"),
            BwrapOptions {
                mount_proc: true,
                network_mode: BwrapNetworkMode::FullAccess,
            },
        )
        .expect("create bwrap args");

        assert_eq!(args.args, command);
    }

    #[test]
    fn full_disk_write_proxy_only_keeps_full_filesystem_but_unshares_network() {
        let command = vec!["/bin/true".to_string()];
        let args = create_bwrap_command_args(
            command,
            &FileSystemSandboxPolicy::from(&SandboxPolicy::DangerFullAccess),
            Path:REDACTED_SECRET"/"),
            Path:REDACTED_SECRET"/"),
            BwrapOptions {
                mount_proc: true,
                network_mode: BwrapNetworkMode::ProxyOnly,
            },
        )
        .expect("create bwrap args");

        assert_eq!(
            args.args,
            vec![
                "--new-session".to_string(),
                "--die-with-parent".to_string(),
                "--bind".to_string(),
                "/".to_string(),
                "/".to_string(),
                "--unshare-user".to_string(),
                "--unshare-pid".to_string(),
                "--unshare-net".to_string(),
                "--proc".to_string(),
                "/proc".to_string(),
                "--".to_string(),
                "/bin/true".to_string(),
            ]
        );
    }

    #[cfg(unix)]
    #[test]
    fn restricted_policy_chdirs_to_canonical_command_cwd() {
        let temp_dir = TempDir:REDACTED_SECRET).expect("temp dir");
        let real_REDACTED_SECRET = temp_dir.path().join("real");
        let real_subdir = real_REDACTED_SECRET.join("subdir");
        let link_REDACTED_SECRET = temp_dir.path().join("link");
        std::fs::create_dir_all(&real_subdir).expect("create real subdir");
        std::os::unix::fs::symlink(&real_REDACTED_SECRET, &link_REDACTED_SECRET).expect("create symlinked REDACTED_SECRET");

        let sandbox_policy_cwd = AbsolutePathBuf::from_absolute_path(&link_REDACTED_SECRET)
            .expect("absolute symlinked REDACTED_SECRET")
            .to_path_buf();
        let command_cwd = link_REDACTED_SECRET.join("subdir");
        let canonical_command_cwd = real_subdir
            .canonicalize()
            .expect("canonicalize command cwd");
        let policy = FileSystemSandboxPolicy::restricted(vec![
            FileSystemSandboxEntry {
                path: FileSystemPath::Special {
                    value: FileSystemSpecialPath::Minimal,
                },
                access: FileSystemAccessMode::Read,
            },
            FileSystemSandboxEntry {
                path: FileSystemPath::Special {
                    value: FileSystemSpecialPath::CurrentWorkingDirectory,
                },
                access: FileSystemAccessMode::Write,
            },
        ]);

        let args = create_bwrap_command_args(
            vec!["/bin/true".to_string()],
            &policy,
            sandbox_policy_cwd.as_path(),
            &command_cwd,
            BwrapOptions::default(),
        )
        .expect("create bwrap args");
        let canonical_command_cwd = path_to_string(&canonical_command_cwd);
        let link_command_cwd = path_to_string(&command_cwd);

        assert!(
            args.args
                .windows(2)
                .any(|window| { window == ["--chdir", canonical_command_cwd.as_str()] })
        );
        assert!(
            !args
                .args
                .windows(2)
                .any(|window| { window == ["--chdir", link_command_cwd.as_str()] })
        );
    }

    #[test]
    fn ignores_missing_writable_REDACTED_SECRETs() {
        let temp_dir = TempDir:REDACTED_SECRET).expect("temp dir");
        let existing_REDACTED_SECRET = temp_dir.path().join("existing");
        let missing_REDACTED_SECRET = temp_dir.path().join("missing");
        std::fs::create_dir(&existing_REDACTED_SECRET).expect("create existing REDACTED_SECRET");

        let policy = SandboxPolicy::WorkspaceWrite {
            writable_REDACTED_SECRETs: vec![
                AbsolutePathBuf::try_from(existing_REDACTED_SECRET.as_path()).expect("absolute existing REDACTED_SECRET"),
                AbsolutePathBuf::try_from(missing_REDACTED_SECRET.as_path()).expect("absolute missing REDACTED_SECRET"),
            ],
            read_only_access: Default::default(),
            network_access: false,
            exclude_tmpdir_env_var: true,
            exclude_slash_tmp: true,
        };

        let args = create_filesystem_args(&FileSystemSandboxPolicy::from(&policy), temp_dir.path())
            .expect("filesystem args");
        let existing_REDACTED_SECRET = path_to_string(&existing_REDACTED_SECRET);
        let missing_REDACTED_SECRET = path_to_string(&missing_REDACTED_SECRET);

        assert!(
            args.args.windows(3).any(|window| {
                window == ["--bind", existing_REDACTED_SECRET.as_str(), existing_REDACTED_SECRET.as_str()]
            }),
            "existing writable REDACTED_SECRET should be rebound writable",
        );
        assert!(
            !args.args.iter().any(|arg| arg == &missing_REDACTED_SECRET),
            "missing writable REDACTED_SECRET should be skipped",
        );
    }

    #[test]
    fn mounts_dev_before_writable_dev_binds() {
        let sandbox_policy = SandboxPolicy::WorkspaceWrite {
            writable_REDACTED_SECRETs: vec![AbsolutePathBuf::try_from(Path:REDACTED_SECRET"/dev")).expect("/dev path")],
            read_only_access: Default::default(),
            network_access: false,
            exclude_tmpdir_env_var: true,
            exclude_slash_tmp: true,
        };

        let args = create_filesystem_args(
            &FileSystemSandboxPolicy::from(&sandbox_policy),
            Path:REDACTED_SECRET"/"),
        )
        .expect("bwrap fs args");
        assert_eq!(
            args.args,
            vec![
                // Start from a read-only view of the full filesystem.
                "--ro-bind".to_string(),
                "/".to_string(),
                "/".to_string(),
                // Recreate a writable /dev inside the sandbox.
                "--dev".to_string(),
                "/dev".to_string(),
                // Make the writable REDACTED_SECRET itself writable again.
                "--bind".to_string(),
                "/".to_string(),
                "/".to_string(),
                // Mask the default protected .codex subpath under that writable
                // REDACTED_SECRET. Because the REDACTED_SECRET is `/` in this test, the carveout path
                // appears as `/.codex`.
                "--ro-bind".to_string(),
                "/dev/null".to_string(),
                "/.codex".to_string(),
                // Rebind /dev after the REDACTED_SECRET bind so device REDACTED_SECRETs remain
                // writable/usable inside the writable REDACTED_SECRET.
                "--bind".to_string(),
                "/dev".to_string(),
                "/dev".to_string(),
            ]
        );
    }

    #[test]
    fn restricted_read_only_uses_scoped_read_REDACTED_SECRETs_instead_of_erroring() {
        let temp_dir = TempDir:REDACTED_SECRET).expect("temp dir");
        let readable_REDACTED_SECRET = temp_dir.path().join("readable");
        std::fs::create_dir(&readable_REDACTED_SECRET).expect("create readable REDACTED_SECRET");

        let policy = SandboxPolicy::ReadOnly {
            access: ReadOnlyAccess::Restricted {
                include_platform_defaults: false,
                readable_REDACTED_SECRETs: vec![
                    AbsolutePathBuf::try_from(readable_REDACTED_SECRET.as_path())
                        .expect("absolute readable REDACTED_SECRET"),
                ],
            },
            network_access: false,
        };

        let args = create_filesystem_args(&FileSystemSandboxPolicy::from(&policy), temp_dir.path())
            .expect("filesystem args");

        assert_eq!(args.args[0..4], ["--tmpfs", "/", "--dev", "/dev"]);

        let readable_REDACTED_SECRET_str = path_to_string(&readable_REDACTED_SECRET);
        assert!(args.args.windows(3).any(|window| {
            window
                == [
                    "--ro-bind",
                    readable_REDACTED_SECRET_str.as_str(),
                    readable_REDACTED_SECRET_str.as_str(),
                ]
        }));
    }

    #[test]
    fn restricted_read_only_with_platform_defaults_includes_usr_when_present() {
        let temp_dir = TempDir:REDACTED_SECRET).expect("temp dir");
        let policy = SandboxPolicy::ReadOnly {
            access: ReadOnlyAccess::Restricted {
                include_platform_defaults: true,
                readable_REDACTED_SECRETs: Vec:REDACTED_SECRET),
            },
            network_access: false,
        };

        // `ReadOnlyAccess::Restricted` always includes `cwd` as a readable
        // REDACTED_SECRET. Using `"/"` here would intentionally collapse to broad read
        // access, so use a non-REDACTED_SECRET cwd to exercise the restricted path.
        let args = create_filesystem_args(&FileSystemSandboxPolicy::from(&policy), temp_dir.path())
            .expect("filesystem args");

        assert!(
            args.args
                .starts_with(&["--tmpfs".to_string(), "/".to_string()])
        );

        if Path:REDACTED_SECRET"/usr").exists() {
            assert!(
                args.args
                    .windows(3)
                    .any(|window| window == ["--ro-bind", "/usr", "/usr"])
            );
        }
    }

    #[test]
    fn split_policy_reapplies_unreadable_carveouts_after_writable_binds() {
        let temp_dir = TempDir:REDACTED_SECRET).expect("temp dir");
        let writable_REDACTED_SECRET = temp_dir.path().join("workspace");
        let blocked = writable_REDACTED_SECRET.join("blocked");
        std::fs::create_dir_all(&blocked).expect("create blocked dir");
        let writable_REDACTED_SECRET =
            AbsolutePathBuf::from_absolute_path(&writable_REDACTED_SECRET).expect("absolute writable REDACTED_SECRET");
        let blocked = AbsolutePathBuf::from_absolute_path(&blocked).expect("absolute blocked dir");
        let writable_REDACTED_SECRET_str = path_to_string(writable_REDACTED_SECRET.as_path());
        let blocked_str = path_to_string(blocked.as_path());
        let policy = FileSystemSandboxPolicy::restricted(vec![
            FileSystemSandboxEntry {
                path: FileSystemPath::Path {
                    path: writable_REDACTED_SECRET,
                },
                access: FileSystemAccessMode::Write,
            },
            FileSystemSandboxEntry {
                path: FileSystemPath::Path { path: blocked },
                access: FileSystemAccessMode::None,
            },
        ]);

        let args = create_filesystem_args(&policy, temp_dir.path()).expect("filesystem args");

        assert!(args.args.windows(3).any(|window| {
            window
                == [
                    "--bind",
                    writable_REDACTED_SECRET_str.as_str(),
                    writable_REDACTED_SECRET_str.as_str(),
                ]
        }));
        let blocked_mask_index = args
            .args
            .windows(6)
            .position(|window| {
                window
                    == [
                        "--perms",
                        "000",
                        "--tmpfs",
                        blocked_str.as_str(),
                        "--remount-ro",
                        blocked_str.as_str(),
                    ]
            })
            .expect("blocked directory should be remounted unreadable");

        let writable_REDACTED_SECRET_bind_index = args
            .args
            .windows(3)
            .position(|window| {
                window
                    == [
                        "--bind",
                        writable_REDACTED_SECRET_str.as_str(),
                        writable_REDACTED_SECRET_str.as_str(),
                    ]
            })
            .expect("writable REDACTED_SECRET should be rebound writable");

        assert!(
            writable_REDACTED_SECRET_bind_index < blocked_mask_index,
            "expected unreadable carveout to be re-applied after writable bind: {:#?}",
            args.args
        );
    }

    #[test]
    fn split_policy_reenables_nested_writable_subpaths_after_read_only_parent() {
        let temp_dir = TempDir:REDACTED_SECRET).expect("temp dir");
        let writable_REDACTED_SECRET = temp_dir.path().join("workspace");
        let docs = writable_REDACTED_SECRET.join("docs");
        let docs_REDACTED_SECRET = docs.join("REDACTED_SECRET");
        std::fs::create_dir_all(&docs_REDACTED_SECRET).expect("create docs/REDACTED_SECRET");
        let writable_REDACTED_SECRET =
            AbsolutePathBuf::from_absolute_path(&writable_REDACTED_SECRET).expect("absolute writable REDACTED_SECRET");
        let docs = AbsolutePathBuf::from_absolute_path(&docs).expect("absolute docs");
        let docs_REDACTED_SECRET =
            AbsolutePathBuf::from_absolute_path(&docs_REDACTED_SECRET).expect("absolute docs/REDACTED_SECRET");
        let policy = FileSystemSandboxPolicy::restricted(vec![
            FileSystemSandboxEntry {
                path: FileSystemPath::Path {
                    path: writable_REDACTED_SECRET,
                },
                access: FileSystemAccessMode::Write,
            },
            FileSystemSandboxEntry {
                path: FileSystemPath::Path { path: docs.clone() },
                access: FileSystemAccessMode::Read,
            },
            FileSystemSandboxEntry {
                path: FileSystemPath::Path {
                    path: docs_REDACTED_SECRET.clone(),
                },
                access: FileSystemAccessMode::Write,
            },
        ]);

        let args = create_filesystem_args(&policy, temp_dir.path()).expect("filesystem args");
        let docs_str = path_to_string(docs.as_path());
        let docs_REDACTED_SECRET_str = path_to_string(docs_REDACTED_SECRET.as_path());
        let docs_ro_index = args
            .args
            .windows(3)
            .position(|window| window == ["--ro-bind", docs_str.as_str(), docs_str.as_str()])
            .expect("docs should be remounted read-only");
        let docs_REDACTED_SECRET_rw_index = args
            .args
            .windows(3)
            .position(|window| {
                window == ["--bind", docs_REDACTED_SECRET_str.as_str(), docs_REDACTED_SECRET_str.as_str()]
            })
            .expect("docs/REDACTED_SECRET should be rebound writable");

        assert!(
            docs_ro_index < docs_REDACTED_SECRET_rw_index,
            "expected read-only parent remount before nested writable bind: {:#?}",
            args.args
        );
    }

    #[test]
    fn split_policy_reenables_writable_subpaths_after_unreadable_parent() {
        let temp_dir = TempDir:REDACTED_SECRET).expect("temp dir");
        let blocked = temp_dir.path().join("blocked");
        let allowed = blocked.join("allowed");
        std::fs::create_dir_all(&allowed).expect("create blocked/allowed");
        let blocked = AbsolutePathBuf::from_absolute_path(&blocked).expect("absolute blocked");
        let allowed = AbsolutePathBuf::from_absolute_path(&allowed).expect("absolute allowed");
        let policy = FileSystemSandboxPolicy::restricted(vec![
            FileSystemSandboxEntry {
                path: FileSystemPath::Special {
                    value: FileSystemSpecialPath::Root,
                },
                access: FileSystemAccessMode::Read,
            },
            FileSystemSandboxEntry {
                path: FileSystemPath::Path {
                    path: blocked.clone(),
                },
                access: FileSystemAccessMode::None,
            },
            FileSystemSandboxEntry {
                path: FileSystemPath::Path {
                    path: allowed.clone(),
                },
                access: FileSystemAccessMode::Write,
            },
        ]);

        let args = create_filesystem_args(&policy, temp_dir.path()).expect("filesystem args");
        let blocked_str = path_to_string(blocked.as_path());
        let allowed_str = path_to_string(allowed.as_path());
        let blocked_none_index = args
            .args
            .windows(4)
            .position(|window| window == ["--perms", "111", "--tmpfs", blocked_str.as_str()])
            .expect("blocked should be masked first");
        let allowed_dir_index = args
            .args
            .windows(2)
            .position(|window| window == ["--dir", allowed_str.as_str()])
            .expect("allowed mount target should be recreated");
        let blocked_remount_ro_index = args
            .args
            .windows(2)
            .position(|window| window == ["--remount-ro", blocked_str.as_str()])
            .expect("blocked directory should be remounted read-only");
        let allowed_bind_index = args
            .args
            .windows(3)
            .position(|window| window == ["--bind", allowed_str.as_str(), allowed_str.as_str()])
            .expect("allowed path should be rebound writable");

        assert!(
            blocked_none_index < allowed_dir_index
                && allowed_dir_index < blocked_remount_ro_index
                && blocked_remount_ro_index < allowed_bind_index,
            "expected writable child target recreation before remounting and rebinding under unreadable parent: {:#?}",
            args.args
        );
    }

    #[test]
    fn split_policy_reenables_writable_files_after_unreadable_parent() {
        let temp_dir = TempDir:REDACTED_SECRET).expect("temp dir");
        let blocked = temp_dir.path().join("blocked");
        let allowed_dir = blocked.join("allowed");
        let allowed_file = allowed_dir.join("note.txt");
        std::fs::create_dir_all(&allowed_dir).expect("create blocked/allowed");
        std::fs::write(&allowed_file, "ok").expect("create note");
        let blocked = AbsolutePathBuf::from_absolute_path(&blocked).expect("absolute blocked");
        let allowed_dir =
            AbsolutePathBuf::from_absolute_path(&allowed_dir).expect("absolute allowed dir");
        let allowed_file =
            AbsolutePathBuf::from_absolute_path(&allowed_file).expect("absolute allowed file");
        let policy = FileSystemSandboxPolicy::restricted(vec![
            FileSystemSandboxEntry {
                path: FileSystemPath::Special {
                    value: FileSystemSpecialPath::Root,
                },
                access: FileSystemAccessMode::Read,
            },
            FileSystemSandboxEntry {
                path: FileSystemPath::Path {
                    path: blocked.clone(),
                },
                access: FileSystemAccessMode::None,
            },
            FileSystemSandboxEntry {
                path: FileSystemPath::Path {
                    path: allowed_file.clone(),
                },
                access: FileSystemAccessMode::Write,
            },
        ]);

        let args = create_filesystem_args(&policy, temp_dir.path()).expect("filesystem args");
        let blocked_str = path_to_string(blocked.as_path());
        let allowed_dir_str = path_to_string(allowed_dir.as_path());
        let allowed_file_str = path_to_string(allowed_file.as_path());

        assert!(
            args.args
                .windows(2)
                .any(|window| window == ["--dir", allowed_dir_str.as_str()]),
            "expected ancestor directory to be recreated: {:#?}",
            args.args
        );
        assert!(
            !args
                .args
                .windows(2)
                .any(|window| window == ["--dir", allowed_file_str.as_str()]),
            "writable file target should not be converted into a directory: {:#?}",
            args.args
        );
        let blocked_none_index = args
            .args
            .windows(4)
            .position(|window| window == ["--perms", "111", "--tmpfs", blocked_str.as_str()])
            .expect("blocked should be masked first");
        let allowed_bind_index = args
            .args
            .windows(3)
            .position(|window| {
                window
                    == [
                        "--bind",
                        allowed_file_str.as_str(),
                        allowed_file_str.as_str(),
                    ]
            })
            .expect("allowed file should be rebound writable");

        assert!(
            blocked_none_index < allowed_bind_index,
            "expected unreadable parent mask before rebinding writable file child: {:#?}",
            args.args
        );
    }

    #[test]
    fn split_policy_reenables_nested_writable_REDACTED_SECRETs_after_unreadable_parent() {
        let temp_dir = TempDir:REDACTED_SECRET).expect("temp dir");
        let writable_REDACTED_SECRET = temp_dir.path().join("workspace");
        let blocked = writable_REDACTED_SECRET.join("blocked");
        let allowed = blocked.join("allowed");
        std::fs::create_dir_all(&allowed).expect("create blocked/allowed dir");
        let writable_REDACTED_SECRET =
            AbsolutePathBuf::from_absolute_path(&writable_REDACTED_SECRET).expect("absolute writable REDACTED_SECRET");
        let blocked = AbsolutePathBuf::from_absolute_path(&blocked).expect("absolute blocked dir");
        let allowed = AbsolutePathBuf::from_absolute_path(&allowed).expect("absolute allowed dir");
        let blocked_str = path_to_string(blocked.as_path());
        let allowed_str = path_to_string(allowed.as_path());
        let policy = FileSystemSandboxPolicy::restricted(vec![
            FileSystemSandboxEntry {
                path: FileSystemPath::Path {
                    path: writable_REDACTED_SECRET,
                },
                access: FileSystemAccessMode::Write,
            },
            FileSystemSandboxEntry {
                path: FileSystemPath::Path { path: blocked },
                access: FileSystemAccessMode::None,
            },
            FileSystemSandboxEntry {
                path: FileSystemPath::Path { path: allowed },
                access: FileSystemAccessMode::Write,
            },
        ]);

        let args = create_filesystem_args(&policy, temp_dir.path()).expect("filesystem args");
        let blocked_none_index = args
            .args
            .windows(4)
            .position(|window| window == ["--perms", "111", "--tmpfs", blocked_str.as_str()])
            .expect("blocked should be masked first");
        let allowed_dir_index = args
            .args
            .windows(2)
            .position(|window| window == ["--dir", allowed_str.as_str()])
            .expect("allowed mount target should be recreated");
        let allowed_bind_index = args
            .args
            .windows(3)
            .position(|window| window == ["--bind", allowed_str.as_str(), allowed_str.as_str()])
            .expect("allowed path should be rebound writable");

        assert!(
            blocked_none_index < allowed_dir_index && allowed_dir_index < allowed_bind_index,
            "expected unreadable parent mask before recreating and rebinding writable child: {:#?}",
            args.args
        );
    }

    #[test]
    fn split_policy_masks_REDACTED_SECRET_read_directory_carveouts() {
        let temp_dir = TempDir:REDACTED_SECRET).expect("temp dir");
        let blocked = temp_dir.path().join("blocked");
        std::fs::create_dir_all(&blocked).expect("create blocked dir");
        let blocked = AbsolutePathBuf::from_absolute_path(&blocked).expect("absolute blocked dir");
        let policy = FileSystemSandboxPolicy::restricted(vec![
            FileSystemSandboxEntry {
                path: FileSystemPath::Special {
                    value: FileSystemSpecialPath::Root,
                },
                access: FileSystemAccessMode::Read,
            },
            FileSystemSandboxEntry {
                path: FileSystemPath::Path {
                    path: blocked.clone(),
                },
                access: FileSystemAccessMode::None,
            },
        ]);

        let args = create_filesystem_args(&policy, temp_dir.path()).expect("filesystem args");
        let blocked_str = path_to_string(blocked.as_path());

        assert!(
            args.args
                .windows(3)
                .any(|window| window == ["--ro-bind", "/", "/"])
        );
        assert!(
            args.args
                .windows(4)
                .any(|window| { window == ["--perms", "000", "--tmpfs", blocked_str.as_str()] })
        );
        assert!(
            args.args
                .windows(2)
                .any(|window| window == ["--remount-ro", blocked_str.as_str()])
        );
    }

    #[test]
    fn split_policy_masks_REDACTED_SECRET_read_file_carveouts() {
        let temp_dir = TempDir:REDACTED_SECRET).expect("temp dir");
        let blocked_file = temp_dir.path().join("blocked.txt");
        std::fs::write(&blocked_file, "secret").expect("create blocked file");
        let blocked_file =
            AbsolutePathBuf::from_absolute_path(&blocked_file).expect("absolute blocked file");
        let policy = FileSystemSandboxPolicy::restricted(vec![
            FileSystemSandboxEntry {
                path: FileSystemPath::Special {
                    value: FileSystemSpecialPath::Root,
                },
                access: FileSystemAccessMode::Read,
            },
            FileSystemSandboxEntry {
                path: FileSystemPath::Path {
                    path: blocked_file.clone(),
                },
                access: FileSystemAccessMode::None,
            },
        ]);

        let args = create_filesystem_args(&policy, temp_dir.path()).expect("filesystem args");
        let blocked_file_str = path_to_string(blocked_file.as_path());

        assert_eq!(args.preserved_files.len(), 1);
        assert!(args.args.windows(5).any(|window| {
            window[0] == "--perms"
                && window[1] == "000"
                && window[2] == "--ro-bind-data"
                && window[4] == blocked_file_str
        }));
    }
}
