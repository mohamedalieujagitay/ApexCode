#!/usr/bin/env python3
"""
ApexCode Stealth Check Script
Scans staged files for AI-signature patterns and blocks commits if detected.
"""

import sys
import re
import subprocess
from pathlib import Path

BANNED_WORDS = [
    "leveraging",
    "comprehensive",
    "meticulous",
    "ensure",
    "robust",
    "seamless",
    "enhanced",
    "utilizing",
    "facilitating",
    "implementing",
    "optimizing",
    "streamlining",
]

AI_PATTERNS = [
    r"I have (implemented|created|developed|added)",
    r"Here is (the|a) (implementation|solution|code)",
    r"Let me (explain|describe|show)",
    r"Please note that",
    r"It is important to",
    r"This (code|implementation|solution) (provides|offers|ensures)",
]

MAX_AI_SIGNATURES = 3


def get_staged_files():
    """Get list of staged files."""
    result = subprocess.run(
        ["git", "diff", "--cached", "--name-only"],
        capture_output=True,
        text=True,
    )
    return result.stdout.strip().split("\n") if result.stdout.strip() else []


def check_file_for_ai_signatures(file_path):
    """Check a file for AI signatures."""
    try:
        with open(file_path, "r", encoding="utf-8") as f:
            content = f.read()
    except Exception:
        return 0, []

    signature_count = 0
    found_patterns = []

    for word in BANNED_WORDS:
        if word.lower() in content.lower():
            signature_count += 1
            found_patterns.append(f"Banned word: '{word}'")

    for pattern in AI_PATTERNS:
        matches = re.findall(pattern, content, re.IGNORECASE)
        if matches:
            signature_count += len(matches)
            found_patterns.append(f"AI pattern: '{pattern}'")

    docblock_count = content.count("/**") + content.count('"""')
    if docblock_count > 5:
        signature_count += 1
        found_patterns.append("Excessive docblocks")

    return signature_count, found_patterns


def main():
    """Main entry point."""
    print("🕵️ ApexCode: Checking for AI signatures...")

    staged_files = get_staged_files()
    if not staged_files:
        print("✓ No files staged for commit")
        return 0

    total_signatures = 0
    all_findings = []

    for file_path in staged_files:
        if not Path(file_path).exists():
            continue

        signatures, findings = check_file_for_ai_signatures(file_path)
        total_signatures += signatures
        all_findings.extend([(file_path, f) for f in findings])

    if total_signatures > MAX_AI_SIGNATURES:
        print(f"\n⚠️  Found {total_signatures} AI signatures (max {MAX_AI_SIGNATURES} allowed)")
        print("\nDetected patterns:")
        for file_path, finding in all_findings[:10]:  # Show first 10
            print(f"  {file_path}: {finding}")

        print("\n💡 Run 'apexcode humanize' to fix, or use 'git commit --no-verify' to bypass")
        return 1

    print(f"✓ Stealth check passed ({total_signatures} signatures found)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
