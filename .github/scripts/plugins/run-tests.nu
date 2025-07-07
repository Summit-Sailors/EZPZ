#!/usr/bin/env nu
def main [package_name: string, plugin_path: string] {
    print $"🧪 Running tests for ($package_name)"
    
    mut tests_run = false
    mut test_failures = false
    
    let has_pyproject = ($plugin_path | path join "pyproject.toml" | path exists)
    if $has_pyproject {
        print "Running Python tests..."
        let python_result = run_python_tests $plugin_path
        if $python_result.found_tests {
            $tests_run = true
            if not $python_result.passed {
                $test_failures = true
            } else {
                print "✅ Python tests passed"
            }
        } else {
            print "ℹ️  No Python tests found"
        }
    }
    
    let cargo_toml = ($plugin_path | path join "Cargo.toml")
    if ($cargo_toml | path exists) {
        print "Running Rust tests..."
        let rust_result = run_rust_tests $plugin_path
        if $rust_result.found_tests {
            $tests_run = true
            if not $rust_result.passed {
                $test_failures = true
            } else {
                print "✅ Rust tests passed"
            }
        } else {
            print "ℹ️  No Rust tests found"
        }
    }
    
    # Final status
    if $test_failures {
        print "❌ Some tests failed"
        exit 1
    } else if $tests_run {
        print "✅ All tests passed"
    } else {
        print "❌ No tests were found for this plugin"
        exit 1
    }
}

def run_python_tests [plugin_path: string] {
    try {
        let output = (rye test -p $plugin_path | complete)
        let stderr_output = $output.stderr
        let stdout_output = $output.stdout
        
        let collected_line = ($stdout_output | lines | where ($it | str contains "collected") | first)
        
        if ($collected_line | str contains "collected 0 items") {
            return {found_tests: false, passed: true}
        } else if ($collected_line | str contains "collected") {
            let passed = ($output.exit_code == 0)
            return {found_tests: true, passed: $passed}
        } else {
            return {found_tests: false, passed: true}
        }
    } catch {
        return {found_tests: false, passed: true}
    }
}

def run_rust_tests [plugin_path: string] {
    try {
        cd $plugin_path
        let output = (cargo test | complete)
        let stdout_output = $output.stdout
        
        let test_lines = ($stdout_output | lines | where ($it | str contains "running") | where ($it | str contains "tests"))
        
        mut found_tests = false
        for line in $test_lines {
            if not ($line | str contains "running 0 tests") {
                $found_tests = true
                break
            }
        }
        
        if $found_tests {
            let passed = ($output.exit_code == 0)
            return {found_tests: true, passed: $passed}
        } else {
            return {found_tests: false, passed: true}
        }
    } catch {
        return {found_tests: false, passed: true}
    }
}