#!/usr/bin/env nu

def main [package_name: string, plugin_path: string] {
    cd $plugin_path
    
    print $"🏗️ Building plugin: ($package_name)"
    
    # Python package
    let pyproject_toml = ($plugin_path | path join "pyproject.toml")
    if ($pyproject_toml | path exists) {
        print "📦 Building Python package..."
        rye build
    }
    
    # Rust package
    let cargo_toml = ($plugin_path | path join "Cargo.toml")
    if ($cargo_toml | path exists) {
        print "🦀 Building Rust package..."
        cargo build --release
    }
}