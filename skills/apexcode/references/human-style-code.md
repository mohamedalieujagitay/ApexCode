# Human-Style Code

Part of the ApexCode skill. Load this file whenever you write, change or review code, together with
[senior-engineer.md](senior-engineer.md) and [ai-code-patterns.md](ai-code-patterns.md). This file
is the balance check for the other two. Avoiding AI patterns must never turn into manufactured
verbosity, forced uniformity or fake imperfection.

How it fits with the rest of ApexCode:

- **Name length follows scope.** Short names (`i`, `n`, `x`, `res`, `response`) are fine in small
  scopes where the meaning is obvious. The generic-name warnings in ai-code-patterns.md apply to
  names that live across wider scopes or carry domain meaning.
- **No manufactured imperfections in code.** This file overrides SKILL.md's controlled shorthand and
  stochastic variation wherever code is concerned. That shorthand applies only to comments, commit
  messages and informal prose, and only when it matches the repo's own voice.
- **Never erase what's already there.** Existing TODOs, FIXMEs, short names and stylistic quirks
  stay (Standing instruction 2 in SKILL.md).

Write code in the style of real, experienced human developers.

The code should feel naturally authored for the specific project, rather than mechanically generated
from generic templates.

Do not force artificial "human imperfections." Write naturally and intentionally.

## 1. Use natural, conventional code

Human developers generally prefer familiar, idiomatic solutions.

Prefer:

- established language conventions
- familiar control flow
- straightforward implementations
- patterns already used in the project
- solutions appropriate to the language and task

Do not introduce unusual patterns simply because they are technically possible.

---

## 2. Use natural variable and function names

Do not assume every identifier needs a long descriptive name.

Use names appropriate to their scope and context.

Short names are acceptable when their meaning is obvious:

```c
int i;
int n;
int j;
```

```python
for i in range(n):
```

Use longer names when they genuinely improve clarity:

```ts
transferAmount;
receiverWallet;
transactionFee;
```

Do not automatically transform every short identifier into a long, overly descriptive name.

Avoid artificial names such as:

```ts
currentProcessedUserData;
finalCalculatedTransactionResult;
validatedInputDataObject;
```

when a shorter natural name is sufficient.

---

## 3. Do not standardize every name

Do not mechanically normalize every identifier into one naming pattern.

Follow the conventions of:

- the language
- the existing repository
- the surrounding code
- the developer's established context

Do not unnecessarily rename existing variables simply to make them more uniform.

---

## 4. Allow natural local names

Small scopes may use concise names when context makes them obvious.

Examples:

```ts
for (let i = 0; i < items.length; i++) {
  ...
}
```

```python
for x in values:
    ...
```

```c
int n = 10;
```

Do not replace every natural short identifier with an unnecessarily verbose alternative.

---

## 5. Write comments that actually carry information

Do not comment every obvious operation.

Avoid:

```ts
// Get user
const user = getUser();
```

```ts
// Increment i
i++;
```

```ts
// Return result
return result;
```

Prefer comments that explain information not obvious from the implementation:

```ts
// Keep the previous price during checkout so an inventory update
// cannot silently change an order that is already being processed.
```

Comments should communicate:

- reasoning
- constraints
- unusual behavior
- technical decisions
- workarounds
- important context

---

## 6. Allow real development context

Human codebases naturally contain signs of ongoing development.

When appropriate, preserve meaningful development markers such as:

```text
TODO
FIXME
HACK
XXX
```

Do not automatically remove them simply because they make the code look less polished.

However, do not create fake TODOs or technical debt merely to imitate human code.

Only include them when they represent a genuine unresolved issue.

---

## 7. Do not force complete documentation

Do not automatically create complete docstrings for every function.

Documentation should exist where it provides useful information.

A simple function can remain simple:

```ts
function add(a: number, b: number) {
  return a + b;
}
```

Do not add a large documentation block merely because the function exists.

---

## 8. Preserve natural complexity

Do not assume human code must always be simple.

Complexity should reflect the actual problem.

Some code may legitimately require:

- nested conditions
- multiple branches
- state transitions
- complex algorithms
- domain-specific abstractions
- specialized data structures

Do not artificially simplify working code when doing so would reduce correctness or clarity.

---

## 9. Do not make everything perfectly symmetrical

Real code evolves around actual requirements.

Do not force every module, component, function, or class to have identical structure.

One component may need:

```text
3 functions
```

while another needs:

```text
8 functions
```

One module may require an abstraction while another does not.

