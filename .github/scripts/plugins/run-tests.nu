#!/usr/bin/env nu

def main [package_name: string, plugin_path: string] {
    print $"🧪 Running tests for ($package_name)"
    
    # Python tests
    let tests_dir = ($plugin_path | path join "tests")
    if ($tests_dir | path exists) {
        print "Running Python tests..."
        rye test -p $plugin_path
    }
    
    # Rust tests
    let cargo_toml = ($plugin_path | path join "Cargo.toml")
    if ($cargo_toml | path exists) {
        print "Running Rust tests..."
        cd $plugin_path
        cargo test
    }
}