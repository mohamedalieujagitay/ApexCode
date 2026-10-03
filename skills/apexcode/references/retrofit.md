# Retrofit: Bring Existing Code Up to ApexCode

Part of the ApexCode skill. Load this file when a user asks you to apply ApexCode to code that
already exists: "retrofit", "clean up this codebase", "apply apexcode to src/", "make this code
match the skill", `/retrofit`. Also read the three other guides in this folder. Retrofit uses all of
them to decide what to change.

Retrofit is the explicit request that Standing instruction 2 in SKILL.md is waiting for. It is the
only mode where you rewrite and remove existing code. You do it under one hard constraint:

**The app's observable behaviour must be identical before and after.** Same outputs for the same
inputs, same public API, same data on disk and over the wire, same side effects. If you can't show
that a change preserves behaviour, you don't make it. You report it instead.

---

## Phase 0: Scope and safety net

1. **Confirm scope.** Which paths? The whole repo, one package, one folder, a list of files? If the
   user didn't say, propose a scope (start with the module they're working in) and ask once.
2. **Version control is mandatory.** Check `git status`.
   - Clean tree: create a branch, e.g. `git switch -c apexcode/retrofit`.
   - Uncommitted changes: stop and ask the user to commit or stash. Never mix your edits with
     theirs.
   - No git: ask the user to initialise it, or get explicit approval to work without rollback.
3. **Never retrofit these, whatever they contain:** generated code, vendored or third-party code,
   lockfiles, database migrations that have already run, build output, minified files, and anything
   in `.gitignore`.

## Phase 1: Baseline

Before touching anything, record what currently works.

1. Detect the toolchain from the manifests (`package.json`, `Cargo.toml`, `pyproject.toml`,
   `go.mod`, `pom.xml`, `Makefile`, CI config). Use only commands the project actually has.
2. Run and record the results of build, type check, lint, the full test suite and, if present, the
   end-to-end tests.
3. Save the results as the baseline: pass and fail counts, failing test names and warnings.
4. **Pre-existing failures aren't yours.** Don't fix them silently, and don't count them as
   regressions. List them in the report. If the build itself fails, stop and report. You can't
   verify behaviour on a project that doesn't build.
5. Snapshot the public surface: exported symbols, HTTP routes, CLI flags, env var names, config
   keys, DB schema and serialized field names. Phase 4 compares against this.

## Phase 2: Map findings

Run Audit mode (SKILL.md Part 6) over the scope, plus the patterns in `ai-code-patterns.md`,
`human-style-code.md` and `senior-engineer.md`. Give each finding exactly one risk tier.

| Tier | Risk                     | Examples                                                                                                                                                                                                                     | What to do                                                                |
| ---- | ------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| 0    | None (no runtime effect) | Narrating or boilerplate comments, AI-style docblocks on private helpers, em dashes and flagged words in comments and docs, AI attribution lines, emoji in comments                                                          | Fix freely                                                                |
| 1    | Local, provable          | Unused imports and locals, unreachable branches, leftover `console.log`/`print`/`debugger`, needless intermediate variables, `else` after `return`, `if (x) return true else return false`, local renames in a function body | Fix in small batches, verify each batch                                   |
| 2    | Behaviour-adjacent       | Swallowed errors, try/catch that only logs, missing validation, hardcoded config, duplicated logic, unused private functions, over-abstraction, generic error messages                                                       | Fix only with a test that pins current behaviour, and keep that behaviour |
| 3    | Contract-changing        | Exported or public names, routes, CLI flags, env vars, config keys, DB columns, serialized fields, error codes or messages clients parse, log formats other tools read, file and module paths                                | Don't change. Recommend in the report with a migration plan               |

When unsure between two tiers, pick the higher one.

### Things that look removable but aren't

Before deleting or renaming anything, rule out every one of these:

- Referenced by string or reflection: `getattr`, `globals()`, `require(variable)`, dynamic
  `import()`, DI containers keyed by name, ORM model names, serializers and decorators that register
  by name.
- Referenced outside the code: templates, config files, routes files, build scripts, CI, Docker,
  docs examples, other repos and services.
- Entry points and framework hooks: `main`, `__init__`, exported handlers, lifecycle methods, test
  fixtures and anything a framework calls by convention.
- Side effects at import time: module-level code that registers, patches or configures something.
- Public API: anything exported from a package entry point is in use by someone, even if this repo
  doesn't call it.
- Intentional quirks: odd-looking code with a comment, a linked issue or a test that pins it. Leave
  it, and note it if it looks wrong.

Search the whole repo, not just the scope, including non-code files, for every name you plan to
remove or rename. If the search can't rule out dynamic use, the change moves up to Tier 3.

### Preserve what ApexCode says to preserve

- Genuine TODO, FIXME, HACK and XXX notes stay.
- Short names in small scopes stay, and so do the repo's own conventions. Don't normalise them.
- Comments that explain why stay, even if their wording isn't how you'd write it.
- Behaviour quirks that callers might depend on stay. Flag them in the report.

## Phase 3: Characterization tests

Any Tier 2 change needs a test that captures today's behaviour before you touch the code.

1. If existing tests already cover the path, note which ones.
2. If not, write characterization tests in the project's test framework and style. They assert what
   the code does now, including edge cases and odd outputs, not what it should do.
