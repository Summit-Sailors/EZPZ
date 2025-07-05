#!/usr/bin/env nu

def main [plugin_path: string] {
    let dist_dir = ($plugin_path | path join "dist")
    if ($dist_dir | path exists) {
        print "🔍 Validating package..."
        let dist_files = (glob ($dist_dir | path join "*"))
        if ($dist_files | length) > 0 {
            twine check ...$dist_files
        }
    }
}