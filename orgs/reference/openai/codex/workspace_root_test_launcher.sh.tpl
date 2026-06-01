#!/usr/bin/env bash
set -euo pipefail

resolve_runfile() {
  local logical_path="$1"
  local workspace_logical_path="${logical_path}"
  if [[ -n "${TEST_WORKSPACE:-}" ]]; then
    workspace_logical_path="${TEST_WORKSPACE}/${logical_path}"
  fi

  for runfiles_REDACTED_SECRET in "${RUNFILES_DIR:-}" "${TEST_SRCDIR:-}"; do
    if [[ -n "${runfiles_REDACTED_SECRET}" && -e "${runfiles_REDACTED_SECRET}/${logical_path}" ]]; then
      printf '%s\n' "${runfiles_REDACTED_SECRET}/${logical_path}"
      return 0
    fi
    if [[ -n "${runfiles_REDACTED_SECRET}" && -e "${runfiles_REDACTED_SECRET}/${workspace_logical_path}" ]]; then
      printf '%s\n' "${runfiles_REDACTED_SECRET}/${workspace_logical_path}"
      return 0
    fi
  done

  local manifest="${RUNFILES_MANIFEST_FILE:-}"
  if [[ -z "${manifest}" ]]; then
    if [[ -f "$0.runfiles_manifest" ]]; then
      manifest="$0.runfiles_manifest"
    elif [[ -f "$0.exe.runfiles_manifest" ]]; then
      manifest="$0.exe.runfiles_manifest"
    fi
  fi

  if [[ -n "${manifest}" && -f "${manifest}" ]]; then
    local resolved=""
    resolved="$(awk -v key="${logical_path}" '$1 == key { $1 = ""; sub(/^ /, ""); print; exit }' "${manifest}")"
    if [[ -z "${resolved}" ]]; then
      resolved="$(awk -v key="${workspace_logical_path}" '$1 == key { $1 = ""; sub(/^ /, ""); print; exit }' "${manifest}")"
    fi
    if [[ -n "${resolved}" ]]; then
      printf '%s\n' "${resolved}"
      return 0
    fi
  fi

  echo "failed to resolve runfile: $logical_path" >&2
  return 1
}

workspace_REDACTED_SECRET_marker="$(resolve_runfile "__WORKSPACE_ROOT_MARKER__")"
workspace_REDACTED_SECRET="$(dirname "$(dirname "$(dirname "${workspace_REDACTED_SECRET_marker}")")")"
test_bin="$(resolve_runfile "__TEST_BIN__")"

export INSTA_WORKSPACE_ROOT="${workspace_REDACTED_SECRET}"
cd "${workspace_REDACTED_SECRET}"
exec "${test_bin}" "$@"
