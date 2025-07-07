#!/usr/bin/env nu

def main [
    operation?: string,
    dry_run?: bool,
    event_name?: string,
    discover_result?: string,
    has_changes?: string,
    plugins_to_register?: string,
    plugins_to_update?: string,
    test_result?: string,
    register_result?: string,
    publish_result?: string
] {
    let github_step_summary = ($env.GITHUB_STEP_SUMMARY? | default "/dev/stdout")
    
    def append_to_report [content: string] {
        $content | save --append $github_step_summary
    }
    
    append_to_report "# 📊 EZPZ Plugin Workflow Report\n\n"
    
    append_to_report "## Workflow Configuration\n"
    append_to_report $"- **Operation**: ($operation | default 'automatic')\n"
    append_to_report $"- **Dry Run**: ($dry_run | default 'false')\n"
    append_to_report $"- **Trigger**: ($event_name | default 'unknown')\n\n"
    
    append_to_report "## Plugin Discovery Results\n"
    if ($discover_result | default "unknown") == "success" {
        append_to_report "✅ **Plugin Discovery**: Success\n"
        append_to_report $"- **Has Changes**: ($has_changes | default 'unknown')\n"
        
        # Count plugins
        let reg_count = if ($plugins_to_register | default "[]") != "[]" {
            ($plugins_to_register | from json | length)
        } else { 0 }
        
        let upd_count = if ($plugins_to_update | default "[]") != "[]" {
            ($plugins_to_update | from json | length)
        } else { 0 }
        
        append_to_report $"- **Plugins to Register**: ($reg_count)\n"
        append_to_report $"- **Plugins to Update**: ($upd_count)\n"
    } else {
        append_to_report "❌ **Plugin Discovery**: Failed\n"
    }
    append_to_report "\n"
    
    append_to_report "## Test Results\n"
    match ($test_result | default "unknown") {
        "success" => { append_to_report "✅ **Plugin Tests**: All tests passed\n" },
        "skipped" => { append_to_report "⏭️ **Plugin Tests**: Skipped (no changes detected)\n" },
        _ => { append_to_report "❌ **Plugin Tests**: Some tests failed\n" }
    }
    append_to_report "\n"
    
    append_to_report "## Registration and Updates\n"
    match ($register_result | default "unknown") {
        "success" => { append_to_report "✅ **Registry Operations**: Success\n" },
        "skipped" => { append_to_report "⏭️ **Registry Operations**: Skipped\n" },
        _ => { append_to_report "❌ **Registry Operations**: Failed\n" }
    }
    append_to_report "\n"
    
    append_to_report "## Publishing Results\n"
    match ($publish_result | default "unknown") {
        "success" => { append_to_report "✅ **Publishing**: Success\n" },
        "skipped" => { append_to_report "⏭️ **Publishing**: Skipped\n" },
        _ => { append_to_report "❌ **Publishing**: Failed\n" }
    }
    append_to_report "\n"
    
    append_to_report "## Overall Status\n"
    let overall_success = (
        ($discover_result | default "unknown") == "success" and
        ($test_result | default "unknown") != "failure" and
        ($register_result | default "unknown") != "failure" and
        ($publish_result | default "unknown") != "failure"
    )
    
    if $overall_success {
        append_to_report "🎉 **Workflow completed successfully!**\n"
    } else {
        append_to_report "⚠️ **Workflow completed with issues. Check individual job results.**\n"
    }
    
    append_to_report "\n"
    append_to_report "---\n"
    append_to_report $"*Report generated at (date now | format date '%Y-%m-%d %H:%M:%S')*\n"
}