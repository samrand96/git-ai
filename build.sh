#!/bin/bash
set -e

# Check for pyinstaller
if ! command -v pyinstaller >/dev/null 2>&1; then
  echo "PyInstaller is not installed. Please install it with:"
  echo "  pip install pyinstaller"
  exit 1
fi

# Always build the project
if [ -f main.py ]; then
  pyinstaller --onefile main.py
else
  echo "main.py not found!"
  exit 1
fi

# Ask to add dist to PATH for this session
read -p "Do you want to add the dist folder to your PATH for this session? (y/n): " ADDPATH
if [[ "$ADDPATH" =~ ^[Yy]$ ]]; then
  export PATH="$(pwd)/dist:$PATH"
  echo "dist folder added to PATH for this session."
else
  echo "Skipping PATH update for this session."
fi

# Ask to add dist to shell profile for permanent PATH
read -p "Do you want to add the dist folder to your PATH permanently (shell profile)? (y/n): " PERMPATH
if [[ "$PERMPATH" =~ ^[Yy]$ ]]; then
  PROFILE=""
  if [ -n "$ZSH_VERSION" ]; then PROFILE=~/.zshrc; fi
  if [ -n "$BASH_VERSION" ]; then PROFILE=~/.bashrc; fi
  if [ -z "$PROFILE" ]; then PROFILE=~/.profile; fi
  echo "export PATH=\"$(pwd)/dist:\$PATH\"" >> "$PROFILE"
  echo "dist folder added to PATH in $PROFILE."
else
  echo "Skipping permanent PATH update."
fi
