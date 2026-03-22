#!/bin/bash

# MachTUI Interactive Installer
# Designed for high-end TUI deployment.

set -e

echo "🚀 Starting MachTUI Installation..."

# 1. Check for Rust/Cargo
if ! command -v cargo &> /dev/null
then
    echo "❌ Cargo could not be found. Please install Rust from https://rustup.rs"
    exit 1
fi

# 2. Build the project
echo "🛠️ Compiling MachTUI Engine..."
cargo build --release

# 3. Install binary
echo "📦 Installing 'mach' command globally..."
cargo install --path .

# 4. Add to PATH (ZSH check)
SHELL_CONFIG=""
if [[ $SHELL == *"zsh"* ]]; then
    SHELL_CONFIG="$HOME/.zshrc"
elif [[ $SHELL == *"bash"* ]]; then
    SHELL_CONFIG="$HOME/.bash_profile"
fi

if [ -n "$SHELL_CONFIG" ]; then
    if ! grep -q 'export PATH="$HOME/.cargo/bin:$PATH"' "$SHELL_CONFIG"; then
        echo "🔗 Adding ~/.cargo/bin to PATH in $SHELL_CONFIG"
        echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> "$SHELL_CONFIG"
    fi
fi

echo "✅ MachTUI Installation Complete!"
echo "✨ Run 'source $SHELL_CONFIG' and then 'mach config' to get started."
