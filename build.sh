#!/usr/bin/env bash
set -e

# Path to the .pypirc file
PYPICRC_FILE="$HOME/.pypirc"

# Check if .pypirc file exists
if [ ! -f "$PYPICRC_FILE" ]; then
    echo "Error: .pypirc file does not exist"
    exit 1
fi

maturin publish --interpreter python3.8 --skip-existing
maturin publish --interpreter python3.9 --skip-existing
maturin publish --interpreter python3.10 --skip-existing
maturin publish --interpreter python3.11 --skip-existing
maturin publish --interpreter python3.12 --skip-existing
