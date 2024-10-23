#!/bin/bash

if ! command -v curl >/dev/null 2>&1; then
    echo "curl is required. Please install it and try again."
    exit 1
fi

if ! command -v cargo >/dev/null 2>&1; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

if ! command -v nu >/dev/null 2>&1; then
    cargo install nu
fi