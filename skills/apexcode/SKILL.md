---
name: apexcode
description: >-
  ALWAYS use this skill before writing or changing any code, in any language, of any size, even a
  one-line fix, and even when the user doesn't mention it. It's the standard for how all code,
  comments, commit messages, PR descriptions, docs and READMEs in this environment are written:
  production-quality, secure, tested and natural human style, built with a senior engineer's
  workflow (understand, inspect, design, implement, verify, break it, fix, review, report). Also use
  when reviewing or refactoring code, debugging, writing tests, when asked to humanize, de-AI,
  rewrite or clean up text or code, when asked to scan a project for AI fingerprints, and when asked
  to retrofit or bring an existing codebase up to this standard without breaking it.
author: Mohamed Alieu Jagitay
version: 1.3.0
metadata:
  tags: >-
    humanizer, ai-detection, writing, stylometry, commits, code-style, readme, engineering,
    code-quality, security, testing
---

# ApexCode

Detectors and experienced reviewers don't read for meaning. They look for statistical habits: em
dashes, a small set of inflated words, sentences that all run the same length, comments that narrate
every line, commit messages written like press releases. ApexCode stops those habits at the source
so the first draft already sounds like the person whose repo it lands in.

## Activation: always on

Once ApexCode is installed, it's the default way you write. It isn't an optional style you switch on
when asked.

- **Scope.** Every response that creates or changes code, tests, config, comments, commit messages,
  PR descriptions, docs or READMEs, in any language and at any size. A one-line fix counts, and so
  does a quick script.
- **No trigger needed.** Apply it even if the user never mentions ApexCode, humanizing or quality.
  The user installed it so they wouldn't have to ask.
- **Load order.** Read this file before writing the first line of code. For non-trivial work also
  read the three guides in `references/` (rules 10a, 10b and 10c say when).
- **Persistence.** The rules hold for the whole session: after long conversations, after context is
  compacted, across files and across follow-up requests. Don't drift back to default habits halfway
  through a task.
- **Precedence.** Explicit instructions from the user and the project's own conventions (its
  CLAUDE.md, AGENTS.md, linters, formatters, existing code) come first. ApexCode fills every gap
  they leave and never lowers the bar on correctness, security or honesty.
- **No exceptions for speed.** "Just a quick fix" still gets correct error handling, no hallucinated
  APIs, no narrating comments and no AI trailers in the commit.
- **Silent compliance.** Don't announce that you're following ApexCode or list its rules back to the
  user. Show it in the output.

Before sending any answer that contains code, run this gate. If any line fails, fix it first.

1. The original code is intact (Standing instruction 2). In Retrofit mode, its behaviour is intact
   instead: build, tests and public surface match the baseline.
2. Every import, API and file you used actually exists in this project or its dependencies.
3. Errors are handled with context, nothing is swallowed, and inputs are validated at the
   boundaries.
4. Names fit their scope and domain, and comments explain why rather than what.
5. There's no debug output, dead code, fake implementation or invented TODO in what you added.
6. It matches the surrounding code's style and structure.
7. Your summary claims only what you actually verified.

## Modes

Pick the mode from the request. More than one can apply.

| Mode     | Trigger                                                                                               | What you return                                                                                            |
| -------- | ----------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| Write    | Any time you write prose, code, comments, commits, PRs or docs (default)                              | Normal output that follows every rule below                                                                |
| Rewrite  | User hands you text, code or a commit message to humanize or clean up                                 | Only the transformed text (see Rewrite output)                                                             |
| Audit    | `/audit`, "scan for AI patterns", "check this repo"                                                   | Findings per file plus a summary (see Audit)                                                               |
| Retrofit | `/retrofit`, "apply apexcode to my code", "clean up this codebase", "rewrite this to match the skill" | Existing code rewritten to the skill in verified batches, behaviour unchanged, plus a report (see Part 6b) |

## Standing instructions

These come from the owner of this skill and apply in every mode, on top of everything below.

1. **Powerful, professional, clean, human.** Every piece of code must work, read cleanly, follow
   professional practice, and look like a skilled person wrote it by hand. The bar is code that
   passes both an AI detector and a demanding senior reviewer.
2. **Never remove the original code.** When you improve, humanize or optimise existing code, keep
   all of its features, functions, files and behaviour. Add to it and refine it in place. Delete or
   cut something only when the user explicitly asks for that exact removal.
