#!/bin/sh
# fetchy installer — downloads pre-built binary or builds from source
set -e

REPO="https://github.com/Samujalphukan228/fetchy"
BIN_NAME="fetchy"
BIN_DIR="$HOME/.local/bin"
CONFIG_DIR="$HOME/.config/fetchy"
LEGACY_BIN="systeminfo"
LEGACY_CONFIG="$HOME/.config/systeminfo"
VERSION="1.0.0"
FORCE_BUILD=0

info()    { printf '  > %s\n' "$1"; }
success() { printf '  ok %s\n' "$1"; }
warn()    { printf '  ! %s\n' "$1"; }
die()     { printf '  error: %s\n' "$1" >&2; exit 1; }

for arg in "$@"; do
    case "$arg" in
        --build) FORCE_BUILD=1 ;;
    esac
done

detect_shell_rc() {
    case "$SHELL" in
        */zsh)  printf '%s\n' "$HOME/.zshrc" ;;
        */bash) printf '%s\n' "$HOME/.bashrc" ;;
        *)      printf '%s\n' "$HOME/.bashrc" ;;
    esac
}

detect_target() {
    _os=$(uname -s)
    _arch=$(uname -m)
    case "$_os" in
        Linux)
            case "$_arch" in
                x86_64|amd64)  printf '%s\n' "x86_64-unknown-linux-gnu" ;;
                aarch64|arm64) printf '%s\n' "aarch64-unknown-linux-gnu" ;;
                *) return 1 ;;
            esac
            ;;
        Darwin)
            case "$_arch" in
                x86_64)        printf '%s\n' "x86_64-apple-darwin" ;;
                arm64|aarch64) printf '%s\n' "aarch64-apple-darwin" ;;
                *) return 1 ;;
            esac
            ;;
        *) return 1 ;;
    esac
}

local_repo() {
    _script="$0"
    case "$_script" in
        sh|dash|bash|ksh|zsh) return 1 ;;
        /*) ;;
        *) _script="$(pwd)/$_script" ;;
    esac
    _dir=$(cd "$(dirname "$_script")" 2>/dev/null && pwd) || return 1
    [ -f "$_dir/Cargo.toml" ] || return 1
    grep -q 'name = "fetchy"' "$_dir/Cargo.toml" 2>/dev/null || return 1
    printf '%s\n' "$_dir"
}

uninstall() {
    info "uninstalling fetchy"
    _removed=0

    if [ -f "$BIN_DIR/$BIN_NAME" ]; then
        rm -f "$BIN_DIR/$BIN_NAME"
        success "removed $BIN_DIR/$BIN_NAME"
        _removed=1
    fi

    if [ -f "/usr/local/bin/$BIN_NAME" ]; then
        rm -f "/usr/local/bin/$BIN_NAME" 2>/dev/null \
            || sudo rm -f "/usr/local/bin/$BIN_NAME"
        success "removed /usr/local/bin/$BIN_NAME"
        _removed=1
    fi

    if [ -f "$BIN_DIR/$LEGACY_BIN" ]; then
        rm -f "$BIN_DIR/$LEGACY_BIN"
        success "removed legacy $BIN_DIR/$LEGACY_BIN"
        _removed=1
    fi

    if [ -f "/usr/local/bin/$LEGACY_BIN" ]; then
        rm -f "/usr/local/bin/$LEGACY_BIN" 2>/dev/null \
            || sudo rm -f "/usr/local/bin/$LEGACY_BIN"
        success "removed legacy /usr/local/bin/$LEGACY_BIN"
        _removed=1
    fi

    [ "$_removed" -eq 1 ] || warn "no installed binary found"

    if [ -d "$CONFIG_DIR" ]; then
        warn "config kept at $CONFIG_DIR (remove manually if needed)"
    fi

    if [ -d "$LEGACY_CONFIG" ]; then
        warn "legacy config kept at $LEGACY_CONFIG (remove manually if needed)"
    fi

    RC=$(detect_shell_rc)
    printf '\n  done. run: source %s\n\n' "$RC"
    exit 0
}

download_binary() {
    TARGET=$(detect_target) || return 1
    URL="$REPO/releases/download/v$VERSION/fetchy-$TARGET"
    TMP_BIN=$(mktemp)

    info "downloading pre-built binary ($TARGET)"
    if ! curl -fsSL "$URL" -o "$TMP_BIN" 2>/dev/null; then
        rm -f "$TMP_BIN"
        return 1
    fi

    mkdir -p "$BIN_DIR"
    chmod +x "$TMP_BIN"
    mv "$TMP_BIN" "$BIN_DIR/$BIN_NAME"
    success "installed $BIN_DIR/$BIN_NAME (pre-built)"
    return 0
}

install_rust_if_needed() {
    if command -v cargo >/dev/null 2>&1; then
        success "rust: $(cargo --version)"
        return
    fi
    info "installing rust via rustup"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path
    # shellcheck disable=SC1091
    . "$HOME/.cargo/env"
    success "rust installed"
}

build_binary() {
    command -v cargo >/dev/null 2>&1 || . "$HOME/.cargo/env"

    _repo=""
    _repo=$(local_repo) || _repo=""
    if [ -n "$_repo" ]; then
        info "building from local checkout"
        (cd "$_repo" && cargo build --release)
        _bin="$_repo/target/release/$BIN_NAME"
    else
        command -v git >/dev/null 2>&1 || die "git is required"
        TMP_DIR=$(mktemp -d)
        info "cloning $REPO"
        git clone --depth 1 "$REPO" "$TMP_DIR/fetchy" >/dev/null 2>&1 \
            || die "failed to clone $REPO"
        info "building release binary (this may take a few minutes)"
        (cd "$TMP_DIR/fetchy" && cargo build --release)
        _bin="$TMP_DIR/fetchy/target/release/$BIN_NAME"
    fi

    [ -f "$_bin" ] || die "binary not found after build"

    mkdir -p "$BIN_DIR"
    cp "$_bin" "$BIN_DIR/$BIN_NAME"
    chmod +x "$BIN_DIR/$BIN_NAME"
    [ -z "$_repo" ] && rm -rf "$TMP_DIR"
    success "installed $BIN_DIR/$BIN_NAME (built from source)"
}

ensure_path() {
    RC=$(detect_shell_rc)
    touch "$RC"
    case ":$PATH:" in
        *":$BIN_DIR:"*) ;;
        *)
            if ! grep -Fq "$BIN_DIR" "$RC" 2>/dev/null; then
                printf '\nexport PATH="%s:$PATH"\n' "$BIN_DIR" >> "$RC"
                success "added $BIN_DIR to PATH in $RC"
            fi
            ;;
    esac
}

print_header() {
    printf '\n'
    printf '  fetchy v%s\n' "$VERSION"
    printf '  system info for your terminal\n'
    printf '\n'
}

if [ "$1" = "--uninstall" ] || [ "$1" = "uninstall" ]; then
    uninstall
fi

print_header

if [ "$FORCE_BUILD" -eq 1 ]; then
    install_rust_if_needed
    build_binary
elif download_binary; then
    :
else
    warn "no pre-built binary for this platform — building from source"
    install_rust_if_needed
    build_binary
fi

ensure_path

RC=$(detect_shell_rc)
printf '\n'
printf '  installed.\n'
printf '  reload your shell:\n'
printf '    source %s\n' "$RC"
printf '\n'
printf '  try:\n'
printf '    %s\n' "$BIN_NAME"
printf '\n'
printf '  force source build: sh install.sh --build\n'
printf '  uninstall: curl -sSf %s/raw/master/install.sh | sh -s -- --uninstall\n' "$REPO"
printf '\n'