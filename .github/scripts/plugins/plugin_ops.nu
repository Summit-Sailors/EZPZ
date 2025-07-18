#!/usr/bin/env nu

def main [command: string, package_name: string, plugin_path: string, --dry-run: string = "false"] {
    cd $plugin_path
    
    let dry_run_bool = ($dry_run == "true")

    match $command {
        "validate" => { validate_plugin $package_name $plugin_path },
        "build" => { build_plugin $package_name $plugin_path },
        "test" => { run_tests $package_name $plugin_path },
        "publish" => { publish_plugin $package_name $plugin_path $dry_run_bool },
        _ => { print $"❌ Unknown command: ($command)"; exit 1 }
    }
}

def validate_plugin [package_name: string, plugin_path: string] {
    print $"🔍 Validating plugin structure for: ($package_name)"
    let has_pyproject = ($plugin_path | path join "pyproject.toml" | path exists)
    let has_cargo = ($plugin_path | path join "Cargo.toml" | path exists)

    if not ($has_pyproject or $has_cargo) {
        print $"❌ Missing both pyproject.toml and Cargo.toml in ($plugin_path)"
        exit 1
    }

    if $has_pyproject {
        print "✅ Found pyproject.toml"
        let py_typed = ($plugin_path | path join "python" $package_name "py.typed" | path exists)
        if $py_typed {
            print "✅ Found py.typed for type hints"
        }
    }

    if $has_cargo {
        print "✅ Found Cargo.toml"
        let lib_rs = ($plugin_path | path join "src" "lib.rs" | path exists)
        let main_rs = ($plugin_path | path join "src" "main.rs" | path exists)
        if not ($lib_rs or $main_rs) {
            print "❌ Rust project missing src/lib.rs or src/main.rs"
            exit 1
        }
    }

    let init_found = check_init_py $package_name $plugin_path
    if not $init_found {
        print "❌ Could not find __init__.py with register_plugin function"
        exit 1
    }

    let dist_dir = ($plugin_path | path join "dist")
    if ($dist_dir | path exists) {
        let dist_files = (glob ($dist_dir | path join "*"))
        if ($dist_files | length) > 0 {
            print "🔍 Validating package..."
            try {
                twine check $dist_files
                print "✅ Package validation passed"
            } catch {
                print "❌ Package validation failed"
                exit 1
            }
        }
    }

    print "✅ Plugin structure validation passed"
}

def check_init_py [package_name: string, plugin_path: string] {
    let patterns = [
        ($plugin_path | path join "python" $package_name "__init__.py"),
        ($plugin_path | path join "src" $package_name "__init__.py"),
        ($plugin_path | path join $package_name "__init__.py"),
        ($plugin_path | path join "__init__.py")
    ]

    for pattern in $patterns {
        if ($pattern | path exists) {
            let content = (open $pattern)
            if ($content | str contains "def register_plugin") {
                print $"✅ Found register_plugin function in ($pattern)"
                return true
            }
        }
    }

    let found_files = (glob ($plugin_path | path join "**" "__init__.py") | each { |file|
        let content = (open $file)
        if ($content | str contains "def register_plugin") {
            $file
        }
    } | compact)

    if ($found_files | length) > 0 {
        print $"✅ Found register_plugin function in ($found_files | first)"
        return true
    }
    return false
}

def build_plugin [package_name: string, plugin_path: string] {
    print $"🏗️ Building plugin: ($package_name)"
    let pyproject_toml = ($plugin_path | path join "pyproject.toml")
    if ($pyproject_toml | path exists) {
        print "📦 Building Python package..."
        try {
            rye build
            print "✅ Python build successful"
        } catch {
            print "❌ Python build failed"
            exit 1
        }
    }

    let cargo_toml = ($plugin_path | path join "Cargo.toml")
    if ($cargo_toml | path exists) {
        print "🦀 Building Rust package..."
        try {
            cargo fetch
            cargo build --release
            print "✅ Rust build successful"
        } catch {
            print "❌ Rust build failed"
            exit 1
        }
    }
}

def run_tests [package_name: string, plugin_path: string] {
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
            }
        }
    }

    let has_cargo = ($plugin_path | path join "Cargo.toml" | path exists)
    if $has_cargo {
        print "Running Rust tests..."
        let rust_result = run_rust_tests $plugin_path
        if $rust_result.found_tests {
            $tests_run = true
            if not $rust_result.passed {
                $test_failures = true
            }
        }
    }

    if $test_failures {
        print "❌ Some tests failed"
        exit 1
    } else if $tests_run {
        print "✅ All tests passed"
    } else {
        print "⚠️ No tests were found for this plugin"
        # Allow empty test suites as a warning, configurable in ezpz.toml
        if ($env.EZPZ_ALLOW_EMPTY_TESTS? | default "false") == "true" {
            print "✅ Continuing despite no tests found"
        } else {
            exit 1
        }
    }
}

def run_python_tests [plugin_path: string] {
    try {
        let output = (rye test -p $plugin_path | complete)
        let collected_line = ($output.stdout | lines | where ($it | str contains "collected") | first)
        if ($collected_line | str contains "collected 0 items") {
            return {found_tests: false, passed: true}
        } else if ($collected_line | str contains "collected") {
            return {found_tests: true, passed: ($output.exit_code == 0)}
        }
    } catch {
        return {found_tests: false, passed: true}
    }
}

def run_rust_tests [plugin_path: string] {
    try {
        cd $plugin_path
        let output = (cargo test | complete)
        let test_lines = ($output.stdout | lines | where ($it | str contains "running") | where ($it | str contains "tests"))
        mut found_tests = false
        for line in $test_lines {
            if not ($line | str contains "running 0 tests") {
                $found_tests = true
                break
            }
        }
        if $found_tests {
            return {found_tests: true, passed: ($output.exit_code == 0)}
        }
    } catch {
        return {found_tests: false, passed: true}
    }
}

def publish_plugin [package_name: string, plugin_path: string, dry_run: bool] {
    if $dry_run {
        print $"🏃 DRY RUN: Would publish ($package_name)"
        return
    }

    let max_attempts = 3
    let pyproject_toml = ($plugin_path | path join "pyproject.toml")
    if ($pyproject_toml | path exists) {
        print $"🚀 Publishing ($package_name) to PyPI..."
        let dist_dir = ($plugin_path | path join "dist")
        if ($dist_dir | path exists) {
            let dist_files = (glob ($dist_dir | path join "*"))
            if ($dist_files | length) > 0 {
                for attempt in 1..$max_attempts {
                    try {
                        twine upload $dist_files
                        print $"✅ Successfully published ($package_name) to PyPI"
                        break
                    } catch {
                        print $"⚠️ Attempt ($attempt) failed for ($package_name)"
                        if $attempt == $max_attempts {
                            print $"❌ Failed to publish ($package_name) to PyPI after ($max_attempts) attempts"
                            exit 1
                        }
                        sleep 5sec
                    }
                }
            } else {
                print $"⚠️ No distribution files found for ($package_name)"
            }
        }
    }

    let cargo_toml = ($plugin_path | path join "Cargo.toml")
    if ($cargo_toml | path exists) {
        print $"🦀 Publishing ($package_name) to crates.io..."
        for attempt in 1..$max_attempts {
            try {
                cargo publish
                print $"✅ Successfully published ($package_name) to crates.io"
                break
            } catch {
                print $"⚠️ Attempt ($attempt) failed for ($package_name)"
                if $attempt == $max_attempts {
                    print $"❌ Failed to publish ($package_name) to crates.io after ($max_attempts) attempts"
                    exit 1
                }
                sleep 5sec
            }
        }
    }
}