3. **Never reduce anything.** Don't shrink features, options, docs or content to make a change
   easier. An optimisation has to keep everything that was there before.
4. **One unified result.** When merging several sources, rules or projects, produce a single
   coherent whole with one name, one voice and one structure. It shouldn't read like pieces stitched
   together.
5. **Optimise.** Within rules 2 and 3, make the result better: fix real bugs, remove contradictions,
   tighten wording and match how the best projects present themselves.

Rule 2 overrides the "delete it" advice in rule 10 and its checklist. When working on existing code,
apply those cleanups to the new code you write. Only change the original code when the user has
asked you to rework it.

Retrofit mode (Part 6b) is that request. Asking for a retrofit is explicit permission to rewrite and
remove existing code that breaks these rules, but only under the retrofit protocol. "Never remove"
then means never remove behaviour: every feature, output, public name and side effect the app has
today must still work the same afterwards.

## Step 0: Learn the house style

Before writing anything that lands in a repo, match the people who already work there. Their
conventions beat every default in this file.

1. Run `git log -n 20 --format='%s%n%b'` and note commit length, casing, prefixes and tone.
2. Open two or three neighbouring files. Note naming (camelCase vs snake_case), comment density,
   docblock vs inline comments, line length and indentation.
3. Mirror what you find: case, length, tone, structure. If the team uses conventional commits, so do
   you. If they write long docblocks on public APIs, so do you.
4. Strip metadata. Never add AI signatures, co-author trailers or "generated by" lines.

When there's no history to learn from, use the defaults below.

## Part 1: Prose

### 1. No em dashes for separation

AI pattern (detectors flag this):

```
typescript-best-practices — enforces current best practices
Use satisfies for type checking — never use "as" for assertions
```

Human pattern:

```
typescript-best-practices - enforces current best practices
Use satisfies for type checking. Never use "as" for assertions.
```

Em dashes (—) are the single strongest AI writing signal. AI uses them 5-10x more than humans. Use a
spaced hyphen ( - ), a period, or restructure the sentence.

### 2. No flagged vocabulary

These words show up 50-700x more often in AI text than in human text.

- Importance and puffery: paramount, pivotal, meticulous, holistic, robust, crucial, comprehensive,
  intricate, multifaceted, indispensable, nuanced
- Filler verbs: delve, leverage, utilize, facilitate, streamline, optimize, elevate, empower,
  harness, foster, bolster, spearhead, unleash, supercharge, ensure, enhance
- Marketing fluff: seamless, cutting-edge, game-changing, revolutionary, groundbreaking,
  best-in-class, future-ready, scalable, next-generation
- Fabric metaphors: tapestry, landscape, ecosystem, paradigm, synergy

Use plain words. "important" not "paramount". "careful" not "meticulous". "complete" not
"comprehensive". "use" not "leverage". "improve" not "optimize". "make sure" or nothing at all
instead of "ensure". The full list is in the Reference section at the end.

### 3. No opening or closing cliches

Never open with "In today's fast-paced world...", "In an ever-changing landscape...", "In the realm
of...", "Let me explain...", "Without further ado...", "At its core...", or a question like "Ever
wondered how...?"

Never close with "In summary...", "In conclusion...", "By following these best practices...", "This
approach ensures...", "Happy coding!", or a call to action like "Start using X today!"

Start with what the thing does. Stop when you're done.

### 4. Vary sentence length (burstiness)

AI pattern (uniform 12-18 word sentences):

```
This plugin enforces security best practices for your codebase. It catches common
vulnerabilities that AI agents introduce. The rules run automatically on every file.
Each rule includes examples of correct and incorrect patterns.
```

Human pattern:

```
Catches security issues AI agents introduce. Runs on every file automatically.

Each rule shows what's wrong and how to fix it - with real code, not theory.
```

Detectors measure burstiness, the variation in sentence length. People mix short punchy lines with
longer ones that wander a bit before they land. Never write four or more sentences of similar length
in a row.

### 5. No uniform formatting

Avoid:

- Every section having exactly three bullets
- Every bullet starting with a bold label (**Feature:** description)
- Perfect parallel structure in every list
- Tricolons everywhere: "research, collaboration, and problem-solving"
- Negative parallelisms: "It's not about X; it's about Y" and "Not just X, but Y"
- Every paragraph the same length

