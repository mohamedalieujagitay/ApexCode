<div align="center">

# ApexCode

Write like the developer whose repo you're in.

[[Docs]](https://mohamedalieujagitay.github.io/ApexCode/) · [[Skill]](skills/apexcode/SKILL.md) ·
[[Changelog]](CHANGELOG.md) · [[Credits]](NOTICE)

</div>

## Overview

ApexCode is an agent skill and toolchain that makes AI-assisted work read like your own. AI text has
a signature: em dashes, the same dozen inflated words, sentences that all run 12-18 words, comments
that narrate every line, commit messages that sound like press releases, and `Co-authored-by`
trailers. Detectors and experienced reviewers spot them in seconds.

ApexCode handles this in two layers.

- The skill changes how your agent writes. It reads your git history and neighbouring files first,
  then writes prose, code, comments, commits and PRs in your house style from the first draft.
- The toolchain checks the result: a Rust CLI, a terminal dashboard and pre-commit hooks score every
  file and fix what slipped through before it reaches history.

There's also an optional training pipeline that fine-tunes a small style-transfer model, for when
rules alone aren't enough.

## Key Features

- **Learns house style.** Before writing anything, it reads the last 20 commits and a few
  neighbouring files, then copies their casing, naming, comment density and commit format. The
  repo's own conventions override every built-in default.
- **12 core rules** covering prose, code and git, each shown with a real before and after example.
- **Natural, not manufactured.** Name length follows scope, complexity follows the problem, and
  nothing is faked: no forced symmetry, no inserted mistakes, no randomness.
- **No AI code patterns.** No hallucinated imports or APIs, duplicated blocks, hardcoded secrets,
  fake implementations, pattern-for-pattern's-sake, generic CRUD or textbook code that could belong
  to any project.
- **Production engineering discipline.** For real coding work the agent follows a nine-phase
  workflow (understand, inspect, design, implement, verify, break it, fix, review, report), checks
  security, data integrity and edge cases, tests failure paths, and never claims anything it hasn't
  verified.
- **Senior-engineer code.** Trusts the types, handles errors with context, uses domain names, writes
  each language idiomatically, and runs a six-point review before handing anything back.
- **Four modes:** write (on by default), rewrite (returns only the transformed text), audit
  (findings rated by severity) and retrofit (fixes existing code without changing behaviour).
- **Works in every major agent:** Claude Code, Cursor, Windsurf and Antigravity. The same rule set
  loads in each one.
- **Local-first detection:** a RoBERTa model runs on your machine, and cloud fallback is opt-in.
- **Commit-time safety net:** four pre-commit hooks check code, identifiers and commit messages.
- **Correctness first:** humanizing never changes behaviour, public APIs, strings or user-facing
  copy.

## Components

| Component        | Location                                         | Description                                                                      |
| ---------------- | ------------------------------------------------ | -------------------------------------------------------------------------------- |
| Skill            | `skills/apexcode/SKILL.md`                       | Full rule reference with write, rewrite and audit modes                          |
| Always-on rule   | `rules/`, `.claude/`, `.cursor/`, `.windsurf/`   | Condensed rule set every supported editor loads automatically                    |
| `/audit`         | `commands/audit.md`                              | Project-wide scan for AI fingerprints with severity ratings                      |
| `/retrofit`      | `commands/retrofit.md`, `references/retrofit.md` | Rewrites existing code to the skill in verified batches without breaking the app |
| CLI              | `crates/cli`                                     | `apexcode` binary: scan, fix, score, dashboard, config, install                  |
| Engines          | `crates/`                                        | Detector, humanizer, undercover rewriter, jitter buffer, TUI                     |
| Hooks            | `scripts/`, `.pre-commit-config.yaml`            | Stealth, comment, naming and commit-message checks                               |
| Engineering      | `skills/apexcode/references/`                    | Senior production engineer guide: workflow, security, testing, review            |
| AI code patterns | `skills/apexcode/references/ai-code-patterns.md` | 29 patterns that mark code as AI-generated, and what to write instead            |
| Human-style code | `skills/apexcode/references/human-style-code.md` | 20 principles for natural, scope-appropriate, domain-shaped code                 |
| Training         | `training/`                                      | Four-stage style-transfer pipeline (SFT, DPO, refinement)                        |

## The Skill

The skill is the heart of ApexCode. It's a single file your agent loads whenever it writes something
a person will read.

### Modes

| Mode     | Triggered by                                          | Output                                                                   |
| -------- | ----------------------------------------------------- | ------------------------------------------------------------------------ |
| Write    | Any prose, code, comment, commit, PR or doc (default) | Normal output following every rule                                       |
| Rewrite  | "humanize this", "clean this up", pasted text or code | Only the transformed text, no preamble                                   |
| Audit    | `/audit`, "scan this repo for AI patterns"            | Findings per file plus a category summary                                |
| Retrofit | `/retrofit`, "apply apexcode to my code"              | Existing code rewritten to the skill, behaviour unchanged, plus a report |

### Step 0: house style

Before writing into a repo, the agent runs `git log -n 20` and opens two or three neighbouring
files. It notes:

- commit length, casing, prefixes and tone
- camelCase or snake_case
- how dense the comments are, and whether they're docblocks or inline
- line length and indentation

It mirrors all of that. If the team uses conventional commits, so does the agent. With no history to
learn from, it falls back to the defaults below.

### The rules

| #   | Rule                       | AI pattern                                         | Human pattern                            |
| --- | -------------------------- | -------------------------------------------------- | ---------------------------------------- |
| 1   | No em dashes               | `Use satisfies — never use "as"`                   | `Use satisfies. Never use "as".`         |
| 2   | No flagged vocabulary      | delve, leverage, robust, seamless, ensure          | use, important, careful, complete        |
| 3   | No opening/closing cliches | "In today's fast-paced world...", "Happy coding!"  | Start with what it does. Stop when done. |
| 4   | Vary sentence length       | Four 15-word sentences in a row                    | Short lines mixed with longer ones       |
| 5   | No uniform formatting      | Every bullet `**Label:** text`, always three items | Uneven lists, labels only where needed   |
| 6   | No excessive markdown      | Bold on every other word                           | Plain text with sparse emphasis          |
| 7   | No hedging                 | "might potentially help in certain scenarios"      | "This improves performance."             |
| 8   | No narrating comments      | `// Return the first result`                       | Comments only on why                     |
| 9   | Natural identifiers        | `userAuthenticationToken`                          | `authToken`, in the repo's casing        |
| 10  | Senior-engineer code       | Errors swallowed into `null`, names like `data`    | Typed errors with context, domain names  |
| 11  | No AI co-author trailers   | `Co-authored-by: Cursor <...>`                     | Nothing                                  |
| 12  | Human commit messages      | `feat: implement comprehensive auth system...`     | `add JWT auth`                           |

### Voice and entropy

These settings shape tone in comments, commits and informal prose. They're turned down for formal
docs.

- Lowercase for short descriptions and sentence case for longer ones, unless the repo does
  otherwise.
- No trailing period on most single-line strings and commit subjects.
- Dev shorthand: auth, info, utils, config, param, impl, opt, sync, repo.
- Fragments and contractions are fine.
- The persona is a tired senior developer: concise, practical, doesn't explain the obvious.
- About 1 in 20 comment or commit lines uses looser shorthand. This never touches identifiers,
  strings, docs or anything that has to compile.

### Rewrite example

Before:

```rust
/**
 * This function implements a robust authentication mechanism
 * that ensures secure user access to the system.
 */
fn authenticate_user(user: User) -> Result<bool> {
```

After:

```rust
// temp auth fix - needs proper token refresh
fn auth_user(user: User) -> Result<bool> {
```

### Audit severity

| Severity | Checks                                                                                  |
| -------- | --------------------------------------------------------------------------------------- |
| High     | Em dashes, flagged vocabulary, AI co-author trailers, "Generated by", swallowed errors  |
| Medium   | Opening and closing cliches, narrating comments, over-formatting, emoji, AI code habits |
| Low      | Over-descriptive identifiers, low burstiness                                            |

Each finding gives the file and line, quotes the text, suggests a replacement and ends with a count
per category.

## Requirements

| Use                 | Needs                                                                     |
| ------------------- | ------------------------------------------------------------------------- |
| Skill and rule only | An agent that loads skills or rules (Claude Code, Cursor, Windsurf, etc.) |
| CLI and dashboard   | Rust stable (to build from source) or a release binary                    |
| Pre-commit hooks    | Python 3 and `pre-commit`                                                 |
| Training pipeline   | Python 3, a CUDA GPU (about 3 GB for inference), and the packages below   |

```bash
pip install -r training/requirements.txt
```

## Usage

### 1. Install the skill

```bash
npx skills add mohamedalieujagitay/ApexCode
```

Or copy `skills/apexcode/` into `.claude/skills/` or `.cursor/skills/`. The skill turns on whenever
your agent writes something. Type `/audit` to scan the current project.

#### Make it always on

ApexCode is built to be followed on every coding task, not only when someone asks. Three layers make
that happen:

1. **The skill** says so up front: it applies to any code change of any size, holds for the whole
   session, and runs a seven-point gate before any answer that contains code.
2. **Claude Code plugin hooks** (`hooks/`) inject the full rule set when a session starts, resumes,
   clears or compacts, and add a one-line reminder to every prompt so long sessions don't drift.
   Installing the plugin is enough to turn them on:

   ```bash
   /plugin marketplace add mohamedalieujagitay/ApexCode
   /plugin install apexcode
   ```

3. **Project rule files** for every major agent. Run this once in each project:

   ```bash
   git clone https://github.com/mohamedalieujagitay/ApexCode.git ~/apexcode
   node ~/apexcode/scripts/install-agent-rules.mjs /path/to/your/project
   ```

   | Agent              | File it writes                                              |
   | ------------------ | ----------------------------------------------------------- |
   | Claude Code        | `.claude/rules/apexcode.md`, `CLAUDE.md`, `.claude/skills/` |
   | Cursor             | `.cursor/rules/apexcode.mdc` (`alwaysApply: true`)          |
   | Windsurf           | `.windsurf/rules/apexcode.md` (`trigger: always_on`)        |
   | Antigravity        | `.agent/rules/apexcode.md`                                  |
   | Cline              | `.clinerules/apexcode.md`                                   |
   | GitHub Copilot     | `.github/copilot-instructions.md`                           |
   | Codex, Amp, others | `AGENTS.md`                                                 |
   | Gemini CLI         | `GEMINI.md`                                                 |

   Shared files like `AGENTS.md` get a marked ApexCode block appended, and your own content stays
   untouched. Re-running updates only that block, and files you wrote yourself are never
   overwritten. Use `--only=cursor,claude` to pick agents, `--no-skill` to skip copying the skill,
   or `--dry-run` to preview.

### Retrofit existing code

Already have a codebase written without ApexCode? Run:

```text
/retrofit src/
```

The agent brings that code up to the skill without changing what the app does. It:

1. Starts from a clean git tree on a new `apexcode/retrofit` branch.
2. Records a baseline: build, type check, lint, tests, and the public surface (exports, routes, CLI
   flags, env vars, schemas).
3. Audits the code and sorts every finding into a risk tier. Comments and docs are fixed freely.
   Local, provable cleanups like unused imports and debug logs go in verified batches.
   Behaviour-adjacent fixes like swallowed errors or duplicated logic only happen behind
   characterization tests that pin today's behaviour. Contract changes like public names, routes and
   schemas are never made, only recommended.
4. Applies changes in small batches. It runs the checks after each batch and commits it with a
   human-style message, and reverts automatically if anything regresses. It never edits a test to
   make a batch pass.
5. Re-runs the full baseline, diffs the public surface, and reports what changed, what was skipped
   and why, and what still needs your call.

Large codebases are handled module by module, with progress saved to
`.apexcode/retrofit-progress.md` so a later session can pick up where the last one stopped. Roll
back everything by deleting the branch, or one change with `git revert`.

### 2. Install the CLI

```bash
curl -sSL https://mohamedalieujagitay.github.io/ApexCode/install.sh | bash
```

Or from crates.io or source:

```bash
cargo install apexcode
```

```bash
git clone https://github.com/mohamedalieujagitay/ApexCode.git
cd ApexCode
cargo build --release
cargo install --path crates/cli
```

Wire it into your editor:

```bash
apexcode install --ide claude      # or cursor, windsurf, antigravity, all
```

### 3. Scan and fix

```bash
apexcode              # dashboard (default)
apexcode scan         # detect AI patterns in staged files
apexcode fix          # auto-fix AI patterns (--dry-run to preview)
apexcode score        # stealth score (--detailed for the breakdown)
apexcode dashboard    # terminal UI with per-file heatmap
apexcode config       # view or reset settings
apexcode init         # set up .apexcode/ in this repo
```

Typical loop:

```bash
git add your-files
apexcode scan
apexcode fix
git commit -m "your message"
```

Settings live in `.apexcode/config.toml`:

```toml
[detection]
threshold = 0.15              # AI probability threshold (0.0 - 1.0)
use_local = true              # local models first
use_cloud_fallback = true     # cloud API when the local score is uncertain

[humanization]
auto_humanize = false         # humanize automatically on commit
entropy_level = 0.5           # transformation intensity (0.0 - 1.0)

[jitter]
enabled = false               # spread staged commits over a time window
min_delay_secs = 60
max_delay_secs = 300

[api]
openrouter_key = ""           # optional
anthropic_key = ""            # optional
base_url = ""                 # optional custom API base URL
```

The same settings can come from the environment. See `.env.example`.

### 4. Turn on pre-commit hooks

```bash
pip install pre-commit
pre-commit install
pre-commit install --hook-type commit-msg
```

| Hook                         | Script                            | Stage      |
| ---------------------------- | --------------------------------- | ---------- |
| `apexcode-stealth-check`     | `scripts/stealth_check.py`        | pre-commit |
| `apexcode-humanize-comments` | `scripts/auto_humanize.py`        | pre-commit |
| `apexcode-naming-check`      | `scripts/naming_check.py`         | pre-commit |
| `apexcode-commit-entropy`    | `scripts/check_commit_entropy.py` | commit-msg |

### 5. Train your own model (optional)

`training/` trains a small BART paraphraser (about 0.1B parameters) that moves AI-written text
toward human style. It needs about 3 GB of GPU memory and takes roughly 1.7 s per sample at
inference.

| Stage | Script                        | Description                                                       |
| ----- | ----------------------------- | ----------------------------------------------------------------- |
| 1     | `stage1_data_construction.py` | Keep only text a detector confidently scores as human             |
| 2     | `stage2_style_sft.py`         | Style-injection SFT on BART (reconstruction + transfer objective) |
| 2→3   | `stage2_inference.py`         | Run the SFT model and collect hard negatives for DPO              |
| 3     | `stage3_dpo_alignment.py`     | DPO fine-tuning against the detector's decision boundary          |
| 4     | `stage4_refinement.py`        | Optional LLM-guided sentence refinement with perplexity ranking   |

**Stage 1: data construction.** Stream a HuggingFace dataset and keep only samples the detector
classifies as human:

```bash
python training/stage1_data_construction.py \
    --dataset dmitva/human_ai_generated_text \
    --data-file model_training_dataset.csv \
    --num-rows 50000 \
    --model-path /path/to/roberta-detector \
    --output filtered_human_text.jsonl
```

**Stage 2: style-injection SFT.** `train_pairs.jsonl` holds `src` (AI text) and `trg` (human text)
fields:

```bash
python training/stage2_style_sft.py \
    --bart-path facebook/bart-base \
    --data-path train_pairs.jsonl \
    --output-dir checkpoints/style_sft \
    --epochs 50 \
    --batch-size 8 \
    --lr 2e-5
```

**Stage 2→3: inference and hard negatives.** Find outputs that still read as AI:

```bash
python training/stage2_inference.py \
    --model-path checkpoints/style_sft/bart \
    --detector-path /path/to/roberta-detector \
    --input-file test_data.jsonl \
    --output-file inference_results.jsonl \
    --batch-size 8
```

**Stage 3: DPO alignment.** Build preference pairs from the hard negatives and fine-tune:

```bash
python training/stage3_dpo_alignment.py \
    --model_name_or_path checkpoints/style_sft/bart \
    --file_a_path inference_results.jsonl \
    --file_b_path train_pairs.jsonl \
    --dpo_output_path dpo_data.jsonl \
    --output_dir checkpoints/dpo
```

**Stage 4: refinement (optional).** Polish each sentence with an LLM, keeping a change only when the
detector still predicts "human":

```bash
python training/stage4_refinement.py \
    --input dpo_output.jsonl \
    --output refined_output.jsonl \
    --detector-model /path/to/roberta-detector \
    --llm-path /path/to/llm \
    --ppl-model /path/to/ppl-model
```

## Project Layout

```
apexcode/
├── skills/apexcode/
│   ├── SKILL.md                   # the skill
│   └── references/                # senior-engineer, ai-code-patterns, human-style-code, retrofit
├── rules/apexcode.mdc             # always-on rule, single source for every editor copy
├── commands/                      # /audit, /retrofit
├── hooks/                         # Claude Code session and prompt hooks
├── .claude/ .cursor/ .windsurf/   # editor rule copies, generated by npm run sync-rules
├── .claude-plugin/ .cursor-plugin/   # plugin and marketplace manifests
├── crates/
│   ├── core/                      # shared types and traits
│   ├── detector/                  # AI detection engine
│   ├── humanizer/                 # style learning and code transformation
│   ├── jitter/                    # commit timing buffer
│   ├── undercover/                # commit, comment and naming rewriter
│   ├── cli/                       # the apexcode binary
│   └── tui/                       # terminal dashboard
├── scripts/                       # pre-commit hooks, installers, validator
├── training/                      # style-transfer training pipeline + requirements.txt
├── docs/                          # GitHub Pages site
├── examples/                      # sample input for the detector
├── graphify-out/                  # generated codebase graph for the docs site
└── .cargo/audit.toml              # cargo-audit advisory ignores
```

## Development

```bash
cargo test                       # all crates
cargo test -p apexcode-core      # one crate: -core, -detector, -humanizer, -jitter, -undercover, -tui
cargo build --release
npm run validate                 # skill, rules, commands, and editor copies in sync
npm run sync-rules               # regenerate .claude/.cursor/.windsurf rules after editing rules/apexcode.mdc
npx prettier --check .           # formatting, as CI runs it
```

Edit the always-on rule only in `rules/apexcode.mdc`, then run `npm run sync-rules`. CI fails if the
editor copies drift.

Publishing steps are in `crates/cargo-instruction.md`. Pushing a `v*` tag runs the release workflow,
which builds binaries for Linux, macOS and Windows.

## Security and Privacy

Detection runs locally by default. Cloud APIs are opt-in and can be switched off entirely. Nothing
is collected or sent anywhere. To report a vulnerability, open a private security advisory on
GitHub.

## Disclaimer

ApexCode is provided as is, for education, research and helping developers keep their own voice when
they use AI tools. It is not meant for bypassing academic integrity systems, cheating in evaluations
or any other misuse. The author and contributors aren't liable for how it's used.

## Citation

```bibtex
@software{jagitay2026apexcode,
  title  = {ApexCode: Write Like the Developer Whose Repo You're In},
  author = {Jagitay, Mohamed Alieu},
  year   = {2026},
  url    = {https://github.com/mohamedalieujagitay/ApexCode}
}
```

The training pipeline implements the MASH method. If you use `training/` in research, please cite it
too:

```bibtex
@article{gu2026mash,
  title={MASH: Evading Black-Box AI-Generated Text Detectors via Style Humanization},
  author={Gu, Yongtong and Li, Songze and Hu, Xia},
  journal={arXiv preprint arXiv:2601.08564},
  year={2026}
}
```

## License

Apache-2.0 and MIT. See [LICENSE](LICENSE). ApexCode builds on open-source work by Ofer Shapira,
John Varghese, and Yongtong Gu, Songze Li and Xia Hu. [NOTICE](NOTICE) lists what came from where.
The `training/` pipeline is under its authors' research-only terms.

## Author

Mohamed Alieu Jagitay · [GitHub](https://github.com/mohamedalieujagitay)
