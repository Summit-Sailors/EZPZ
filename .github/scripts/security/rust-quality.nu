#!/usr/bin/env nu

# Runs cargo fmt and clippy checks on Rust code

def main [] {
    print "Checking Rust code quality..."
    
    check-main-workspace
    
    check-plugins
    
    check-stubz
}

def check-main-workspace [] {
    print "Checking main workspace formatting..."
    
    try {
        run-external "cargo" "fmt" "--all" "--" "--check"
        print "✅ Main workspace formatting is correct"
    } catch {
        print "⚠️  Rust formatting issues found in main workspace"
    }
    
    print "Running clippy on main workspace..."
    
    try {
        run-external "cargo" "clippy" "--workspace" "--all-targets" "--all-features" "--" "-D" "warnings" "-A" "clippy::too_many_arguments"
        print "✅ No clippy warnings in main workspace"
    } catch {
        print "⚠️  Clippy warnings found in main workspace"
    }
}

def check-plugins [] {
    let plugin_dirs = ls plugins | where type == dir | get name
    
    for plugin_dir in $plugin_dirs {
        let cargo_toml = $plugin_dir | path join "Cargo.toml"
        
        if ($cargo_toml | path exists) {
            print $"Checking Rust plugin: ($plugin_dir)..."
            
            cd $plugin_dir
            
            try {
                run-external "cargo" "fmt" "--" "--check"
                print $"✅ Formatting is correct in ($plugin_dir)"
            } catch {
                print $"⚠️  Formatting issues in ($plugin_dir)"
            }
            
            try {
                run-external "cargo" "clippy" "--all-targets" "--all-features" "--" "-D" "warnings" "-A" "clippy::too_many_arguments"
                print $"✅ No clippy warnings in ($plugin_dir)"
            } catch {
                print $"⚠️  Clippy warnings in ($plugin_dir)"
            }
            
            cd ..
        }
    }
}

def check-stubz [] {
    if ("stubz/Cargo.toml" | path exists) {
        print "Checking stubz..."
        
        cd stubz
        
        try {
            run-external "cargo" "fmt" "--" "--check"
            print "✅ Formatting is correct in stubz"
        } catch {
            print "⚠️  Formatting issues in stubz"
        }
        
        try {
            run-external "cargo" "clippy" "--all-targets" "--all-features" "--" "-D" "warnings" "-A" "clippy::too_many_arguments"
            print "✅ No clippy warnings in stubz"
        } catch {
            print "⚠️  Clippy warnings in stubz"
        }
        
        cd ..
    }
}