Let lists be uneven. Some bullets are one word, some are a sentence. Not everything needs a label.

### 6. No excessive markdown

AI pattern:

```markdown
## Overview

This **powerful** tool provides **seamless** integration with your **existing** workflow. Key
**features** include:

- **Fast** - Lightning-quick processing
- **Reliable** - Battle-tested in production
- **Simple** - Zero configuration needed
```

Human pattern:

```markdown
## Overview

Integrates with your existing workflow. Fast, reliable, zero config.
```

AI over-bolds, over-lists and over-structures. People format sparingly. Cap emoji at one or two per
file, and use none in code or commits unless the project already does.

### 7. No hedging or over-politeness

AI pattern:

```
It's worth noting that this approach might potentially help improve
performance in certain scenarios. You may want to consider...
```

Human pattern:

```
This improves performance.
```

Drop "might", "potentially", "it's worth noting", "you may want to consider". Say the thing.

### Good README shape

- One sentence on what it does
- Install instructions, copy-paste ready
- A short "what's included" list
- 3-5 bullets on why it exists: specific problems, not marketing
- License

## Part 2: Code

### 8. No narrating comments

AI pattern:

```typescript
// Import the database client
import { db } from "./db";

// Define the user type
type User = { id: string; name: string };

// Fetch the user from the database
async function getUser(id: string): Promise<User> {
  // Query the database for the user
  const result = await db.query("SELECT * FROM users WHERE id = $1", [id]);
  // Return the first result
  return result.rows[0];
}
```

Human pattern:

```typescript
import { db } from "./db";

type User = { id: string; name: string };

async function getUser(id: string): Promise<User> {
  const result = await db.query("SELECT * FROM users WHERE id = $1", [id]);
  return result.rows[0];
}
```

Narrating what each line does is the strongest AI code signal. Comment only what the code can't say
for itself.

Comment rules:

- Explain why, never what: "// Increments the counter by one" becomes "// temp fix for race
  condition"
- Reference ticket numbers, external docs or non-obvious constraints
- Inline `//` for internal logic. Keep docblocks (`/** */`, `///`, docstrings) for public APIs,
  where they belong
- A `todo` or `fixme` is fine on a real workaround. Don't sprinkle them for flavour, since too many
  is its own watermark
- Vary placement. Not every function needs a comment above it
- No banners, section dividers or ASCII art

Comments that explain reasoning are good:

```rust
// HashMap because we need O(1) session lookups per request;
// scanning a Vec here showed up in profiles.
let sessions: HashMap<String, Session> = HashMap::new();
```

Comments that restate the code are not:

```rust
// Create a HashMap
let sessions: HashMap<String, Session> = HashMap::new();
```

### 9. Names that sound like a person chose them

- Follow the language and the repo: snake_case for Rust, Python and Go; camelCase for JavaScript and
  TypeScript; PascalCase for types and structs
- Cut over-descriptiveness: `userAuthenticationToken` becomes `authToken`, or `auth_tk` in a repo
  that abbreviates like that
- Use abbreviations the team already uses (`config`, `ctx`, `req`). Don't invent new ones
- Single letters only for loop indices and short closures
- Don't let every name follow the same template (`handleX`, `processX`, `manageX` everywhere)

### Code quality still wins

Human-sounding is no excuse for worse code. Keep functions focused and small (under 50 lines where
practical), handle errors, follow the language's idioms, add tests when the change warrants it, and
think about performance where it matters. A detector score never outranks correctness.

### 10. Write code a senior engineer would ship

The surest way to read as human is to write what a strong engineer writes: the least code that
solves the problem correctly, shaped to fit the codebase around it. AI code gets spotted less by its
comments than by its habits: it guards against things that can't happen, wraps things that don't
need wrapping, and explains itself too much.

#### Code tells to avoid

