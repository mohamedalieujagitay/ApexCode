#!/usr/bin/env python3
"""
ApexCode Auto-Humanize Script
Automatically rewrites AI-style comments and code to be more human-like.
"""

import sys
import re
import subprocess
from pathlib import Path

WORD_REPLACEMENTS = {
    "authentication": "auth",
    "information": "info",
    "utility": "utils",
    "configuration": "config",
    "parameter": "param",
    "argument": "arg",
    "variable": "var",
    "function": "func",
    "synchronous": "sync",
    "asynchronous": "async",
    "implementation": "impl",
    "optimization": "opt",
    "performance": "perf",
    "validation": "val",
    "exception": "err",
    "error": "err",
    "message": "msg",
    "response": "resp",
    "request": "req",
    "identifier": "id",
    "reference": "ref",
    "pointer": "ptr",
    "buffer": "buf",
    "string": "str",
    "integer": "int",
    "boolean": "bool",
    "character": "char",
    "array": "arr",
    "object": "obj",
    "dictionary": "dict",
    "collection": "col",
    "sequence": "seq",
    "iterator": "iter",
}

COMMENT_TRANSFORMATIONS = [
    # Remove "This function/method/variable" prefixes
    (r"This function\s+", ""),
    (r"This method\s+", ""),
    (r"This variable\s+", ""),
    (r"This class\s+", ""),

    # Simplify explanations
    (r"Returns the\s+", "Returns "),
    (r"Gets the\s+", "Gets "),
    (r"Sets the\s+", "Sets "),
    (r"Checks if\s+", "Checks "),

    # Remove formal phrases
    (r"It is important to note that", ""),
    (r"Please note that", ""),
    (r"Note that", ""),
    (r"Make sure to", ""),
    (r"Ensure that", ""),
]


def get_staged_files():
    """Get list of staged files."""
    result = subprocess.run(
        ["git", "diff", "--cached", "--name-only"],
        capture_output=True,
        text=True,
    )
    return result.stdout.strip().split("\n") if result.stdout.strip() else []


def humanize_content(content):
    """Humanize code content."""
    humanized = content

    for formal, informal in WORD_REPLACEMENTS.items():
        # Match whole words only
        pattern = r"\b" + re.escape(formal) + r"\b"
        humanized = re.sub(pattern, informal, humanized, flags=re.IGNORECASE)

    for pattern, replacement in COMMENT_TRANSFORMATIONS:
        humanized = re.sub(pattern, replacement, humanized, flags=re.IGNORECASE)

    # Remove terminal periods from single-line comments
    humanized = re.sub(r"(//|#)\s*([^.]+)\.\s*$", r"\1 \2", humanized, flags=re.MULTILINE)

    # Simplify docblocks to inline comments where appropriate
    # Convert /** ... */ to // ... for simple comments
    humanized = re.sub(
        r"/\*\*\s*([^\*]+)\s*\*/",
        lambda m: f"// {m.group(1).strip()}",
        humanized,
    )

    return humanized


def humanize_file(file_path):
    """Humanize a single file."""
    try:
        with open(file_path, "r", encoding="utf-8") as f:
            content = f.read()
    except Exception:
        return False

    humanized = humanize_content(content)

    if humanized == content:
        return False

    with open(file_path, "w", encoding="utf-8") as f:
        f.write(humanized)

    subprocess.run(["git", "add", str(file_path)], capture_output=True)

    return True


def main():
    """Main entry point."""
    print("🎭 ApexCode: Auto-humanizing code...")

    staged_files = get_staged_files()
    if not staged_files:
        print("✓ No files staged for commit")
        return 0

    humanized_count = 0

    for file_path in staged_files:
        if not Path(file_path).exists():
            continue

        if humanize_file(file_path):
            humanized_count += 1
            print(f"  ✓ Humanized {file_path}")

    if humanized_count > 0:
        print(f"✓ Auto-humanized {humanized_count} file(s)")
    else:
        print("✓ No changes needed")

    return 0


if __name__ == "__main__":
    sys.exit(main())
