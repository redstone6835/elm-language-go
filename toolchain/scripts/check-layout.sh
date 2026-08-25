#!/bin/sh
set -eu

die() {
	printf 'error: %s\n' "$*" >&2
	exit 1
}

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo_root=$(CDPATH= cd -- "$script_dir/../.." && pwd)

missing_directories=$(
	find "$repo_root" -type d \
		-not -path "$repo_root/.git*" \
		-not -path "$repo_root/target*" \
		-not -path "$repo_root/.cache*" \
		-exec test ! -f '{}/README.md' ';' -print
)
if [ -n "$missing_directories" ]; then
	printf '%s\n' "$missing_directories" >&2
	die "tracked-purpose directories are missing README.md"
fi

[ ! -e "$repo_root/.gitmodules" ] || die "Git submodules are not allowed"
[ ! -e "$repo_root/vendor" ] || die "vendored TamaGo sources are not allowed"
if git -C "$repo_root" ls-files | grep -Eq \
	'(^vendor/|^target/|^toolchain/downloads/|\.eki$|\.elf$|\.tar\.gz$)'; then
	die "generated images, vendored sources, or toolchain archives are tracked"
fi

printf 'verified repository layout and directory documentation\n'