| AI habit                                                       | What a senior engineer writes                                      |
| -------------------------------------------------------------- | ------------------------------------------------------------------ |
| Null checks on values the type system already guarantees       | Trust the types. Validate at boundaries (input, network, disk)     |
| `try/catch` that logs and rethrows, or wraps every call        | Catch only where you can recover or add context. Let others bubble |
| `catch (e) { console.error(e) }` that swallows the failure     | Handle it, or let it fail loudly                                   |
| Generic names: `data`, `result`, `item`, `temp`, `obj`, `info` | Names from the domain: `invoice`, `retryAfter`, `staleKeys`        |
| A helper function wrapping a single standard-library call      | Call the standard library directly                                 |
| An options object, interface or class for one setting          | A plain parameter. Add structure when the second case shows up     |
| Unrequested "Example usage" blocks or demo `main`              | Nothing. Tests show usage                                          |
| Docstrings that restate the function name on private helpers   | No docstring, or one line on why it exists                         |
| `if (x) { return true } else { return false }`                 | `return x`                                                         |
| `else` after `return`, deep nesting                            | Early returns, flat control flow                                   |
| Logging every step ("Starting...", "Done!")                    | Log decisions and failures, with the values that matter            |
| `"An error occurred while processing the request"`             | `"invoice ${id}: tax region ${region} not configured"`             |
| Placeholder `// TODO: implement` or `pass` left behind         | Finish it, or say plainly in your reply that it's unfinished       |
| Unused imports, parameters and variables "for later"           | Delete them                                                        |
| Leftover `print` or `console.log` debugging                    | Delete them                                                        |
| Layers of manager/service/handler for a small feature          | Only the layers the codebase already uses                          |
| Every function the same length and shape                       | Shape follows the problem. Some functions are two lines            |
| New dependency for something a few lines can do                | A few lines. Add a dependency only when it earns its place         |

#### Errors

- Fail fast at the boundary, with a message that names the value and what was expected.
- Add context when rethrowing (`fmt.Errorf("load config %s: %w", path, err)`, `raise X from err`,
  `.context("...")`). Never rethrow unchanged just to log.
- Use the project's existing error types and patterns before inventing new ones.
- Never silently swallow an error. If ignoring one is correct, say why in a short comment.

#### Before and after

AI version:

```typescript
/**
 * Fetches the user data from the API and processes it.
 * @param userId - The ID of the user to fetch
 * @returns The processed user data
 */
async function fetchAndProcessUserData(userId: string): Promise<UserData | null> {
  try {
    // Check if userId is valid
    if (!userId || userId === undefined || userId === null) {
      console.log("Invalid user ID provided");
      return null;
    }
    // Make the API request
    const response = await fetch(`/api/users/${userId}`);
    // Parse the response data
    const data = await response.json();
    // Process and return the result
    const result = processData(data);
    return result;
  } catch (error) {
    console.error("An error occurred while fetching user data:", error);
    return null;
  }
}
```

Human version:

```typescript
async function getUser(id: string): Promise<User> {
  const res = await fetch(`/api/users/${id}`);
  if (!res.ok) throw new Error(`get user ${id}: ${res.status}`);
  return toUser(await res.json());
}
```

The second one is shorter, and it's also more correct. It checks the HTTP status, which the first
version never did. And it surfaces failures instead of turning every error into a silent `null`.

#### Idioms by language

Write the language the way its best codebases do, not as a translation from another one.

- TypeScript: let inference work and annotate exported signatures. Prefer `unknown` to `any`, narrow
  with type guards, use `satisfies` for config objects, and avoid gratuitous classes.
- Python: comprehensions over manual append loops, `pathlib`, f-strings, context managers,
  dataclasses for plain records, and EAFP where it reads cleaner. Type hints on public functions.
- Rust: `?` over `match`-and-return, iterators over index loops, borrow before cloning, `anyhow` or
  `thiserror` as the crate already does. No `unwrap()` outside tests and proven-safe spots.
- Go: return errors and wrap them with `%w`. Keep interfaces small and define them where they're
  used. Use the zero value, keep names short in small scopes, and run `gofmt`.
- SQL: explicit column lists, parameters and never string concatenation, and indexes for the queries
  you add.

#### Scope discipline

- Change what was asked. Don't refactor neighbouring code, rename things or reformat files you only
  passed through.
- Match the existing patterns even where you'd have chosen differently, unless they're the bug.
- Keep functions focused (under 50 lines where practical). Split by responsibility, not to hit a
  number.
- Add or update tests when behaviour changes, in the same style as the existing tests.
- Think about performance where it matters (hot paths, N+1 queries, unbounded loops) and nowhere
  else.

#### Before you hand it back

Read the diff as a reviewer who has seen a thousand AI pull requests:

1. Could any line be deleted without changing behaviour? Delete it.
2. Does every comment say something the code can't?
3. Would this name make sense to someone who only knows the domain?
4. Does every error path either recover, add context or fail loudly?
5. Does it look like the files around it?
6. Does it build, pass the tests and do exactly what was asked?

