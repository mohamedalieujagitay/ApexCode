#!/usr/bin/env python3
"""
ApexCode Commit Entropy Check
Validates commit messages for human-like patterns.
"""

import sys
import re

BANNED_COMMIT_WORDS = [
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

AI_COMMIT_PATTERNS = [
    r"I have (implemented|created|developed|added)",
    r"Here is (the|a) (implementation|solution|fix)",
    r"Let me (explain|describe)",
    r"Please note that",
    r"It is important to",
]

MAX_SUBJECT_LENGTH = 72


def check_commit_message(message):
    """Check commit message for AI signatures."""
    lines = message.strip().split("\n")
    if not lines:
        return False, ["Empty commit message"]

    subject = lines[0]
    findings = []

    if len(subject) > MAX_SUBJECT_LENGTH:
        findings.append(f"Subject too long ({len(subject)} chars, max {MAX_SUBJECT_LENGTH})")

    for word in BANNED_COMMIT_WORDS:
        if word.lower() in subject.lower():
            findings.append(f"Banned word in subject: '{word}'")

    for pattern in AI_COMMIT_PATTERNS:
        if re.search(pattern, subject, re.IGNORECASE):
            findings.append(f"AI pattern detected: '{pattern}'")

    formal_indicators = ["This commit", "This change", "This PR", "This MR"]
    for indicator in formal_indicators:
        if subject.lower().startswith(indicator.lower()):
            findings.append(f"Formal language: '{indicator}'")

    # Check for excessive punctuation
    if subject.count(".") > 1:
        findings.append("Too many periods in subject")

    return len(findings) == 0, findings


def main():
    """Main entry point."""
    # Read commit message from file
    if len(sys.argv) > 1:
        commit_file = sys.argv[1]
        try:
            with open(commit_file, "r", encoding="utf-8") as f:
                message = f.read()
        except Exception:
            print("⚠️  Could not read commit message file")
            return 1
    else:
        print("⚠️  No commit message file provided")
        return 1

    print("📝 ApexCode: Checking commit message entropy...")

    is_valid, findings = check_commit_message(message)

    if not is_valid:
        print("\n⚠️  Commit message needs improvement:")
        for finding in findings:
            print(f"  - {finding}")

        print("\n💡 Tips for human-like commit messages:")
        print("  • Keep it brief (5-7 words max)")
        print("  • Focus on 'what' not 'how'")
        print("  • Use lowercase for casual commits")
        print("  • Avoid formal words like 'implementing', 'optimizing'")
        print("  • Example: 'fixed login lag' instead of 'Implemented robust authentication'")

        return 1

    print("✓ Commit message looks human")
    return 0


if __name__ == "__main__":
    sys.exit(main())
