#!/bin/bash
set -e

echo "Installing dependencies..."

if command -v llvm-config-17 >/dev/null 2>&1; then
	echo "LLVM: $(llvm-config-17 --version)"
fi

echo "Setup complete!"