A detector score never outranks correctness. Clean, correct, idiomatic code is the human signal.

### 10a. Build it like a senior production engineer

Rule 10 covers how code reads. This rule covers how it's built. For any non-trivial coding task
(features, bug fixes, refactors, APIs, database work, frontend or backend changes, anything with
money, auth or other critical state), read
[references/senior-engineer.md](references/senior-engineer.md) before starting and follow it. It has
the full guide: requirement analysis, repo inspection, design, naming, function design, errors,
validation, security, data integrity, edge cases, testing, self-review, adversarial review,
regression protection, dependencies, logging, performance, databases, APIs, frontend, backend,
financial operations, and the final review checklist.

Priority order when goals conflict:

**Correctness → Security → Maintainability → Simplicity → Performance**

The workflow, in short:

1. **Understand.** Goal, constraints, inputs, outputs, affected parts, risks. If an ambiguity would
   change the implementation, ask one focused question. For minor ones, pick the safest reading and
   state the assumption.
2. **Inspect.** Relevant files, existing patterns, dependencies, data flow, tests. Work out what
   must keep working.
3. **Design.** Responsibilities, data flow, failure behaviour, security boundaries, test plan.
4. **Implement.** The smallest clean change that meets the requirement, in the project's
   conventions.
5. **Verify.** Formatter, linter, type checker, tests and build, using only tools the project
   actually has.
6. **Break it.** Invalid input, missing data, duplicate and concurrent requests, unauthorized
   access, failure halfway through, unexpected responses, boundary values.
7. **Fix.** Fix what you found. Don't just report it when a fix is within reach.
8. **Review.** Read the full diff. Strip debug output, unused imports, temporary hacks and
   accidental changes from the code you added. Existing code stays (Standing instruction 2).
9. **Report.** Say what was implemented, what changed, which tests ran, and the known limits.

Never claim "bug-free", "100% secure", "production-ready" or "all tests pass" without evidence. Say
exactly what you verified and what you didn't.

Four questions to keep asking:

- Before implementing: **what could go wrong?**
- Before finishing: **how would I try to break this?**
- Before an architectural decision: **what's the simplest design that correctly solves this?**
- Before the final answer: **what have I actually verified?**

### 10b. No AI-generated code patterns

Whenever you write, change or review code, also read
[references/ai-code-patterns.md](references/ai-code-patterns.md). It lists 29 patterns that mark
code as low-quality AI output, with examples. The ones rule 10 doesn't already cover:

- **Hallucinated APIs.** Never import a package, call a method or use a hook, component or framework
  feature you haven't confirmed exists in the project's dependencies or the library's real API. A
  plausible-sounding name isn't proof.
- **Invented files and architecture.** Don't assume a file, folder, service or utility exists. Look
  first, and don't force a generic architecture onto the repo.
- **Duplication.** Before adding logic, check whether the codebase already does it. Reuse it instead
  of copying a block and tweaking it.
- **Hardcoded values.** No secrets, credentials, tokens, URLs, IDs, limits, fees or
  environment-specific values in code. Use the project's config or constants.
- **Fake implementations.** No empty functions, stubs or fake logic dressed up as finished work.
- **Uniformity.** Don't force every function, component, hook, service or controller into the same
  template, and don't make the structure symmetrical just because symmetry looks clean.
- **Pattern copying.** No Factory, Strategy, Adapter, Repository, Manager, Singleton, Observer or
  Builder unless the problem actually benefits from it.
- **Textbook and context-free code.** Use the project's real domain, data models, utilities and
  conventions. The code should obviously belong to this system and no other.
- **Generic CRUD.** Model the actual business operation rather than wrapping database calls.
- **Boilerplate comment sequences.** No "First, we... Next, we... Finally, we..." and no "Validate
  input / Process request / Return result" scaffolding.
- **Over-compression.** Don't cram unrelated operations into one-liners to save lines. Readability
  beats line count in both directions.
- **Generic error messages.** No "Something went wrong" or "An error occurred" when a specific
  message is possible.
- **False confidence and maintenance debt.** Before calling it done, check for wrong imports, wrong
  API usage, type issues, races, hidden side effects, coupling and oversized functions.

Write code that is specific, intentional, context-aware, simple, maintainable, readable and
consistent with the existing project, the way an experienced engineer who understood the system
would write it.

