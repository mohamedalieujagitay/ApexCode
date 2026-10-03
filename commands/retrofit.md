---
name: retrofit
description:
  Rewrite existing code to the apexcode skill in verified batches without changing app behaviour
---

Retrofit the code at the path given after the command, or ask for a scope if none was given. Apply
every rule in the apexcode skill to the existing code, and rewrite or remove whatever violates those
rules. The app's observable behaviour must not change.

Follow Retrofit mode (Part 6b) in the apexcode skill and the full protocol in
`skills/apexcode/references/retrofit.md`. Every phase is mandatory:

1. **Scope and safety net:** confirm the paths, require a clean git tree, and work on a new
   `apexcode/retrofit` branch.
2. **Baseline:** record build, type check, lint and test results, and snapshot the public surface.
3. **Map:** run the `/audit` checks and give each finding a risk tier (0 cosmetic, 1 local, 2
   behaviour-adjacent, 3 contract-changing).
4. **Characterization tests:** pin current behaviour before any Tier 2 change.
5. **Apply in small batches:** check after each batch and commit it if everything passes. Revert the
   batch and record why if anything regresses.
6. **Full verification:** compare everything with the baseline, diff the public surface and re-run
   the audit.
7. **Report:** what changed, before and after test results, what was skipped and why, Tier 3
   recommendations, and what wasn't verified.

Never change Tier 3 items (public names, routes, schemas, env vars, messages clients parse) without
explicit approval for that specific change. Never weaken or delete a test to get a batch through.
Never touch generated, vendored or migration code.
