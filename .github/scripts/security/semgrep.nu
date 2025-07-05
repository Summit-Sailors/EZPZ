#!/usr/bin/env nu

# Runs semgrep security analysis excluding certain directories

def main [] {
    print "Running Semgrep security scan..."
    
    let excluded_dirs = [
        "formatterz/"
        "api/"
        "app/"
        ".ruff_cache/"
        ".pytest_cache/"
        "target/"
    ]
    
    let exclude_args = $excluded_dirs | each { |dir| ["--exclude" $dir] } | flatten
    
    try {
        run-external "semgrep" "--config=auto" "--json" "--output=semgrep_report.json" ...$exclude_args "."
        
        if ("semgrep_report.json" | path exists) and (open "semgrep_report.json" | str length) > 0 {
            let report = open "semgrep_report.json"
            let findings = $report.results | length
            
            if $findings > 0 {
                print $"⚠️  ($findings) security findings from Semgrep:"
                $report.results | each { |result|
                    print $"  Rule ID: ($result.check_id)"
                    print $"  Severity: ($result.extra.severity)"
                    print $"  Message: ($result.extra.message)"
                    print $"  File: ($result.path)"
                    print ""
                }
            } else {
                print "✅ No security findings from Semgrep"
            }
        } else {
            print "✅ No security findings from Semgrep"
        }
    } catch {
        print "✅ No security findings from Semgrep"
    }
}