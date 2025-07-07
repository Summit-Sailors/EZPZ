import os
import json
from typing import Any
from pathlib import Path


def create_sarif() -> None:
  sarif: dict[str, Any] = {"version": "2.1.0", "$schema": "https://schemastore.azurewebsites.net/schemas/json/sarif-2.1.0.json", "runs": []}

  # the source root for relative paths in SARIF
  # to make the links in GitHub Security tab clickable
  source_root = os.getenv("GITHUB_WORKSPACE", Path.cwd())

  if Path("rust_audit.json").exists():
    with Path.open(Path("rust_audit.json")) as f:
      rust_data = json.load(f)

    results = [
      {
        "ruleId": f"rust-audit-{vuln.get('advisory', {}).get('id', 'unknown')}",
        "message": {
          "text": f"Vulnerability in {vuln.get('package', {}).get('name', 'unknown')}@{vuln.get('package', {}).get('version', 'unknown')}: {vuln.get('advisory', {}).get('title', 'Unknown')}"
        },
        "level": "error",  # Cargo audit results are typically severe enough for 'error'
        "locations": [
          {
            "physicalLocation": {
              "artifactLocation": {"uri": "Cargo.toml"},
              # "region": {"startLine": line_number}
            }
          }
        ],
      }
      for vuln in rust_data.get("vulnerabilities", {}).get("list", [])
    ]

    sarif["runs"].append({"tool": {"driver": {"name": "Cargo Audit", "version": "1.0.0"}}, "results": results})

  # Process Bandit
  if Path("bandit_report.json").exists():
    with Path.open(Path("bandit_report.json")) as f:
      bandit_data = json.load(f)

    results = []
    for issue in bandit_data.get("results", []):
      relative_path = Path(issue.get("filename", "")).relative_to(source_root).as_posix()
      level = "note"
      if issue.get("issue_severity", "").upper() == "MEDIUM":
        level = "warning"
      elif issue.get("issue_severity", "").upper() == "HIGH":
        level = "error"

      results.append(
        {
          "ruleId": f"bandit-{issue.get('test_id', 'unknown')}",
          "message": {"text": issue.get("issue_text", "Security issue")},
          "level": level,
          "locations": [{"physicalLocation": {"artifactLocation": {"uri": relative_path}, "region": {"startLine": issue.get("line_number", 1)}}}],
        }
      )

    sarif["runs"].append({"tool": {"driver": {"name": "Bandit", "version": "1.0.0"}}, "results": results})

  if Path("pip_audit_raw.json").exists():
    with Path.open(Path("pip_audit_raw.json")) as f:
      pip_data = json.load(f)

    results = []
    for vuln in pip_data.get("vulnerabilities", []):
      # pip-audit -> package, not a specific line in source
      results.append(
        {
          "ruleId": f"pip-audit-{vuln.get('id', 'unknown')}",
          "message": {
            "text": f"Vulnerability in {vuln.get('package', {}).get('name', 'unknown')}@{vuln.get('package', {}).get('version', 'unknown')}: {vuln.get('description', 'Unknown')}"
          },
          "level": "error",
          "locations": [{"physicalLocation": {"artifactLocation": {"uri": "pyproject.toml"}}}],  # to main config
        }
      )

    sarif["runs"].append({"tool": {"driver": {"name": "Pip Audit", "version": "1.0.0"}}, "results": results})

  with Path.open(Path("security-results.sarif"), "w") as f:
    json.dump(sarif, f, indent=2)


if __name__ == "__main__":
  create_sarif()
