#!/bin/sh
set -eu

die() {
	printf 'error: %s\n' "$*" >&2
	exit 1
}

if [ "$#" -gt 1 ]; then
	die "usage: $0 [tamago-go1.27.0.linux-amd64.tar.gz]"
fi

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo_root=$(CDPATH= cd -- "$script_dir/../.." && pwd)
go_mod="$repo_root/go.mod"
go_sum="$repo_root/go.sum"
lock_file="$repo_root/toolchain/tamago/toolchain.lock.toml"
pin_file="$repo_root/internal/tamagoanchor/pin.go"
patch_dir="$repo_root/toolchain/tamago/patches"

[ -f "$go_mod" ] || die "missing $go_mod"
[ -f "$go_sum" ] || die "missing $go_sum"
[ -f "$lock_file" ] || die "missing $lock_file"
[ -f "$pin_file" ] || die "missing $pin_file"

grep -Fqx 'go 1.27.0' "$go_mod" || die "go.mod must declare Go 1.27.0"
grep -Fqx 'require github.com/usbarmory/tamago v1.27.0' "$go_mod" ||
	die "go.mod must require github.com/usbarmory/tamago v1.27.0"
grep -Fqx 'tool github.com/usbarmory/tamago/cmd/tamago' "$go_mod" ||
	die "go.mod must declare the official tamago tool"
grep -Fqx 'github.com/usbarmory/tamago v1.27.0 h1:ykmK41XTYPQ1XMklahbQGQQ8SvLy5T++O6qmrNYgJoI=' "$go_sum" ||
	die "go.sum is missing the pinned TamaGo module checksum"
grep -Fqx 'github.com/usbarmory/tamago v1.27.0/go.mod h1:TooN7bhk0N4muZeXWHBz69iTTfZ/QHoOv7eOoKQCL1U=' "$go_sum" ||
	die "go.sum is missing the pinned TamaGo go.mod checksum"

grep -Fqx 'module = "github.com/usbarmory/tamago"' "$lock_file" ||
	die "toolchain lock has the wrong framework module"
grep -Fqx 'version = "v1.27.0"' "$lock_file" ||
	die "toolchain lock has the wrong framework version"
grep -Fqx 'tag = "tamago-go1.27.0"' "$lock_file" ||
	die "toolchain lock has the wrong compiler tag"
grep -Fqx 'tag_object = "c6f1d7290a6fe7356694fd4dc711ec4cee62df79"' "$lock_file" ||
	die "toolchain lock has the wrong compiler tag object"
grep -Fqx 'commit = "81722d046d7fd325f6719cc7c638b8822a2803b3"' "$lock_file" ||
	die "toolchain lock has the wrong compiler commit"
grep -Fqx 'delivery = "go-tool"' "$lock_file" ||
	die "toolchain lock must use go-tool delivery"
grep -Fqx 'patch_set = "none"' "$lock_file" ||
	die "toolchain lock must describe the current patch set"
grep -Fqx 'sha256 = "1b014c1b12a6195fad1eebd1f9af6d5c788b4db11569ca43548080f744e9c7a2"' "$lock_file" ||
	die "toolchain lock has the wrong release checksum"

grep -Fq 'FrameworkModule  = "github.com/usbarmory/tamago"' "$pin_file" ||
	die "TamaGo framework module anchor is stale"
grep -Fq 'FrameworkVersion = "v1.27.0"' "$pin_file" ||
	die "TamaGo framework version anchor is stale"
grep -Fq 'CompilerTag      = "tamago-go1.27.0"' "$pin_file" ||
	die "TamaGo compiler tag anchor is stale"
grep -Fq 'CompilerCommit   = "81722d046d7fd325f6719cc7c638b8822a2803b3"' "$pin_file" ||
	die "TamaGo compiler commit anchor is stale"

if find "$patch_dir" -type f -name '*.patch' -print | grep -q .; then
	die "patch_set is none but patch files are present"
fi

[ ! -e "$repo_root/vendor" ] || die "vendor/ is not an accepted TamaGo source"
[ ! -e "$repo_root/.gitmodules" ] || die "Git submodules are not an accepted TamaGo source"

if [ "$#" -eq 1 ]; then
	artifact=$1
	[ -f "$artifact" ] || die "artifact does not exist: $artifact"

	case $(basename -- "$artifact") in
		tamago-go1.27.0.linux-amd64.tar.gz) ;;
		*) die "unexpected artifact name: $(basename -- "$artifact")" ;;
	esac

	if command -v sha256sum >/dev/null 2>&1; then
		actual=$(sha256sum -- "$artifact" | awk '{print $1}')
	elif command -v shasum >/dev/null 2>&1; then
		actual=$(shasum -a 256 -- "$artifact" | awk '{print $1}')
	else
		die "sha256sum or shasum is required to verify an artifact"
	fi

	expected=1b014c1b12a6195fad1eebd1f9af6d5c788b4db11569ca43548080f744e9c7a2
	[ "$actual" = "$expected" ] || die "artifact SHA-256 mismatch"
	printf 'verified artifact: %s\n' "$artifact"
fi

printf 'verified repository pins for TamaGo v1.27.0 and tamago-go1.27.0\n'
