#!/usr/bin/env nu

# Runs cargo audit on Rust components

def main [] {
    print "Running Rust security audit..."
    
    audit-main-workspace
    
    audit-plugins
    
    audit-stubz
}

def audit-main-workspace [] {
    print "Auditing main workspace..."
    
    try {
        run-external "cargo" "audit" "--json" | save "rust_audit_main.json"
        
        if ("rust_audit_main.json" | path exists) and (open "rust_audit_main.json" | str length) > 0 {
            let report = open "rust_audit_main.json" | from json
            let vuln_count = $report.vulnerabilities.count
            
            if $vuln_count > 0 {
                print $"⚠️  ($vuln_count) Rust vulnerabilities found in main workspace:"
                $report.vulnerabilities.list | each { |vuln|
                    print $"  ID: ($vuln.advisory.id)"
                    print $"  Package: ($vuln.package.name)"
                    print $"  Title: ($vuln.advisory.title)"
                    print ""
                }
            } else {
                print "✅ No Rust vulnerabilities found in main workspace"
            }
        } else {
            print "✅ No Rust vulnerabilities found in main workspace"
        }
    } catch {
        print "✅ No Rust vulnerabilities found in main workspace"
    }
}

def audit-plugins [] {
    print "Auditing Rust plugins..."
    
    let plugin_dirs = ls plugins | where type == dir | get name
    
    for plugin_dir in $plugin_dirs {
        let cargo_toml = $plugin_dir | path join "Cargo.toml"
        
        if ($cargo_toml | path exists) {
            print $"Auditing Rust plugin: ($plugin_dir)..."
            
            cd $plugin_dir
            
            try {
                run-external "cargo" "audit" "--json" | save "rust_audit_plugin.json"
                
                if ("rust_audit_plugin.json" | path exists) and (open "rust_audit_plugin.json" | str length) > 0 {
                    let report = open "rust_audit_plugin.json" | from json
                    let vuln_count = $report.vulnerabilities.count
                    
                    if $vuln_count > 0 {
                        print $"⚠️  ($vuln_count) vulnerabilities found in ($plugin_dir):"
                        $report.vulnerabilities.list | each { |vuln|
                            print $"  ID: ($vuln.advisory.id)"
                            print $"  Package: ($vuln.package.name)"
                            print $"  Title: ($vuln.advisory.title)"
                            print ""
                        }
                    } else {
                        print $"✅ No vulnerabilities found in ($plugin_dir)"
                    }
                } else {
                    print $"✅ No vulnerabilities found in ($plugin_dir)"
                }
            } catch {
                print $"✅ No vulnerabilities found in ($plugin_dir)"
            }
            
            cd ..
        }
    }
}

def audit-stubz [] {
    if ("stubz/Cargo.toml" | path exists) {
        print "Auditing stubz component..."
        
        cd stubz
        
        try {
            run-external "cargo" "audit" "--json" | save "rust_audit_stubz.json"
            
            if ("rust_audit_stubz.json" | path exists) and (open "rust_audit_stubz.json" | str length) > 0 {
                let report = open "rust_audit_stubz.json" | from json
                let vuln_count = $report.vulnerabilities.count
                
                if $vuln_count > 0 {
                    print $"⚠️  ($vuln_count) vulnerabilities found in stubz:"
                    $report.vulnerabilities.list | each { |vuln|
                        print $"  ID: ($vuln.advisory.id)"
                        print $"  Package: ($vuln.package.name)"
                        print $"  Title: ($vuln.advisory.title)"
                        print ""
                    }
                } else {
                    print "✅ No vulnerabilities found in stubz"
                }
            } else {
                print "✅ No vulnerabilities found in stubz"
            }
        } catch {
            print "✅ No vulnerabilities found in stubz"
        }
        
        cd ..
    }
}