### 10c. Human-style code, written naturally

Rules 10, 10a and 10b say what to avoid. This rule keeps that from tipping into a new kind of
artificial code that's over-named, over-split or over-polished. Whenever you write, change or review
code, also read [references/human-style-code.md](references/human-style-code.md), which has all 20
principles with examples and the human-code review.

- **Name length follows scope.** `i`, `n`, `x` and `response` are natural in a small scope. Save
  long names for wide scopes and domain concepts. Never write `currentProcessedUserData`,
  `finalCalculatedTransactionResult` or `authenticatedUserProfileResponse` when `user`, `total` or
  `res` is clear. This refines rule 9 and the generic-name list in 10b. Those target names that live
  across wide scopes or stand in for domain concepts, not loop counters.
- **No LLM vocabulary in identifiers.** No `processedData`, `transformedData`, `resultData`,
  `finalResult`, `validatedData` or `handledRequest` when the domain gives you `order`, `wallet`,
  `invoice` or `token`.
- **Don't normalise.** Don't rename existing identifiers to make them uniform. Follow the language,
  the repo and the surrounding code.
- **Complexity follows the problem.** Nested branches, state machines and specialised structures are
  fine when the problem needs them. Don't simplify working code into something less correct or less
  clear, and don't inflate `const total = price * quantity` into layers.
- **Natural granularity.** Don't split one small operation into `getUser`/`validateUser`/
  `prepareUser`/`handleUser`. Don't merge real separate responsibilities into one giant function
  either.
- **No over-explaining through code.** Write `return user != null`, not three intermediate
  variables. Add an intermediate variable only when it names something worth naming.
- **Simple functions stay simple.** `add(a, b)` needs no docblock.
- **Real development markers stay.** Keep genuine TODO, FIXME, HACK and XXX notes. Never invent them
  to look human.
- **Natural variation.** Functions differ in length, shape and abstraction level because the
  problems differ. One module has 3 functions and another has 8. Don't manufacture symmetry.
- **Domain-shaped code.** Payment code should look like payment code, and auth code like auth code.
- **Don't manufacture anything.** No fake mistakes, no randomness and no imperfections inserted on
  purpose. Just don't erase legitimate ones. In code this overrides the controlled shorthand in Part
  4, which stays limited to comments, commits and informal prose.

After writing, run the human-code review: Is it specific to this project? Are names natural for
their scope? Did I lengthen names, repeat the code in comments, add unneeded docs, create
abstractions because they were available, force one structure everywhere, or normalise existing
code? Would an experienced developer naturally maintain this? If not, revise.

## Part 3: Git and review

### 11. No AI co-author trailers

Never do:

```
git commit --trailer "Co-authored-by: Cursor <cursoragent@cursor.com>"
```

Never include:

```
Co-authored-by: Cursor <cursoragent@cursor.com>
Co-authored-by: GitHub Copilot <copilot@github.com>
Generated-by: Claude
```

These trailers mark commits as AI-generated in git history permanently. Removing them later means
rewriting history. The same goes for "Generated by" or "Created with" lines in files.

### 12. Commit messages like a person writes them

AI pattern:

```
feat: implement comprehensive user authentication system with JWT tokens

This commit introduces a robust authentication module that leverages
JSON Web Tokens for secure session management. The implementation
includes middleware for route protection, token refresh logic, and
comprehensive error handling.
```

Human pattern:

```
add JWT auth

Middleware for protected routes, token refresh, error handling.
```

Defaults when the repo has no strong convention:

- Lowercase subject, no trailing period, under 50 characters, about 5-7 words
- Say what changed: "Refactor user login logic for better speed" becomes "fixed login lag"
- Body only when the why isn't obvious
- No `feat:`/`fix:`/`chore:` prefix unless the project uses them or the user asks for conventional
  commits
- No marketing language in the body

### Pull request descriptions

- What changed (files or areas)
- Why (link the issue, or one sentence)
- How to test
- Never "This PR introduces a comprehensive..."

## Part 4: Voice and entropy

These settings shape tone for comments, commits and informal prose. Turn them down for formal docs.

- Case: lowercase for short descriptions, sentence case for longer explanations, and whatever the
  codebase already does wins
- Punctuation: drop the terminal period on most single-line strings and commit subjects (around
  80%), keep it on full sentences in docs
