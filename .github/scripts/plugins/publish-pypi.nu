#!/usr/bin/env nu

def main [package_name: string, plugin_path: string] {
    print $"🚀 Publishing ($package_name) to PyPI..."
    
    let dist_dir = ($plugin_path | path join "dist")
    if ($dist_dir | path exists) {
        let dist_files = (glob ($dist_dir | path join "*"))
        if ($dist_files | length) > 0 {
            twine upload ...$dist_files
            print $"✅ Successfully published ($package_name) to PyPI"
        } else {
            print $"⚠️ No distribution files found for ($package_name)"
        }
    } else {
        print $"⚠️ No distribution directory found for ($package_name)"
    }
}