#!/bin/sh
set -eu

die() {
	printf 'error: %s\n' "$*" >&2
	exit 1
}

if [ "$#" -lt 2 ]; then
	die "usage: $0 <goarch> <go-command> [arguments...]"
fi

target_arch=$1
shift

case "$target_arch" in
	*[!a-z0-9_]*|'') die "invalid GOARCH: $target_arch" ;;
esac

go_command=${GO:-go}
command -v "$go_command" >/dev/null 2>&1 || die "Go command not found: $go_command"

host_os=$("$go_command" env GOHOSTOS)
host_arch=$("$go_command" env GOHOSTARCH)
[ -n "$host_os" ] || die "Go returned an empty GOHOSTOS"
[ -n "$host_arch" ] || die "Go returned an empty GOHOSTARCH"

launcher=$(
	env GOOS="$host_os" GOARCH="$host_arch" GOOSPKG= GOFLAGS= \
		"$go_command" tool -n tamago
)

case "$launcher" in
	/*) ;;
	*) die "go tool returned a non-absolute launcher path: $launcher" ;;
esac
case "$launcher" in
	*[![:graph:]]*) die "go tool returned more than one launcher argument" ;;
esac
[ -x "$launcher" ] || die "TamaGo launcher is not executable: $launcher"

# Force the official launcher to install its matching distribution before the
# target command, then verify that the annotated release tag resolved to the
# commit recorded in this repository. This keeps the standard upstream cache
# and launcher flow while detecting a moved or substituted compiler tag.
"$launcher" version >/dev/null
cache_base=${XDG_CACHE_HOME:-${HOME:-}}
[ -n "$cache_base" ] || die "HOME or XDG_CACHE_HOME is required for TamaGo cache verification"
if [ -z "${XDG_CACHE_HOME:-}" ]; then
	cache_base=$cache_base/.cache
fi
compiler_source=$cache_base/tamago-go/tamago-go1.27.0
[ -d "$compiler_source/.git" ] || die "TamaGo compiler checkout is missing: $compiler_source"
actual_commit=$(git -C "$compiler_source" rev-parse HEAD)
expected_commit=81722d046d7fd325f6719cc7c638b8822a2803b3
[ "$actual_commit" = "$expected_commit" ] ||
	die "TamaGo compiler commit mismatch: $actual_commit"

exec env \
	GOOS=tamago \
	GOOSPKG=github.com/usbarmory/tamago \
	GOARCH="$target_arch" \
	"$launcher" "$@"