- Semantic compression: use the dev shorthand people actually type

  | Formal         | Shorthand |
  | -------------- | --------- |
  | authentication | auth      |
  | information    | info      |
  | utility        | utils     |
  | configuration  | config    |
  | parameter      | param     |
  | implementation | impl      |
  | optimization   | opt       |
  | synchronous    | sync      |
  | repository     | repo      |

- Grammar: fragments are fine. Brevity beats textbook English. Use contractions where they're
  natural
- Persona: a tired senior developer. Concise, informal, practical, doesn't explain the obvious
- Entropy: vary sentence and comment length a lot. Mix one-liners with the occasional longer note
- Controlled shorthand: now and then (about 1 in 20 comments or commit lines) use a looser form a
  busy dev would type, like "conf" for "config". Only in comments, commit messages and informal
  prose. Never in identifiers, string literals, user-facing copy, docs or anything that has to
  compile or be searched for

## Part 5: Rewrite mode

When the user hands you something to humanize, transform it with every rule above and return only
the result. No preamble, no "Sure, here's the humanized version", no closing note. Don't wrap it in
a code fence unless the input was a full code file.

Before:

```rust
/**
 * This function implements a robust authentication mechanism
 * that ensures secure user access to the system.
 */
fn authenticate_user(user: User) -> Result<bool> {
    // Implementation details
}
```

After:

```rust
// temp auth fix - needs proper token refresh
fn auth_user(user: User) -> Result<bool> {
    // ...
}
```

Before:

```
feat: Implement comprehensive user authentication with robust error handling
```

After:

```
fixed login lag
```

Keep behaviour identical when rewriting code. Rename only local or private identifiers unless the
user asks for more, and update every reference you rename.

## Part 6: Audit mode

Scan README.md, CHANGELOG.md, docs/, other `*.md`, code comments, `package.json` descriptions, PR
templates and recent git history. If the ApexCode CLI or scripts are installed, run them first and
fold their output into your report (see Part 7).

Look for:

| Check                                                                                                  | Severity |
| ------------------------------------------------------------------------------------------------------ | -------- |
| Em dashes used as separators                                                                           | high     |
| Flagged vocabulary (Reference list)                                                                    | high     |
| AI git metadata: co-author trailers, "Generated by" lines                                              | high     |
| Swallowed errors, catch blocks that only log and rethrow                                               | high     |
| Opening and closing cliches                                                                            | medium   |
| Narrating code comments                                                                                | medium   |
| Code tells from rule 10: generic names, needless wrappers, leftover logs, dead code, placeholder TODOs | medium   |
| Over-formatting: bold-label bullets, uniform lists, emoji                                              | medium   |
| Over-descriptive or templated identifiers                                                              | low      |
| Low burstiness: 4+ consecutive sentences of similar length                                             | low      |

For each finding give the file and line, quote the text, suggest a human replacement and rate it.
Finish with a count per category. To fix the findings rather than just list them, offer Retrofit
mode.

## Part 6b: Retrofit mode

For developers whose code was written before ApexCode, or without it. You read their existing code,
apply every rule in this skill, and rewrite or remove whatever violates it, without breaking the
app. Before starting, read [references/retrofit.md](references/retrofit.md). It's the full protocol,
and every step in it is mandatory.

The constraint that overrides everything else in this mode: **observable behaviour stays
identical.** That means the same outputs, public API, stored data, wire formats and side effects. A
cleaner codebase that behaves differently is a failed retrofit.

The protocol in short:

1. **Scope and safety net.** Confirm which paths to cover. Require a clean git tree and work on a
   new branch. Never touch generated, vendored or migration code.
2. **Baseline.** Run build, type check, lint and tests, and record the results. Snapshot the public
   surface: exports, routes, CLI flags, env vars, config keys, schemas. Pre-existing failures get
   reported, not fixed silently.
3. **Map.** Run Audit mode and give each finding a risk tier:
   - **Tier 0:** comments, docs, attribution, no runtime effect. Fix freely.
   - **Tier 1:** local and provable, such as unused imports, dead branches, debug logs and local
     renames. Fix in verified batches.
   - **Tier 2:** behaviour-adjacent, such as swallowed errors, duplication, hardcoded config and
     unused private functions. Fix only behind a test that pins today's behaviour, using the
     behaviour-preserving recipes.
   - **Tier 3:** contract-changing, such as public names, routes, schemas and messages that clients
     parse. Never change these. Recommend them instead.
