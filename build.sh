#!/usr/bin/env bash
set -e

# Path to the .pypirc file
PYPICRC_FILE="$HOME/.pypirc"

# Check if .pypirc file exists
if [ ! -f "$PYPICRC_FILE" ]; then
    echo "Error: .pypirc file does not exist"
    exit 1
fi

rm -f target/wheels/*

maturin build --sdist --interpreter python3.9 
maturin build --sdist --interpreter python3.10
maturin build --sdist --interpreter python3.11
maturin build --sdist --interpreter python3.12
maturin build --sdist --interpreter python3.13
