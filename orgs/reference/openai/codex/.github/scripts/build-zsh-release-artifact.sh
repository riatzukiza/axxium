#!/usr/bin/env bash

set -euo pipefail

if [[ "$#" -ne 1 ]]; then
  echo "usage: $0 <archive-path>" >&2
  exit 1
fi

archive_path="$1"
workspace="${GITHUB_WORKSPACE:?missing GITHUB_WORKSPACE}"
zsh_commit="${ZSH_COMMIT:?missing ZSH_COMMIT}"
zsh_patch="${ZSH_PATCH:?missing ZSH_PATCH}"
temp_REDACTED_SECRET="${RUNNER_TEMP:-/tmp}"
work_REDACTED_SECRET="$(mktemp -d "${temp_REDACTED_SECRET%/}/codex-zsh-release.XXXXXX")"
trap 'rm -rf "$work_REDACTED_SECRET"' EXIT

source_REDACTED_SECRET="${work_REDACTED_SECRET}/zsh"
package_REDACTED_SECRET="${work_REDACTED_SECRET}/codex-zsh"
wrapper_path="${work_REDACTED_SECRET}/exec-wrapper"
stdout_path="${work_REDACTED_SECRET}/stdout.txt"
wrapper_log_path="${work_REDACTED_SECRET}/wrapper.log"

git clone https://git.code.sf.net/p/zsh/code "$source_REDACTED_SECRET"
cd "$source_REDACTED_SECRET"
git checkout "$zsh_commit"
git apply "${workspace}/${zsh_patch}"
./Util/preconfig
./configure

cores="$(command -v nproc >/dev/null 2>&1 && nproc || getconf _NPROCESSORS_ONLN)"
make -j"${cores}"

cat > "$wrapper_path" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
: "${CODEX_WRAPPER_LOG:?missing CODEX_WRAPPER_LOG}"
printf '%s\n' "$@" > "$CODEX_WRAPPER_LOG"
file="$1"
shift
if [[ "$#" -eq 0 ]]; then
  exec "$file"
fi
arg0="$1"
shift
exec -a "$arg0" "$file" "$@"
EOF
chmod +x "$wrapper_path"

CODEX_WRAPPER_LOG="$wrapper_log_path" \
EXEC_WRAPPER="$wrapper_path" \
"${source_REDACTED_SECRET}/Src/zsh" -fc '/bin/echo smoke-zsh' > "$stdout_path"

grep -Fx "smoke-zsh" "$stdout_path"
grep -Fx "/bin/echo" "$wrapper_log_path"

mkdir -p "$package_REDACTED_SECRET/bin" "$(dirname "${workspace}/${archive_path}")"
cp "${source_REDACTED_SECRET}/Src/zsh" "$package_REDACTED_SECRET/bin/zsh"
chmod +x "$package_REDACTED_SECRET/bin/zsh"

(cd "$work_REDACTED_SECRET" && tar -czf "${workspace}/${archive_path}" codex-zsh)
