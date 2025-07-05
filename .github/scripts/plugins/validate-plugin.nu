#!/usr/bin/env nu

def main [package_name: string, plugin_path: string] {
    print $"🔍 Validating plugin structure for: ($package_name)"
    
    let has_pyproject = ($plugin_path | path join "pyproject.toml" | path exists)
    let has_cargo = ($plugin_path | path join "Cargo.toml" | path exists)
    
    if $has_pyproject {
        print "✅ Found pyproject.toml"
    }
    
    if $has_cargo {
        print "✅ Found Cargo.toml"
    }
    
    if not ($has_pyproject or $has_cargo) {
        print $"❌ Missing both pyproject.toml and Cargo.toml in ($plugin_path)"
        exit 1
    }
    
    # Check for __init__.py with register_plugin function
    let init_found = check_init_py $package_name $plugin_path
    
    if not $init_found {
        print "❌ Could not find __init__.py with register_plugin function in any expected location"
        exit 1
    }
    
    let tests_found = check_tests_directory $plugin_path
    
    if not $tests_found {
        print "❌ Missing tests directory in expected locations"
        exit 1
    }
    
    if $has_cargo {
        let lib_rs = ($plugin_path | path join "src" "lib.rs" | path exists)
        let main_rs = ($plugin_path | path join "src" "main.rs" | path exists)
        
        if not ($lib_rs or $main_rs) {
            print "❌ Rust project missing src/lib.rs or src/main.rs"
            exit 1
        } else {
            print "✅ Found Rust source files"
        }
    }
    
    # Python package structure for hybrid projects
    if $has_pyproject and ($plugin_path | path join "python" | path exists) {
        print "✅ Detected hybrid Python/Rust project structure"
        
        let py_typed = ($plugin_path | path join "python" $package_name "py.typed" | path exists)
        if $py_typed {
            print "✅ Found py.typed for type hints"
        }
        
        let stub_files = (glob ($plugin_path | path join "python" $package_name "*.pyi") | length)
        if $stub_files > 0 {
            print "✅ Found Python stub files"
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
    
    # recursive search for any __init__.py with register_plugin
    print "🔍 Searching recursively for __init__.py with register_plugin..."
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

def check_tests_directory [plugin_path: string] {
    let test_paths = [
        ($plugin_path | path join "tests"),
        ($plugin_path | path join "python" "tests"),
        ($plugin_path | path join "src" "tests")
    ]
    
    for test_path in $test_paths {
        if ($test_path | path exists) {
            print $"✅ Found tests directory"
            return true
        }
    }
    
    return false
}