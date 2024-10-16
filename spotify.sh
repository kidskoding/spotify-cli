#!/bin/bash
# Installation script for spotify command

MAN_DIR="/usr/local/share/man/man1"
MAN_PAGE="spotify.1"

if [ "$(id -u)" -ne 0 ]; then 
    echo "This script must be run as root/administrator. Please use sudo."
    exit 1
fi

if [ ! -f "target/release/spotify" ]; then
    echo "Source file 'target/release/spotify' does not exist. Please build the project first."
    exit 1
fi
cp "target/release/spotify" "/usr/local/bin"
if [ $? -ne 0 ]; then
    echo "Failed to install spotify command. Please try again."
    exit 1
fi

mkdir -p "$MAN_DIR"
if [ $? -ne 0 ]; then
    echo "Failed to install spotify command. Please try again."
    exit 1
fi
cp "$MAN_PAGE" "$MAN_DIR"
if [ $? -ne 0 ]; then
    echo "Failed to install spotify command. Please try again."
    exit 1
fi

echo "Installation complete. Please use 'man spotify' or 'spotify --help' for more information."