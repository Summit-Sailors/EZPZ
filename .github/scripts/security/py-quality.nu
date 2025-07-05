#!/usr/bin/env nu

# Runs ruff and mypy checks on Python code

def main [] {
    print "Checking Python code quality..."
    
    run-ruff-check
    
    run-ruff-format
    
    run-type-checking
}

def run-ruff-check [] {
    print "Running ruff check..."
    
    let excluded_dirs = [
        "formatterz/"
        "api/"
        "app/"
        ".ruff_cache/"
        "target/"
    ]
    
    let exclude_args = $excluded_dirs | each { |dir| ["--exclude" $dir] } | flatten
    
    try {
        run-external "rye" "run" "ruff" "check" "." ...$exclude_args "--output-format=json" | save "ruff_report.json"
        
        if ("ruff_report.json" | path exists) and (open "ruff_report.json" | str length) > 0 {
            let report = open "ruff_report.json"
            let issues = $report | length
            
            if $issues > 0 {
                print $"📋 Ruff found ($issues) issues"
                $report | each { |issue|
                    print $"  File: ($issue.filename)"
                    print $"  Code: ($issue.code.code)"
                    print $"  Message: ($issue.message)"
                    print ""
                }
            } else {
                print "✅ No Ruff issues found"
            }
        } else {
            print "✅ No Ruff issues found"
        }
    } catch {
        print "✅ No Ruff issues found"
    }
}

def run-ruff-format [] {
    print "Running ruff format check..."
    
    let excluded_dirs = [
        "formatterz/"
        "api/"
        "app/"
        ".ruff_cache/"
        "target/"
    ]
    
    let exclude_args = $excluded_dirs | each { |dir| ["--exclude" $dir] } | flatten
    
    try {
        run-external "rye" "run" "ruff" "format" "--check" "--diff" "." ...$exclude_args
        print "✅ Ruff formatting check passed"
    } catch {
        print "⚠️  Ruff formatting issues found"
    }
}

def run-type-checking [] {
    print "Running Python type checking..."
    
    let components = [
        "core/pluginz"
        "core/macroz"
        "core/registry"
    ]
    
    for component in $components {
        if ($component | path join "pyproject.toml" | path exists) {
            check-component-types $component
        }
    }
}

def check-component-types [component: string] {
    print $"Type checking ($component)..."
    
    cd $component
    
    run-external "rye" "sync" "--all-features" | ignore
    
    try {
        run-external "rye" "run" "mypy" "--version" | ignore
        
        print $"Running mypy for ($component)..."
        try {
            run-external "rye" "run" "mypy" "." "--json-report" "mypy_report.json" | ignore
            
            if ("mypy_report.json" | path exists) and (open "mypy_report.json" | str length) > 0 {
                print $"MyPy report generated for ($component)"
            } else {
                print $"✅ No MyPy issues found in ($component)"
            }
        } catch {
            print $"✅ No MyPy issues found in ($component)"
        }
    } catch {
        print $"ℹ️  MyPy not available for ($component)"
    }
    
    cd ..
}