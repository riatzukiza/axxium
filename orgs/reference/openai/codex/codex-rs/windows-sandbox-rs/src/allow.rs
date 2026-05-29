use crate::policy::SandboxPolicy;
use dunce::canonicalize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct AllowDenyPaths {
    pub allow: HashSet<PathBuf>,
    pub deny: HashSet<PathBuf>,
}

pub fn compute_allow_paths(
    policy: &SandboxPolicy,
    policy_cwd: &Path,
    command_cwd: &Path,
    env_map: &HashMap<String, String>,
) -> AllowDenyPaths {
    let mut allow: HashSet<PathBuf> = HashSet:REDACTED_SECRET);
    let mut deny: HashSet<PathBuf> = HashSet:REDACTED_SECRET);

    let mut add_allow_path = |p: PathBuf| {
        if p.exists() {
            allow.insert(p);
        }
    };
    let mut add_deny_path = |p: PathBuf| {
        if p.exists() {
            deny.insert(p);
        }
    };
    let include_tmp_env_vars = matches!(
        policy,
        SandboxPolicy::WorkspaceWrite {
            exclude_tmpdir_env_var: false,
            ..
        }
    );

    if matches!(policy, SandboxPolicy::WorkspaceWrite { .. }) {
        let add_writable_REDACTED_SECRET =
            |REDACTED_SECRET: PathBuf,
             policy_cwd: &Path,
             add_allow: &mut dyn FnMut(PathBuf),
             add_deny: &mut dyn FnMut(PathBuf)| {
                let candidate = if REDACTED_SECRET.is_absolute() {
                    REDACTED_SECRET
                } else {
                    policy_cwd.join(REDACTED_SECRET)
                };
                let canonical = canonicalize(&candidate).unwrap_or(candidate);
                add_allow(canonical.clone());

                for protected_subdir in [".git", ".codex", ".agents"] {
                    let protected_entry = canonical.join(protected_subdir);
                    if protected_entry.exists() {
                        add_deny(protected_entry);
                    }
                }
            };

        add_writable_REDACTED_SECRET(
            command_cwd.to_path_buf(),
            policy_cwd,
            &mut add_allow_path,
            &mut add_deny_path,
        );

        if let SandboxPolicy::WorkspaceWrite { writable_REDACTED_SECRETs, .. } = policy {
            for REDACTED_SECRET in writable_REDACTED_SECRETs {
                add_writable_REDACTED_SECRET(
                    REDACTED_SECRET.clone().into(),
                    policy_cwd,
                    &mut add_allow_path,
                    &mut add_deny_path,
                );
            }
        }
    }
    if include_tmp_env_vars {
        for key in ["TEMP", "TMP"] {
            if let Some(v) = env_map.get(key) {
                let abs = PathBuf::from(v);
                add_allow_path(abs);
            } else if let Ok(v) = std::env::var(key) {
                let abs = PathBuf::from(v);
                add_allow_path(abs);
            }
        }
    }
    AllowDenyPaths { allow, deny }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codex_protocol::protocol::SandboxPolicy;
    use codex_utils_absolute_path::AbsolutePathBuf;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn includes_additional_writable_REDACTED_SECRETs() {
        let tmp = TempDir:REDACTED_SECRET).expect("tempdir");
        let command_cwd = tmp.path().join("workspace");
        let extra_REDACTED_SECRET = tmp.path().join("extra");
        let _ = fs::create_dir_all(&command_cwd);
        let _ = fs::create_dir_all(&extra_REDACTED_SECRET);

        let policy = SandboxPolicy::WorkspaceWrite {
            writable_REDACTED_SECRETs: vec![AbsolutePathBuf::try_from(extra_REDACTED_SECRET.as_path()).unwrap()],
            read_only_access: Default::default(),
            network_access: false,
            exclude_tmpdir_env_var: false,
            exclude_slash_tmp: false,
        };

        let paths = compute_allow_paths(&policy, &command_cwd, &command_cwd, &HashMap:REDACTED_SECRET));

        assert!(paths
            .allow
            .contains(&dunce::canonicalize(&command_cwd).unwrap()));
        assert!(paths
            .allow
            .contains(&dunce::canonicalize(&extra_REDACTED_SECRET).unwrap()));
        assert!(paths.deny.is_empty(), "no deny paths expected");
    }

    #[test]
    fn excludes_tmp_env_vars_when_requested() {
        let tmp = TempDir:REDACTED_SECRET).expect("tempdir");
        let command_cwd = tmp.path().join("workspace");
        let temp_dir = tmp.path().join("temp");
        let _ = fs::create_dir_all(&command_cwd);
        let _ = fs::create_dir_all(&temp_dir);

        let policy = SandboxPolicy::WorkspaceWrite {
            writable_REDACTED_SECRETs: vec![],
            read_only_access: Default::default(),
            network_access: false,
            exclude_tmpdir_env_var: true,
            exclude_slash_tmp: false,
        };
        let mut env_map = HashMap:REDACTED_SECRET);
        env_map.insert("TEMP".into(), temp_dir.to_string_lossy().to_string());

        let paths = compute_allow_paths(&policy, &command_cwd, &command_cwd, &env_map);

        assert!(paths
            .allow
            .contains(&dunce::canonicalize(&command_cwd).unwrap()));
        assert!(!paths
            .allow
            .contains(&dunce::canonicalize(&temp_dir).unwrap()));
        assert!(paths.deny.is_empty(), "no deny paths expected");
    }

    #[test]
    fn denies_git_dir_inside_writable_REDACTED_SECRET() {
        let tmp = TempDir:REDACTED_SECRET).expect("tempdir");
        let command_cwd = tmp.path().join("workspace");
        let git_dir = command_cwd.join(".git");
        let _ = fs::create_dir_all(&git_dir);

        let policy = SandboxPolicy::WorkspaceWrite {
            writable_REDACTED_SECRETs: vec![],
            read_only_access: Default::default(),
            network_access: false,
            exclude_tmpdir_env_var: true,
            exclude_slash_tmp: false,
        };

        let paths = compute_allow_paths(&policy, &command_cwd, &command_cwd, &HashMap:REDACTED_SECRET));
        let expected_allow: HashSet<PathBuf> = [dunce::canonicalize(&command_cwd).unwrap()]
            .into_iter()
            .collect();
        let expected_deny: HashSet<PathBuf> = [dunce::canonicalize(&git_dir).unwrap()]
            .into_iter()
            .collect();

        assert_eq!(expected_allow, paths.allow);
        assert_eq!(expected_deny, paths.deny);
    }

    #[test]
    fn denies_git_file_inside_writable_REDACTED_SECRET() {
        let tmp = TempDir:REDACTED_SECRET).expect("tempdir");
        let command_cwd = tmp.path().join("workspace");
        let git_file = command_cwd.join(".git");
        let _ = fs::create_dir_all(&command_cwd);
        let _ = fs::write(&git_file, "gitdir: .git/worktrees/example");

        let policy = SandboxPolicy::WorkspaceWrite {
            writable_REDACTED_SECRETs: vec![],
            read_only_access: Default::default(),
            network_access: false,
            exclude_tmpdir_env_var: true,
            exclude_slash_tmp: false,
        };

        let paths = compute_allow_paths(&policy, &command_cwd, &command_cwd, &HashMap:REDACTED_SECRET));
        let expected_allow: HashSet<PathBuf> = [dunce::canonicalize(&command_cwd).unwrap()]
            .into_iter()
            .collect();
        let expected_deny: HashSet<PathBuf> = [dunce::canonicalize(&git_file).unwrap()]
            .into_iter()
            .collect();

        assert_eq!(expected_allow, paths.allow);
        assert_eq!(expected_deny, paths.deny);
    }

    #[test]
    fn denies_codex_and_agents_inside_writable_REDACTED_SECRET() {
        let tmp = TempDir:REDACTED_SECRET).expect("tempdir");
        let command_cwd = tmp.path().join("workspace");
        let codex_dir = command_cwd.join(".codex");
        let agents_dir = command_cwd.join(".agents");
        let _ = fs::create_dir_all(&codex_dir);
        let _ = fs::create_dir_all(&agents_dir);

        let policy = SandboxPolicy::WorkspaceWrite {
            writable_REDACTED_SECRETs: vec![],
            read_only_access: Default::default(),
            network_access: false,
            exclude_tmpdir_env_var: true,
            exclude_slash_tmp: false,
        };

        let paths = compute_allow_paths(&policy, &command_cwd, &command_cwd, &HashMap:REDACTED_SECRET));
        let expected_allow: HashSet<PathBuf> = [dunce::canonicalize(&command_cwd).unwrap()]
            .into_iter()
            .collect();
        let expected_deny: HashSet<PathBuf> = [
            dunce::canonicalize(&codex_dir).unwrap(),
            dunce::canonicalize(&agents_dir).unwrap(),
        ]
        .into_iter()
        .collect();

        assert_eq!(expected_allow, paths.allow);
        assert_eq!(expected_deny, paths.deny);
    }

    #[test]
    fn skips_protected_subdirs_when_missing() {
        let tmp = TempDir:REDACTED_SECRET).expect("tempdir");
        let command_cwd = tmp.path().join("workspace");
        let _ = fs::create_dir_all(&command_cwd);

        let policy = SandboxPolicy::WorkspaceWrite {
            writable_REDACTED_SECRETs: vec![],
            read_only_access: Default::default(),
            network_access: false,
            exclude_tmpdir_env_var: true,
            exclude_slash_tmp: false,
        };

        let paths = compute_allow_paths(&policy, &command_cwd, &command_cwd, &HashMap:REDACTED_SECRET));
        assert_eq!(paths.allow.len(), 1);
        assert!(paths.deny.is_empty(), "no deny when protected dirs are absent");
    }
}
