#!/usr/bin/env nu

def main [plugin_path: string] {
    cd $plugin_path
    cargo build --release
}