3. Run them against the untouched code. They must pass before you refactor.
4. Keep them in the final diff. They're part of the value you deliver.

If a path can't be tested reasonably (it needs real infrastructure, for example), downgrade the
change to a recommendation.

### Behaviour-preserving recipes for Tier 2

| Finding                                   | Safe transformation                                                                                                                                                                |
| ----------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `catch (e) {}` swallows an error          | Keep the same return value and control flow, but record the error with context through the project's logger. Changing it to rethrow is a behaviour change, so recommend it instead |
| `catch (e) { console.error(e); throw e }` | Remove the wrapper only if the caller already logs. Otherwise replace `console.error` with the project logger plus context                                                         |
| Generic error message                     | Make it specific only if no test, client or log parser matches the old text. Otherwise recommend                                                                                   |
| Hardcoded URL, limit or fee               | Move it to the project's config or constants with the same default value. Behaviour is unchanged unless someone sets the new key                                                   |
| Hardcoded secret                          | Don't move or rotate it silently. Report it as a security finding. The user must rotate it and decide where it lives                                                               |
| Duplicated logic                          | Extract only when the copies are identical in behaviour, including edge cases. Point every copy at the shared one and run each copy's tests                                        |
| Missing input validation                  | Adding rejection changes behaviour. Recommend it, or add it only if the user approves                                                                                              |
| Needless wrapper or abstraction layer     | Inline it only if it's private and every call site is in scope. Otherwise recommend                                                                                                |
| Unused private function                   | Remove it only after the full search in "Things that look removable but aren't"                                                                                                    |

## Phase 4: Apply in small, verified batches

1. **Batch size.** One tier and one category in one module at a time, e.g. "Tier 1, unused imports,
   `src/billing/`". Aim for diffs a reviewer can read in a couple of minutes.
2. **Order.** Tier 0 everywhere first, then Tier 1, then Tier 2. Within a tier, go module by module.
3. **Renames.** Use the language's rename tooling if you have it. Otherwise update every reference
   in the same batch, and search afterwards for the old name in code, strings, templates and config.
4. **After every batch,** run the fast checks: type check, lint and the tests for the touched
   modules.
   - All pass: commit the batch with a human-style message (`drop unused imports in billing`,
     `log swallowed errors in sync worker`). No AI trailers.
   - Anything fails that passed in the baseline: revert the batch, retry the smaller safe part if
     there is one, otherwise skip the finding and record why. Never edit or delete a test to make a
     batch pass. A failing test means the change altered behaviour.
5. **Formatting.** Only run the project's formatter on lines you changed, or on whole files if the
   project already formats whole files in CI. Never reformat untouched code. It buries the real
   diff.
6. **Large codebases.** Keep a progress file, `.apexcode/retrofit-progress.md`, listing modules
   done, in progress and pending, plus skipped findings. If the session ends, the next one resumes
   from it. Prioritise high-severity findings and the modules the team edits most.

## Phase 5: Full verification

When all batches are done:

1. Run everything from the baseline again: build, type check, lint, full tests and end-to-end.
2. Compare with the baseline. Every check that passed before must still pass, the test count must be
   the same or higher, and no new warnings may appear.
3. Compare the public surface with the Phase 1 snapshot. Any difference is a bug in the retrofit, so
   revert the batch that caused it.
4. Read the full diff as a reviewer (`git diff <base>...HEAD`). Confirm every hunk maps to a
   finding, and that nothing outside scope changed.
5. Re-run Audit mode on the scope to confirm the findings are gone. Remaining ones should all be
   accounted for as skipped or recommended.

## Phase 6: Report

Write it in ApexCode's voice: plain, specific, nothing claimed that wasn't checked.

```text
Retrofit of src/billing and src/sync on branch apexcode/retrofit (14 commits)

Changed:
- 63 narrating comments and 9 boilerplate docblocks removed
- 41 unused imports, 6 dead branches and 12 debug logs removed
- 4 swallowed errors now logged with context (same return values)
- 3 hardcoded limits moved to config/limits.ts (same defaults)

Tests:
- Added 11 characterization tests for sync retry and invoice rounding
- Baseline: 212 passed, 2 failed (pre-existing: test_tz_dst, test_legacy_export)
- After: 223 passed, same 2 failed
- Build, tsc and eslint clean. Public exports unchanged

Skipped (would change behaviour or couldn't be proven safe):
- invoice.ts:88 rethrowing instead of returning null: callers rely on null
- utils/registry.ts unused-looking handlers: registered by name in plugins.json

Recommended (needs your call):
- Rename exported `processData` to `reconcileLedger` (public API, needs a deprecation alias)
- Hardcoded Stripe key in payments/client.ts:12: rotate it and move it to env

Not verified:
- End-to-end tests (none in repo)
- Behaviour against production data
```

To roll back everything: `git switch <base> && git branch -D apexcode/retrofit`. To roll back one
change: `git revert <commit>`.

---

## Hard rules

- Behaviour stays identical. A retrofit that breaks the app has failed, however clean the code
  looks.
- Never weaken, skip, delete or rewrite an existing test to get a batch through.
- Never change Tier 3 items without explicit approval for that specific change. When you have
  approval, keep a compatibility alias or shim.
- Never claim the retrofit is safe beyond what Phase 5 actually verified.
- Stay in scope. Notice something outside it? Put it in the report.
