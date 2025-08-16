#!/usr/bin/env nu

def main [command: string, package_name: string, plugin_path: string, --dry-run: string = "false"] {
  cd $plugin_path
  
  let dry_run = ($dry_run == "true")
  let actual_path = if ($plugin_path | str ends-with $package_name) { "." } else { $plugin_path }

  match $command {
    "validate" => { validate_plugin $package_name $actual_path }
    "build" => { build_plugin $package_name $actual_path }
    "test" => { test_plugin $package_name $actual_path }
    "test-pipeline" => { test_pipeline $package_name $actual_path }
    "publish" => { publish_plugin $package_name $actual_path $dry_run }
    _ => { error make { msg: $"Unknown command: ($command)" } }
  }
}

def test_pipeline [package_name: string, plugin_path: string] {
  validate_plugin $package_name $plugin_path
  build_plugin $package_name $plugin_path
  test_plugin $package_name $plugin_path
  print $"✅ Plugin ($package_name) passed all tests"
}

def test_plugin [package_name: string, plugin_path: string] {
  cd $plugin_path
  
  let test_dir = ($plugin_path | path join "tests")
  let test_dir_alt = ($plugin_path | path join "test")
  if ((get_path_type $test_dir) == "dir" or (get_path_type $test_dir_alt) == "dir") {
    try {
      ^python3 -m pytest -v
      print "✅ Python tests passed"
    } catch {
      print "❌ Python tests failed"
      exit 1
    }
  }
  
  let cargo_toml = ($plugin_path | path join "Cargo.toml")
  if (get_path_type $cargo_toml) == "file" {
    try {
      ^cargo test
      print "✅ Rust tests passed"
    } catch {
      print "❌ Rust tests failed"
      exit 1
    }
  }
}

def validate_plugin [package_name: string, plugin_path: string] {
  let pyproject_type = (get_path_type ($plugin_path | path join "pyproject.toml"))
  let cargo_type = (get_path_type ($plugin_path | path join "Cargo.toml"))
  
  let has_pyproject = ($pyproject_type == "file")
  let has_cargo = ($cargo_type == "file")
  
  if not ($has_pyproject or $has_cargo) {
    print $"❌ Missing both pyproject.toml and Cargo.toml in ($plugin_path)"
    exit 1
  }

  if $has_pyproject {
    print "✅ Found pyproject.toml"
    let py_typed_type = (get_path_type ($plugin_path | path join "python" $package_name "py.typed"))
    if ($py_typed_type == "file") {
      print "✅ Found py.typed for type hints"
    }
  }

  if $has_cargo {
    print "✅ Found Cargo.toml"
    let lib_rs_type = (get_path_type ($plugin_path | path join "src" "lib.rs"))
    let main_rs_type = (get_path_type ($plugin_path | path join "src" "main.rs"))
    if not (($lib_rs_type == "file") or ($main_rs_type == "file")) {
      print "❌ Rust project missing src/lib.rs or src/main.rs"
      exit 1
    }
    print "✅ Found Rust source file (lib.rs or main.rs)"
  }

  let init_found = check_init_py $package_name $plugin_path
  if not $init_found {
    print "❌ Could not find __init__.py with register_plugin function"
    exit 1
  }

  let dist_dir = ($plugin_path | path join "dist")
  let dist_type = (get_path_type $dist_dir)
  if ($dist_type == "dir") {
    let dist_files = (glob ($dist_dir | path join "*"))
    if ($dist_files | length) > 0 {
      try {
        ^twine check ...$dist_files
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
  let cleanup_patterns = ["dist" "build" "*.egg-info"]
  for pattern in $cleanup_patterns {
    try {
      let items = (glob $pattern)
      for item in $items {
        rm -rf $item
      }
    }
  }

  let cargo_toml = ($plugin_path | path join "Cargo.toml")
  let pyproject_toml = ($plugin_path | path join "pyproject.toml")
  
  # For mixed projects, build Rust first if both exist
  if (($cargo_toml | path exists) and ($pyproject_toml | path exists)) {
    try {
      ^cargo fetch
      ^cargo build --release
      print "✅ Rust build successful"
    } catch {
      print "❌ Rust build failed"
      exit 1
    }
    
    try {
      # Use maturin directly for mixed projects instead of rye
      ^maturin build --release
      print "✅ Python/Rust mixed build successful"
    } catch {
      print "❌ Python/Rust mixed build failed"
      exit 1
    }
  } else {
    # Handle pure Python projects
    if ($pyproject_toml | path exists) {
      try {
        ^rye build
        print "✅ Python build successful"
      } catch {
        print "❌ Python build failed"
        exit 1
      }
    }
    
    # Handle pure Rust projects
    if ($cargo_toml | path exists) {
      try {
        ^cargo fetch
        ^cargo build --release
        print "✅ Rust build successful"
      } catch {
        print "❌ Rust build failed"
        exit 1
      }
    }
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
    let dist_dir = ($plugin_path | path join "dist")
    if ($dist_dir | path exists) {
      let dist_files = (glob ($dist_dir | path join "*"))
      if ($dist_files | length) > 0 {
        for attempt in 1..$max_attempts {
          try {
            ^twine upload ...$dist_files
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
    for attempt in 1..$max_attempts {
      try {
        ^cargo publish
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

def get_path_type [path: string] {
  try {
    ($path | path type)
  } catch {
    null
  }
}