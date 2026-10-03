#!/usr/bin/env node
// Claude Code plugin hook. Whatever this prints to stdout lands in the model's context.
//   session: full ApexCode rule set (startup, resume, /clear, after compaction)
//   prompt:  one-line reminder on every prompt so long sessions don't drift

import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root =
  process.env.CLAUDE_PLUGIN_ROOT ||
  path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const mode = process.argv[2] === "prompt" ? "prompt" : "session";

const REMINDER =
  "ApexCode is active. For any code, comment, commit, PR or doc in this reply: follow the apexcode " +
  "skill (standing instructions, rules 1-12, workflow 10a-10c), keep all original code, and say only " +
  "what you actually verified.";

function ruleBody() {
  const raw = readFileSync(path.join(root, "rules", "apexcode.mdc"), "utf8").replace(/\r\n/g, "\n");
  return raw.startsWith("---\n") ? raw.slice(raw.indexOf("\n---\n", 4) + 5).trim() : raw.trim();
}

if (mode === "prompt") {
  console.log(REMINDER);
} else {
  try {
    console.log(
      [
        "<apexcode>",
        "ApexCode is installed and always on. Apply these rules to every response that writes or changes code, comments, commits, PRs or docs, even when the user doesn't mention ApexCode. Load the apexcode skill for the full reference before any non-trivial coding task.",
        "",
        ruleBody(),
        "</apexcode>",
      ].join("\n"),
    );
  } catch {
    // Rule file missing: still keep the agent on the skill rather than failing the session.
    console.log(REMINDER);
  }
}
