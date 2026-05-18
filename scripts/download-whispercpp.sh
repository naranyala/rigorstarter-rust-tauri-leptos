#!/usr/bin/env bash
set -euo pipefail

REPO_URL="https://github.com/ggml-org/whisper.cpp"
LOCAL_DIR="$(pwd)/thirdparty/whisper.cpp"

trap 'echo "❌ Error at line $LINENO"; exit 1' ERR

# Detect package manager
if command -v apt-get &>/dev/null; then
    PKG_MANAGER="apt"
elif command -v dnf &>/dev/null; then
    PKG_MANAGER="dnf"
else
    echo "❌ No supported package manager found (apt or dnf)."
    exit 1
fi

echo "📦 Using package manager: $PKG_MANAGER"

# Install dependencies (local build only, no global install)
if [[ "$PKG_MANAGER" == "apt" ]]; then
    sudo apt-get update || echo "⚠️ Some repos failed to update, continuing..."
    sudo apt-get install -y git build-essential cmake || { echo "❌ Dependency install failed"; exit 1; }
elif [[ "$PKG_MANAGER" == "dnf" ]]; then
    sudo dnf install -y git gcc gcc-c++ make cmake || { echo "❌ Dependency install failed"; exit 1; }
fi

# Clone or update repo into ./thirdparty
mkdir -p "$(pwd)/thirdparty"
if [[ -d "$LOCAL_DIR" ]]; then
    echo "🔄 Updating whisper.cpp..."
    git -C "$LOCAL_DIR" pull
else
    echo "⬇️ Cloning whisper.cpp into thirdparty..."
    git clone --depth=1 "$REPO_URL" "$LOCAL_DIR"
fi

# Build locally
cd "$LOCAL_DIR"
cmake -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build -j"$(nproc)"

echo "✅ whisper.cpp built locally in: $LOCAL_DIR/build/bin"
echo "👉 To run, use: ./thirdparty/whisper.cpp/build/bin/whisper-cli --help"


