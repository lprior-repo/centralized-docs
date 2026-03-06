#!/usr/bin/env bash
set -e

echo "======================================================"
echo " Installing doc_transformer (Centralized Docs)        "
echo "======================================================"
echo ""

# Check if Rust/Cargo is installed
if ! command -v cargo &>/dev/null; then
	echo "Rust and Cargo are required but not found."
	echo "Installing Rust via rustup..."
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
echo "This may take a minute depending on your Mac's speed..."
echo ""

# Install directly from the git repository
cargo install --git https://github.com/lprior-repo/centralized-docs doc_transformer --force

echo ""
echo "======================================================"
echo "✓ Installation Complete!"
echo "======================================================"
echo ""
echo "You can now run the tool from anywhere in your terminal."
echo ""
echo "Try it out:"
echo "  doc_transformer --help"
echo "  doc_transformer ingest-git https://github.com/tokio-rs/tokio -o ./tokio_docs"
echo ""
