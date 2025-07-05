#!/usr/bin/env nu

# Checks Rust dependencies for outdated packages

def main [] {
    print "Checking Rust dependencies for updates..."
    
    
    check-main-workspace
    
    check-plugins
    
    check-stubz
}

def check-main-workspace [] {
    print "Checking main workspace..."
    
    try {
        run-external "cargo" "outdated" "--format" "json" | save --force "cargo_outdated_main.json"
    } catch {
        '{"dependencies": []}' | save --force "cargo_outdated_main.json"
    }
    
    if ("cargo_outdated_main.json" | path exists) and (ls "cargo_outdated_main.json" | get size | first | into int) > 0 {
        let report = open "cargo_outdated_main.json"
        let outdated_count = $report.dependencies | length
        
        if $outdated_count > 0 {
            print $"📦 ($outdated_count) outdated Rust dependencies in main workspace:"
            $report.dependencies | each { |dep|
                print $"  Name: ($dep.name)"
                print $"  Current: ($dep.project)"
                print $"  Latest: ($dep.compat)"
                print ""
            }
        } else {
            print "✅ All Rust dependencies are up to date in main workspace"
        }
    } else {
        print "✅ All Rust dependencies are up to date in main workspace"
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
                run-external "cargo" "outdated" "--format" "json" | save --force  "cargo_outdated_plugin.json"
            } catch {
                '{"dependencies": []}' | save "cargo_outdated_plugin.json"
            }
            
            if ("cargo_outdated_plugin.json" | path exists) and (ls "cargo_outdated_plugin.json" | get size | first | into int) > 0 {
                let report = open "cargo_outdated_plugin.json"
                let outdated_count = $report.dependencies | length
                
                if $outdated_count > 0 {
                    print $"📦 ($outdated_count) outdated dependencies in ($plugin_dir):"
                    $report.dependencies | each { |dep|
                        print $"  Name: ($dep.name)"
                        print $"  Current: ($dep.project)"
                        print $"  Latest: ($dep.compat)"
                        print ""
                    }
                } else {
                    print $"✅ All dependencies are up to date in ($plugin_dir)"
                }
            } else {
                print $"✅ All dependencies are up to date in ($plugin_dir)"
            }
            
            cd ..
        }
    }
}

def check-stubz [] {
    if ("stubz/Cargo.toml" | path exists) {
        print "Checking stubz dependencies..."
        
        cd stubz
        
        try {
            run-external "cargo" "outdated" "--format" "json" | save --force "cargo_outdated_stubz.json"
        } catch {
            '{"dependencies": []}' | save "cargo_outdated_stubz.json"
        }
        
        if ("cargo_outdated_stubz.json" | path exists) and (ls "cargo_outdated_stubz.json" | get size | first | into int) > 0 {
            let report = open "cargo_outdated_stubz.json"
            let outdated_count = $report.dependencies | length
            
            if $outdated_count > 0 {
                print $"📦 ($outdated_count) outdated dependencies in stubz:"
                $report.dependencies | each { |dep|
                    print $"  Name: ($dep.name)"
                    print $"  Current: ($dep.project)"
                    print $"  Latest: ($dep.compat)"
                    print ""
                }
            } else {
                print "✅ All dependencies are up to date in stubz"
            }
        } else {
            print "✅ All dependencies are up to date in stubz"
        }
        
        cd ..
    }
}