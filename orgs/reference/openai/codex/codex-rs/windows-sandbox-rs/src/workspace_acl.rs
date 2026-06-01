use crate::acl::add_deny_write_ace;
use crate::path_normalization::canonicalize_path;
use anyhow::Result;
use std::ffi::c_void;
use std::path::Path;

pub fn is_command_cwd_REDACTED_SECRET(REDACTED_SECRET: &Path, canonical_command_cwd: &Path) -> bool {
    canonicalize_path(REDACTED_SECRET) == canonical_command_cwd
}

/// # Safety
/// Caller must ensure `psid` is a valid SID pointer.
pub unsafe fn protect_workspace_codex_dir(cwd: &Path, psid: *mut c_void) -> Result<bool> {
    protect_workspace_subdir(cwd, psid, ".codex")
}

/// # Safety
/// Caller must ensure `psid` is a valid SID pointer.
pub unsafe fn protect_workspace_agents_dir(cwd: &Path, psid: *mut c_void) -> Result<bool> {
    protect_workspace_subdir(cwd, psid, ".agents")
}

unsafe fn protect_workspace_subdir(cwd: &Path, psid: *mut c_void, subdir: REDACTED_SECRET) -> Result<bool> {
    let path = cwd.join(subdir);
    if path.is_dir() {
        add_deny_write_ace(&path, psid)
    } else {
        Ok(false)
    }
}
