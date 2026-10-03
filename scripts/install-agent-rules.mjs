#!/usr/bin/env node
// Installs ApexCode into a project so every supported coding agent loads it on every session.
//
//   node scripts/install-agent-rules.mjs [target-dir] [--only=claude,cursor,...] [--no-skill] [--dry-run]
//
// Dedicated rule files are written whole. Shared instruction files (AGENTS.md, CLAUDE.md, ...) get a
// marked block appended, and re-running replaces only that block. Files we don't own are never
// overwritten.

import { promises as fs } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const START = "<!-- apexcode:start -->";
const END = "<!-- apexcode:end -->";
const OWNED = "<!-- apexcode:managed -->";

const args = process.argv.slice(2);
const flag = (name) => args.find((a) => a === `--${name}` || a.startsWith(`--${name}=`));
const target = path.resolve(args.find((a) => !a.startsWith("--")) ?? process.cwd());
const only = flag("only")
  ?.split("=")[1]
  ?.split(",")
  .map((s) => s.trim().toLowerCase());
const dryRun = Boolean(flag("dry-run"));
const withSkill = !flag("no-skill");

const raw = (await fs.readFile(path.join(repoRoot, "rules", "apexcode.mdc"), "utf8")).replace(
  /\r\n/g,
  "\n",
);
const body = raw.startsWith("---\n") ? raw.slice(raw.indexOf("\n---\n", 4) + 5).trim() : raw.trim();

const header =
  "ApexCode is always on in this project. Apply these rules to every response that writes or changes " +
  "code, comments, commits, PRs or docs, even when nobody mentions ApexCode.";
const block = `${START}\n${header}\n\n${body}\n${END}`;
const dedicated = (frontmatter) => `${frontmatter}${OWNED}\n\n${header}\n\n${body}\n`;

// agent -> files. "file" is written whole; "block" is appended to a shared file.
const targets = {
  claude: [
    { kind: "file", rel: ".claude/rules/apexcode.md", content: dedicated("") },
    {
      kind: "block",
      rel: "CLAUDE.md",
      content: `${START}\nApexCode is always on in this project. Follow @.claude/rules/apexcode.md for all code, comments, commits, PRs and docs.\n${END}`,
    },
  ],
  cursor: [
    {
      kind: "file",
      rel: ".cursor/rules/apexcode.mdc",
      content: dedicated(
        "---\ndescription: ApexCode - production-quality, human-style code and prose\nalwaysApply: true\n---\n\n",
      ),
    },
  ],
  windsurf: [
    {
      kind: "file",
      rel: ".windsurf/rules/apexcode.md",
      content: dedicated("---\ntrigger: always_on\n---\n\n"),
    },
  ],
  antigravity: [
    {
      kind: "file",
      rel: ".agent/rules/apexcode.md",
      content: dedicated("---\ntrigger: always_on\n---\n\n"),
    },
  ],
  cline: [{ kind: "file", rel: ".clinerules/apexcode.md", content: dedicated("") }],
  copilot: [{ kind: "block", rel: ".github/copilot-instructions.md", content: block }],
  agents: [{ kind: "block", rel: "AGENTS.md", content: block }],
  gemini: [{ kind: "block", rel: "GEMINI.md", content: block }],
};

async function read(file) {
  try {
    return await fs.readFile(file, "utf8");
  } catch (err) {
    if (err.code === "ENOENT") return null;
    throw err;
  }
}

async function write(file, content) {
  if (dryRun) return;
  await fs.mkdir(path.dirname(file), { recursive: true });
  await fs.writeFile(file, content);
}

async function installFile({ rel, content }) {
  const file = path.join(target, rel);
  const existing = await read(file);
  if (existing !== null && !existing.includes(OWNED)) {
    return `skip     ${rel} (exists and isn't managed by ApexCode)`;
  }
  if (existing === content) return `ok       ${rel}`;
  await write(file, content);
  return `${existing === null ? "created " : "updated "} ${rel}`;
}

async function installBlock({ rel, content }) {
  const file = path.join(target, rel);
  const existing = await read(file);
  if (existing === null) {
    await write(file, `${content}\n`);
    return `created  ${rel}`;
  }
  const s = existing.indexOf(START);
  const e = existing.indexOf(END);
  if (s !== -1 && e > s) {
    const next = existing.slice(0, s) + content + existing.slice(e + END.length);
    if (next === existing) return `ok       ${rel}`;
    await write(file, next);
    return `updated  ${rel}`;
  }
  await write(file, `${existing.replace(/\s*$/, "")}\n\n${content}\n`);
  return `appended ${rel}`;
}

async function copySkill() {
  const src = path.join(repoRoot, "skills", "apexcode");
  const dest = path.join(target, ".claude", "skills", "apexcode");
  if (path.resolve(src) === path.resolve(dest)) return "ok       .claude/skills/apexcode";
  if (!dryRun) await fs.cp(src, dest, { recursive: true });
  return "copied   .claude/skills/apexcode (skill + references)";
}

const stat = await fs.stat(target).catch(() => null);
if (!stat?.isDirectory()) {
  console.error(`Target directory not found: ${target}`);
  process.exit(1);
}

const selected = only ?? Object.keys(targets);
const unknown = selected.filter((name) => !targets[name]);
if (unknown.length) {
  console.error(
    `Unknown agent(s): ${unknown.join(", ")}. Choose from: ${Object.keys(targets).join(", ")}`,
  );
  process.exit(1);
}

console.log(`Installing ApexCode into ${target}${dryRun ? " (dry run)" : ""}\n`);
for (const name of selected) {
  for (const t of targets[name]) {
    console.log("  " + (t.kind === "file" ? await installFile(t) : await installBlock(t)));
  }
}
if (withSkill && (!only || only.includes("claude"))) console.log("  " + (await copySkill()));
console.log("\nDone. Agents pick the rules up on their next session.");
