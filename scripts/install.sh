#!/usr/bin/env bash
set -euo pipefail

REPO="${CTD_REPO:-lprior-repo/centralized-docs}"
INSTALL_DIR="${CTD_INSTALL_DIR:-$HOME/.local/bin}"
VERSION="${CTD_VERSION:-latest}"
TMP_ARCHIVE="$(mktemp)"
TMP_DIR="$(mktemp -d)"

cleanup() {
	rm -f "$TMP_ARCHIVE"
	rm -f "$TMP_DIR/ctd" "$TMP_DIR/ctd.exe" "$TMP_DIR/ctd-mcp" "$TMP_DIR/ctd-mcp.exe" "$TMP_DIR/llms_txt_validator" "$TMP_DIR/llms_txt_validator.exe" "$TMP_DIR/README.md" "$TMP_DIR/LICENSE"
	rmdir "$TMP_DIR" 2>/dev/null || true
}

trap cleanup EXIT

say() {
	printf '%s\n' "$*"
}

fail() {
	printf 'error: %s\n' "$*" >&2
	exit 1
}

need_cmd() {
	command -v "$1" >/dev/null 2>&1 || fail "missing required command: $1"
}

detect_platform() {
	local os
	local arch

	os="$(uname -s)"
	arch="$(uname -m)"

	case "$os" in
	Linux) os="linux" ;;
	Darwin) os="macos" ;;
	MINGW* | MSYS* | CYGWIN*) os="windows" ;;
	*) fail "unsupported operating system: $os" ;;
	esac

	case "$arch" in
	x86_64 | amd64) arch="x86_64" ;;
	arm64 | aarch64) arch="aarch64" ;;
	*) fail "unsupported architecture: $arch" ;;
	esac

	printf '%s %s\n' "$os" "$arch"
}

resolve_version() {
	if [ "$VERSION" != "latest" ]; then
		printf '%s\n' "$VERSION"
		return
	fi

	local latest_url
	latest_url="$(curl -fsSLI -o /dev/null -w '%{url_effective}' "https://github.com/$REPO/releases/latest")"
	printf '%s\n' "${latest_url##*/}"
}

asset_name_for() {
	local os="$1"
	local arch="$2"
	local version="$3"

	case "$os-$arch" in
	linux-x86_64) printf 'ctd-%s-linux-x86_64.tar.gz\n' "$version" ;;
	macos-aarch64) printf 'ctd-%s-macos-aarch64.tar.gz\n' "$version" ;;
	windows-x86_64) printf 'ctd-%s-windows-x86_64.zip\n' "$version" ;;
	*) printf '\n' ;;
	esac
}

sha256_tool() {
	if command -v sha256sum >/dev/null 2>&1; then
		printf 'sha256sum\n'
	elif command -v shasum >/dev/null 2>&1; then
		printf 'shasum -a 256\n'
	else
		fail "missing checksum tool: need sha256sum or shasum"
	fi
}

verify_checksum() {
	local asset_name="$1"
	local version="$2"
	local expected
	local actual
	local checksum_cmd

	expected="$(curl -fsSL "https://github.com/$REPO/releases/download/$version/SHA256SUMS.txt" | grep -F "  $asset_name" | awk '{print $1}')"
	[ -n "$expected" ] || fail "could not find checksum for $asset_name"

	checksum_cmd="$(sha256_tool)"
	actual="$(eval "$checksum_cmd \"$TMP_ARCHIVE\"" | awk '{print $1}')"

	[ "$actual" = "$expected" ] || fail "checksum mismatch for $asset_name"
}

install_binary() {
	local src="$1"
	local dest="$2"

	[ -f "$src" ] || fail "expected binary not found in archive: $(basename "$src")"
	install -m 0755 "$src" "$dest"
}

need_cmd curl
need_cmd install

read -r OS ARCH <<EOF
$(detect_platform)
EOF

RESOLVED_VERSION="$(resolve_version)"
ASSET_NAME="$(asset_name_for "$OS" "$ARCH" "$RESOLVED_VERSION")"

if [ "$OS" = "windows" ]; then
	need_cmd unzip
else
	need_cmd tar
fi

if [ -z "$ASSET_NAME" ]; then
	fail "no prebuilt release is available for $OS/$ARCH. Build from source with: cargo install --path centralized-docs"
fi

ASSET_URL="https://github.com/$REPO/releases/download/$RESOLVED_VERSION/$ASSET_NAME"

say "Installing ctd $RESOLVED_VERSION for $OS/$ARCH"
mkdir -p "$INSTALL_DIR"
curl -fsSL "$ASSET_URL" -o "$TMP_ARCHIVE"
verify_checksum "$ASSET_NAME" "$RESOLVED_VERSION"
if [ "$OS" = "windows" ]; then
	unzip -o "$TMP_ARCHIVE" -d "$TMP_DIR" >/dev/null
else
	tar -xzf "$TMP_ARCHIVE" -C "$TMP_DIR"
fi

if [ "$OS" = "windows" ]; then
	install_binary "$TMP_DIR/ctd.exe" "$INSTALL_DIR/ctd.exe"
	if [ -f "$TMP_DIR/ctd-mcp.exe" ]; then
		install_binary "$TMP_DIR/ctd-mcp.exe" "$INSTALL_DIR/ctd-mcp.exe"
	fi
	if [ -f "$TMP_DIR/llms_txt_validator.exe" ]; then
		install_binary "$TMP_DIR/llms_txt_validator.exe" "$INSTALL_DIR/llms_txt_validator.exe"
	fi
else
	install_binary "$TMP_DIR/ctd" "$INSTALL_DIR/ctd"
	if [ -f "$TMP_DIR/ctd-mcp" ]; then
		install_binary "$TMP_DIR/ctd-mcp" "$INSTALL_DIR/ctd-mcp"
	fi
	if [ -f "$TMP_DIR/llms_txt_validator" ]; then
		install_binary "$TMP_DIR/llms_txt_validator" "$INSTALL_DIR/llms_txt_validator"
	fi
fi

say "Installed to $INSTALL_DIR"

case ":$PATH:" in
*":$INSTALL_DIR:"*) ;;
*)
	say "Add this to your shell profile if needed:"
	say "  export PATH=\"$INSTALL_DIR:\$PATH\""
	;;
esac

say "Run 'ctd --help' to get started"