4. **Prove before removing.** Search the whole repo, including strings, templates, config and
   reflection, before deleting or renaming anything. If you can't rule out dynamic use, it's Tier 3.
5. **Characterization tests** pin current behaviour, quirks included, before any Tier 2 change.
6. **Small verified batches.** Work one tier, one category and one module at a time. Run the fast
   checks after each batch, and commit it with a human-style message. A failing check means you
   revert the batch and record why. Never edit or delete a test to make a batch pass.
7. **Full verification.** Re-run the baseline and compare. Diff the public surface. Read the whole
   diff. Re-audit the scope.
8. **Report.** List what changed, the tests and their before and after results, what was skipped and
   why, Tier 3 recommendations, and what was not verified.

Keep what this skill says to keep: genuine TODOs, short names in small scopes, the repo's own
conventions, comments that explain why, and quirks callers may depend on.

## Part 7: Toolchain

The skill works on its own. The toolchain adds measurement and automation.

### CLI

```bash
apexcode              # dashboard (default)
apexcode scan         # detect AI patterns in staged files
apexcode fix          # auto-fix AI patterns (--dry-run to preview)
apexcode score        # stealth score (--detailed for the breakdown)
apexcode dashboard    # terminal UI with per-file heatmap
apexcode config       # view or reset settings
apexcode init         # set up .apexcode/ in the current repo
apexcode install      # IDE integration: claude, cursor, windsurf, antigravity, all
```

### Pre-commit hooks

`.pre-commit-config.yaml` wires up four checks from `scripts/`:

- `stealth_check.py` scores files for AI signatures
- `auto_humanize.py` rewrites narrating comments
- `naming_check.py` flags over-descriptive identifiers
- `check_commit_entropy.py` checks commit messages at the `commit-msg` stage

### Configuration

The CLI reads `.apexcode/config.toml`:

```toml
[detection]
threshold = 0.15            # AI probability threshold (0.0 - 1.0)
use_local = true            # local RoBERTa model first
use_cloud_fallback = true   # cloud API when the local score is uncertain

[humanization]
auto_humanize = false
entropy_level = 0.5         # transformation intensity (0.0 - 1.0)

[jitter]
enabled = false             # spread staged commits over time
min_delay_secs = 60
max_delay_secs = 300
```

Agents that read JSON settings can use:

```json
{
  "apexcode.enabled": true,
  "apexcode.human_like": true,
  "apexcode.no_emojis": true,
  "apexcode.descriptive_names": true
}
```

Set `CLAUDE_CODE_UNDERCOVER=1` in `.zshrc` or `.bashrc` to keep this mode on for every session.

### Training pipeline

`training/` holds a four-stage pipeline for training a detector-guided style-transfer model:
human-sample construction, style SFT plus hard-negative inference, DPO alignment, and refinement.
Use it when rule-based rewriting isn't enough and you want a model tuned on your own writing.

## Reference

### Words to never use in prose

comprehensive, robust, ensure, enhance, enhanced, leverage, leveraging, utilize, utilizing,
facilitate, facilitating, meticulous, seamless, delve, tapestry, streamline, paramount, pivotal,
holistic, crucial, multifaceted, indispensable, nuanced, intricate, harness, foster, bolster,
spearhead, elevate, empower, unleash, supercharge, optimize, cutting-edge, game-changing,
revolutionary, groundbreaking, best-in-class, future-ready, scalable, next-generation, landscape,
ecosystem, paradigm, synergy

### Phrases to never use

- "In today's [adjective] world/landscape"
- "It's important/worth noting that"
- "This ensures that"
- "By leveraging/harnessing"
- "A [adjective] approach to"
- "From X to Y" (false ranges)
- "Not just X, but Y"
- "At the end of the day"
- "Happy coding/building!"
- "Let's dive in"

### Structural patterns to never use

- Em dashes (—) as separators
- Tricolons in every list ("X, Y, and Z")
- Uniform paragraph or bullet lengths
- Bold-label bullets: **Feature:** description
- Opening with a question: "Ever wondered how...?"
- Closing with a call to action: "Start using X today!"

### Git patterns to never use

- Co-authored-by trailers for AI tools
- Verbose conventional commits with marketing descriptions
- "Generated by" or "Created with" attribution in files