Do not manufacture symmetry for its own sake.

---

## 10. Follow existing human conventions

Before writing code, inspect the surrounding implementation.

Learn:

- naming patterns
- formatting
- function structure
- comments
- file organization
- API usage
- error handling
- testing style

Then write code that naturally belongs beside it.

The surrounding repository is the strongest style reference.

---

## 11. Do not automatically expand every idea

Do not turn a small implementation into a large architecture.

A human developer may reasonably write:

```ts
const total = price * quantity;
```

rather than creating several layers around it.

Use abstractions when the system needs them.

Do not generate abstractions simply because the AI knows a pattern that could be applied.

---

## 12. Use the language naturally

Code should reflect normal idioms of the language being used.

For example:

**C**

```c
for (int i = 0; i < n; i++) {
    ...
}
```

**JavaScript / TypeScript**

```ts
const users = activeUsers.filter(isActive);
```

**Python**

```python
for item in items:
    ...
```

Do not impose one universal coding style across different languages.

---

## 13. Do not over-explain through code

Avoid unnecessary intermediate variables when they add no meaning.

Instead of:

```ts
const userIsAuthenticated = user != null;
const authenticationResult = userIsAuthenticated;
return authenticationResult;
```

write:

```ts
return user != null;
```

Use intermediate variables when they improve understanding, not merely to make the code longer.

---

## 14. Do not make every variable perfectly descriptive

Context matters.

This:

```ts
const response = await fetch(url);
```

can be completely natural.

This:

```ts
const authenticatedUserProfileResponse = await fetch(userProfileEndpoint);
```

may be unnecessarily verbose.

Choose the smallest name that remains clear in context.

---

## 15. Expect natural variation

Do not force every function to have:

```text
exactly the same number of lines
exactly the same number of variables
exactly the same control-flow pattern
exactly the same comment structure
exactly the same abstraction level
```

Natural code varies according to the problem.

---

## 16. Code should reflect the author's understanding

Do not generate generic solutions disconnected from the product.

Use the project's actual:

- domain language
- business rules
- existing data structures
- APIs
- naming
- architecture
- constraints

A payment system should look like payment-system code.

An inventory system should look like inventory-system code.

An authentication system should look like authentication-system code.

---

## 17. Do not automatically remove all imperfections

Do not "polish" code into unnatural uniformity.

Real code can contain:

- concise identifiers
- different levels of abstraction
- occasional TODOs
- localized comments
- small stylistic variations
- simple functions beside complex ones

Do not manufacture these characteristics.

Simply do not erase legitimate ones unnecessarily.

---

## 18. Avoid generic LLM vocabulary

Avoid repeatedly generating artificial identifiers such as:

```text
processedData
transformedData
resultData
finalResult
responseData
validatedData
updatedData
handledRequest
processedRequest
```

when the domain provides a more natural name.

Prefer:

```text
order
user
wallet
transaction
product
message
token
invoice
```

when those are the actual concepts.

---

## 19. Use natural function granularity

Do not split every operation into a separate function.

Avoid:

```ts
getUser();
validateUser();
processUser();
prepareUser();
handleUser();
returnUser();
```

when the operation is naturally one small unit.

Likewise, do not create massive functions when the logic genuinely contains separate
responsibilities.

Choose boundaries naturally.

---

## 20. Preserve individuality without sacrificing quality

Human developers have recognizable individual coding styles.

Allow reasonable variation in:

- identifier length
- control-flow choices
- function structure
- commenting
- formatting conventions
- implementation details

while maintaining correctness and the project's established conventions.

Do not intentionally randomize style.

---

## Human-code review

After implementation, review the code and ask:

- Does this feel specific to the project?
- Are the names natural for their scope?
- Did I unnecessarily make names longer?
- Did I add comments that simply repeat the code?
- Did I add documentation that nobody needs?
- Did I create abstractions because they were necessary, or because they were available?
- Did I force every function into the same structure?
- Did I unnecessarily normalize existing code?
- Did I duplicate familiar patterns instead of using the project's existing implementation?
- Does the code read like something an experienced developer would naturally maintain?

If not, simplify or revise it.

## Final standard

Write code that is:

**Natural. Contextual. Intentional. Idiomatic. Specific to the project. Readable. Maintainable.
Appropriately complex.**

Do not manufacture "human mistakes."

Do not manufacture randomness.

Do not manufacture imperfections.

Simply write software the way an experienced human engineer would naturally write it when solving
the actual problem inside the actual codebase.
