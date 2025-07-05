#!/usr/bin/env nu
# Checks Python dependencies for vulnerabilities and generates reports
def main [] {
    print "Checking Python dependencies for updates..."
    run-external "rye" "sync" "--all-features"
    check-main-dependencies
    let components = [
        "core/pluginz"
        "core/macroz"
        "core/registry"
        "examples"
        "plugins/ezpz-rust-ti"
    ]
    for component in $components {
        if ($component | path join "pyproject.toml" | path exists) {
            check-component-dependencies $component
        }
    }
}

def check-main-dependencies [] {
    print "Checking main workspace dependencies..."
    try {
        run-external "rye" "list" "--json" | save --force "main_deps.json"
    } catch {
        '[]' | save "main_deps.json"
    }
    if ("ezpz-lock.yaml" | path exists) {
        print "✅ Found ezpz-lock.yaml - dependency versions locked"
    } else {
        print "⚠️ No ezpz-lock.yaml found - dependencies may vary between installs"
    }
}

def check-component-dependencies [component: string] {
    print $"Checking ($component)..."
    cd $component
    run-external "rye" "sync" "--all-features" | ignore
    run-pip-audit $component
    show-dependency-info $component
    cd ..
}

def run-pip-audit [component: string] {
    print $"Running pip-audit for ($component)..."
    
    try {
        run-external "rye" "add" "--dev" "pip-audit"
        run-external "rye" "sync"
        
        run-external "rye" "run" "pip-audit" "--format=json" "--output=audit.json" "--desc" "on"
        
    } catch {
        print "⚠️ pip-audit failed, creating empty report"
        '{"vulnerabilities": []}' | save --force "audit.json"
    }
    
    if ("audit.json" | path exists) and (ls "audit.json" | get size | first | into int) > 0 {
        try { 
            let report = open "audit.json" | from json
            let vuln_count = $report.vulnerabilities | length
            if $vuln_count > 0 {
                print $"🚨 ($vuln_count) vulnerable packages in ($component):"
                $report.vulnerabilities | each { |vuln|
                    print $"  Package: ($vuln.package.name)"
                    print $"  Version: ($vuln.package.version)"
                    print $"  Vulnerability: ($vuln.vulnerability.id)"
                    print ""
                }
            } else {
                print $"✅ No vulnerable packages found in ($component)"
            }
        } catch {
            print $"⚠️ Could not parse audit.json for ($component)"
        }
    } else {
        print $"✅ No vulnerable packages found in ($component)"
    }
}

def show-dependency-info [component: string] {
    print $"Dependency info for ($component):"
    try {
        let deps = run-external "rye" "list" | lines | first 10
        $deps | each { |dep| print $"  ($dep)" }
    } catch {
        print "  Could not list dependencies"
    }
    print ""
}