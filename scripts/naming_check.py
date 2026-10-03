#!/usr/bin/env python3
"""
ApexCode Naming Check
Validates variable and function names for human-like patterns.
"""

import sys
import re
import subprocess
from pathlib import Path

OVER_DESCRIPTIVE_PATTERNS = [
    r"userAuthenticationToken",
    r"userAuthenticationInformation",
    r"configurationSettings",
    r"utilityFunction",
    r"helperMethod",
    r"implementationDetails",
    r"processingLogic",
    r"validationRules",
    r"errorHandling",
    r"exceptionManagement",
]

SHORTER_ALTERNATIVES = {
    "userAuthenticationToken": ["authToken", "user_tk", "auth"],
    "userAuthenticationInformation": ["authInfo", "user_info"],
    "configurationSettings": ["config", "settings", "cfg"],
    "utilityFunction": ["util", "helper"],
    "helperMethod": ["helper", "util"],
    "implementationDetails": ["impl", "details"],
    "processingLogic": ["logic", "processor"],
    "validationRules": ["rules", "validator"],
    "errorHandling": ["errorHandler", "err_handler"],
    "exceptionManagement": ["exceptionHandler", "exc_handler"],
}


def get_staged_files():
    """Get list of staged files."""
    result = subprocess.run(
        ["git", "diff", "--cached", "--name-only"],
        capture_output=True,
        text=True,
    )
    return result.stdout.strip().split("\n") if result.stdout.strip() else []


def check_file_naming(file_path):
    """Check a file for over-descriptive naming."""
    try:
        with open(file_path, "r", encoding="utf-8") as f:
            content = f.read()
    except Exception:
        return []

    findings = []

    for pattern in OVER_DESCRIPTIVE_PATTERNS:
        if re.search(r"\b" + pattern + r"\b", content):
            alternatives = SHORTER_ALTERNATIVES.get(pattern, [])
            findings.append({
                "pattern": pattern,
                "alternatives": alternatives,
            })

    return findings


def main():
    """Main entry point."""
    print("🔤 ApexCode: Checking naming conventions...")

    staged_files = get_staged_files()
    if not staged_files:
        print("✓ No files staged for commit")
        return 0

    total_findings = 0

    for file_path in staged_files:
        if not Path(file_path).exists():
            continue

        findings = check_file_naming(file_path)
        if findings:
            total_findings += len(findings)
            print(f"\n  {file_path}:")
            for finding in findings:
                pattern = finding["pattern"]
                alternatives = finding["alternatives"]
                alt_str = ", ".join(alternatives) if alternatives else "shorter alternative"
                print(f"    ⚠️  Consider: '{pattern}' → {alt_str}")

    if total_findings > 0:
        print(f"\n💡 Found {total_findings} over-descriptive name(s)")
        print("   Run 'apexcode humanize' to auto-fix, or use 'git commit --no-verify' to bypass")
        return 1

    print("✓ Naming conventions look good")
    return 0


if __name__ == "__main__":
    sys.exit(main())
