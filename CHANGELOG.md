# Changelog

All notable changes to ApexCode are recorded here.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and the project uses
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Retrofit mode and `/retrofit`: bring existing code up to the skill without changing behaviour
- Always-on activation: Claude Code session and prompt hooks, plus `install-agent-rules.mjs` for
  Claude Code, Cursor, Windsurf, Antigravity, Cline, Copilot, AGENTS.md and Gemini
- Reference guides: senior engineer, AI code patterns, human-style code
- `npm run sync-rules`, with `npm run validate` failing when editor rule copies drift
- `training/requirements.txt`

### Changed

- Training pipeline moved to `training/`
- Always-on rule renamed to `rules/apexcode.mdc` and made the single source for every editor copy
- cargo-audit config moved to `.cargo/audit.toml`, where `cargo audit` actually reads it
- `.gitignore` deduplicated and regrouped

### Removed

- Duplicate GitHub Pages workflow (`deploy-pages.yml`); `pages.yml` covers it
- `skills/unified.md`, already merged into the skill
- Empty placeholder files under `.claude/` and `extras/`

### Fixed

- Windsurf rule used `trigger: always`; Windsurf expects `always_on`
- CI formatting check failed on 24 files

## [1.0.0] - 2026-10-03

### Added

- `apexcode` skill with write, rewrite and audit modes covering prose, code, git and voice
- House-style step: agents read recent commits and neighbouring files before writing
- Single always-on rule shipped for Cursor, Claude Code and Windsurf
- `/audit` command that folds CLI and hook output into one report, including identifier checks
- Training pipeline in `training/` for detector-guided style transfer
- NOTICE file with upstream attribution

### Changed

- One name across every crate, binary, config path, hook and env var: `apexcode` (`apexcode-core`,
  `apexcode-detector`, ... and `.apexcode/config.toml`)
- Engine types renamed to `ApexDetector`, `ApexHumanizer`, `ApexJitter` and `ApexTui`
- Running `apexcode` with no subcommand opens the dashboard, as documented
- CLI version now comes from `Cargo.toml`
- Controlled shorthand is limited to comments, commits and informal prose, never identifiers,
  strings or docs
- `npm run validate` ignores loose files in `skills/`

## [0.1.0] - 2026-04-29

### Added

- AI detection engine with local model support and cloud fallback
- Code humanization with style learning from repo history
- Commit timing buffer (jitter engine)
- Terminal dashboard
- Git hooks for automatic scanning
- Multi-language support: Rust, Python, JavaScript, TypeScript, Go, C++, C
- TOML configuration
- IDE rules for Cursor, Claude, Windsurf and Antigravity
- Writing rules for prose, comments, commits and READMEs
- Installation scripts, CI/CD pipeline and documentation

[Unreleased]: https://github.com/mohamedalieujagitay/ApexCode/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/mohamedalieujagitay/ApexCode/releases/tag/v1.0.0
[0.1.0]: https://github.com/mohamedalieujagitay/ApexCode/releases/tag/v0.1.0
