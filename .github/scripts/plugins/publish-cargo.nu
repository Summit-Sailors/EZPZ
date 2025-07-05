#!/usr/bin/env nu

def main [package_name: string, plugin_path: string] {
    cd $plugin_path
    
    print $"🦀 Publishing ($package_name) to crates.io..."
    
    let cargo_toml = ($plugin_path | path join "Cargo.toml")
    if ($cargo_toml | path exists) {
        cargo publish
        print $"✅ Successfully published ($package_name) to crates.io"
    } else {
        print $"⚠️ No Cargo.toml found for ($package_name)"
    }
}