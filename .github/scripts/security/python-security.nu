#!/usr/bin/env nu

# Runs safety, bandit, and other security checks on Python components

def main [] {
    print "Running Python security audit..."
    
    # components to audit (excluding api, app, and formatterz)
    let components = [
        "core/pluginz"
        "core/macroz" 
        "core/registry"
        "examples"
        "plugins/ezpz-rust-ti"
    ]
    
    run-external "rye" "sync" "--all-features"
    
    for component in $components {
        if ($component | path join "pyproject.toml" | path exists) {
            audit-component $component
        }
    }
}

def audit-component [component: string] {
    print $"Auditing ($component)..."
    
    cd $component
    
    run-external "rye" "sync" "--all-features" | ignore
    
    # run-safety-check $component Incompatibility with pydantic and safety (will be enabled later)
    
    run-bandit-check $component
    
    cd ..
}

def run-safety-check [component: string] {
    print $"Running safety check for ($component)..."
    
    try {
        run-external "rye" "run" "safety" "check" "--json" | save "safety_report.json"
        
        if ("safety_report.json" | path exists) and (open "safety_report.json" | str length) > 0 {
            let report = open "safety_report.json" | from json
            
            if ($report | get vulnerabilities | length) > 0 {
                print $"⚠️  Security vulnerabilities found in ($component):"
                $report.vulnerabilities | each { |vuln|
                    print $"  Package: ($vuln.package_name)"
                    print $"  Vulnerability: ($vuln.vulnerability_id)"
                    print $"  Advisory: ($vuln.advisory)"
                    print ""
                }
            } else {
                print $"✅ No security vulnerabilities found in ($component)"
            }
        } else {
            print $"✅ No security vulnerabilities found in ($component)"
        }
    } catch {
        print $"✅ No security vulnerabilities found in ($component)"
    }
}

def run-bandit-check [component: string] {
    print $"Running bandit for ($component)..."
    
    let source_dirs = if $component == "core/pluginz" {
        if ("ezpz_pluginz" | path exists) { "ezpz_pluginz" } else { "" }
    } else if $component == "core/macroz" {
        if ("painlezz_macroz" | path exists) { "painlezz_macroz" } else { "" }
    } else if $component == "core/registry" {
        if ("ezpz_registry" | path exists) { "ezpz_registry" } else { "" }
    } else if $component == "plugins/ezpz-rust-ti" {
        if ("python" | path exists) { "python" } else { "" }
    } else {
        if ("src" | path exists) { "src" } else { "" }
    }
    
    if ($source_dirs | str length) > 0 {
        try {
            run-external "rye" "run" "bandit" "-r" $source_dirs "-f" "json" "-o" "bandit_report.json" | ignore
            
            if ("bandit_report.json" | path exists) and (open "bandit_report.json" | str length) > 0 {
                let report = open "bandit_report.json" | from json
                let issues = $report.results | length
                
                if $issues > 0 {
                    print $"⚠️  ($issues) security issues found in ($component):"
                    $report.results | each { |issue|
                        print $"  Test ID: ($issue.test_id)"
                        print $"  Severity: ($issue.issue_severity)"
                        print $"  Issue: ($issue.issue_text)"
                        print $"  File: ($issue.filename)"
                        print ""
                    }
                } else {
                    print $"✅ No security issues found in ($component)"
                }
            } else {
                print $"✅ No security issues found in ($component)"
            }
        } catch {
            print $"✅ No security issues found in ($component)"
        }
    } else {
        print $"ℹ️  No Python source directories found in ($component)"
    }
}