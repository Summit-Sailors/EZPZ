#!/usr/bin/env nu

# Generates a summary of dependency updates

def main [] {
    print "Collecting dependency updates for summary..."
    
    generate-summary
}

def generate-summary [] {
    let current_date = date now | format date "%Y-%m-%d %H:%M:%S"
    
    let summary = [
        "## Dependency Update Summary"
        $"Generated on: ($current_date)"
        ""
        "### Python Dependencies"
    ]
    
    let lock_file_status = if ("ezpz-lock.yaml" | path exists) { "✅" } else { "❌" }
    let summary = ($summary | append $"- Lock file: ezpz-lock.yaml ($lock_file_status)")
    
    let total_vulns = count-python-vulnerabilities
    let summary = ($summary | append $"- Total vulnerable packages: ($total_vulns)")
    let summary = ($summary | append "")
    
    let summary = ($summary | append "### Rust Dependencies")
    let total_outdated = count-rust-outdated
    let summary = ($summary | append $"- Total outdated Rust dependencies: ($total_outdated)")
    let summary = ($summary | append "")
    
    let summary = ($summary | append "### Components Checked")
    let components = [
        "- core/pluginz"
        "- core/macroz"
        "- core/registry"
        "- examples"
        "- plugins/ezpz-rust-ti"
        "- stubz"
    ]
    let summary = ($summary | append $components)
    let summary = ($summary | append "")
    
    # Save summary
    $summary | str join "\n" | save "dependency_summary.md"
    
    print "Dependency update summary generated"
}

def count-python-vulnerabilities [] {
    let audit_files = glob "**/audit.json"
    let total = $audit_files | reduce -f 0 { |file, acc|
        try {
            let report = open $file | from json
            $acc + ($report.vulnerabilities | length)
        } catch {
            $acc
        }
    }
    $total
}

def count-rust-outdated [] {
    let outdated_files = glob "**/cargo_outdated*.json"
    let total = $outdated_files | reduce -f 0 { |file, acc|
        try {
            let report = open $file | from json
            $acc + ($report.dependencies | length)
        } catch {
            $acc
        }
    }
    $total
}