#!/usr/bin/env bash
set -e

echo "======================================================"
echo " Installing doc_transformer (Centralized Docs)        "
echo "======================================================"
echo ""

# Detect OS
OS="$(uname -s)"
case "${OS}" in
Linux*) MACHINE=Linux ;;
Darwin*) MACHINE=Mac ;;
*) MACHINE="UNKNOWN:${OS}" ;;
esac

echo "Detected OS: ${MACHINE}"
echo ""

# Check if Rust/Cargo is installed
if ! command -v cargo &>/dev/null; then
	echo "Rust and Cargo are required but not found."
	echo "Installing Rust via rustup..."

	if [ "${MACHINE}" == "Mac" ]; then
		# On Mac, ensure xcode-select is installed
		if ! xcode-select -p &>/dev/null; then
			echo "Installing Xcode Command Line Tools..."
			xcode-select --install || true
			echo "Please complete the Xcode Command Line Tools installation dialog if it appeared."
			echo "Then re-run this script."
		fi
	elif [ "${MACHINE}" == "Linux" ]; then
		# On Debian/Ubuntu, build-essential and pkg-config/libssl-dev are often needed for compiling.
		if command -v apt-get &>/dev/null; then
			echo "Checking for build dependencies (cc/linker/openssl)..."
			if ! command -v cc &>/dev/null || ! command -v pkg-config &>/dev/null; then
				echo "Installing build-essential, pkg-config, and libssl-dev (requires sudo)..."
				sudo apt-get update && sudo apt-get install -y build-essential pkg-config libssl-dev || echo "Warning: failed to install build dependencies. Compilation might fail."
			fi
		fi
	fi

	# Install rustup
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

	# Source the cargo environment for the rest of the script
	if [ -f "$HOME/.cargo/env" ]; then
		source "$HOME/.cargo/env"
	else
		echo "Please restart your terminal or run: source $HOME/.cargo/env"
		exit 1
	fi
else
	echo "✓ Rust is already installed"
fi

echo ""
echo "Fetching and compiling doc_transformer from GitHub..."
echo "This may take a minute depending on your computer's speed..."
echo ""

# Install directly from the git repository
cargo install --git https://github.com/lprior-repo/centralized-docs doc_transformer --force

echo ""
echo "======================================================"
echo "✓ Installation Complete!"
echo "======================================================"
echo ""

# Check if ~/.cargo/bin is in the PATH
if [[ ":$PATH:" != *":$HOME/.cargo/bin:"* ]]; then
	echo "⚠️  WARNING: ~/.cargo/bin is not in your PATH."
	echo "To run doc_transformer, you need to add it to your PATH."
	echo ""
	if [[ "${SHELL}" == *"zsh"* ]]; then
		echo "Run this command to fix it for your current shell:"
		echo "  export PATH=\"\$HOME/.cargo/bin:\$PATH\""
		echo ""
		echo "And add this line to your ~/.zshrc file to make it permanent:"
		echo "  export PATH=\"\$HOME/.cargo/bin:\$PATH\""
	elif [[ "${SHELL}" == *"bash"* ]]; then
		echo "Run this command to fix it for your current shell:"
		echo "  export PATH=\"\$HOME/.cargo/bin:\$PATH\""
		echo ""
		echo "And add this line to your ~/.bashrc or ~/.bash_profile to make it permanent:"
		echo "  export PATH=\"\$HOME/.cargo/bin:\$PATH\""
	else
		echo "Please add \$HOME/.cargo/bin to your PATH in your shell profile."
	fi
	echo "======================================================"
	echo ""
else
	echo "You can now run the tool from anywhere in your terminal."
	echo ""
	echo "Try it out:"
	echo "  doc_transformer --help"
	echo "  doc_transformer ingest-git https://github.com/tokio-rs/tokio -o ./tokio_docs"
	echo ""
fi
