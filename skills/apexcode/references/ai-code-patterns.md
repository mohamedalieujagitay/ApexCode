# Avoid AI-Generated Code Patterns

Part of the ApexCode skill. Load this file whenever you write, change or review code, alongside
[senior-engineer.md](senior-engineer.md). SKILL.md rule 10 has the short version and the before and
after example. This file has the full list.

Where a section below says to remove code, it applies to code you add in this task. Existing code
stays unless the user asked for it to be reworked (Standing instruction 2 in SKILL.md).

Write code that does not exhibit the common characteristics associated with low-quality AI-generated
code.

The goal is professional, natural, context-aware engineering code that fits the existing codebase.

## 1. Avoid overly simple or repetitive structures

Do not automatically produce the same patterns repeatedly across the codebase.

Avoid making every function, component, class, or module follow an identical template when the
problem does not require it.

Do not default to repetitive architectural patterns simply because they are common in training data.

Code should reflect the specific problem being solved.

---

## 2. Avoid generic, repetitive code

Do not generate repetitive implementations when existing logic can be reused appropriately.

Look for duplicated:

- business logic
- validation
- transformations
- API handling
- state management
- database operations

Do not copy and slightly modify the same block repeatedly.

Use appropriate reuse where it improves maintainability.

---

## 3. Avoid excessive uniformity

Do not make the entire codebase look mechanically generated.

Avoid forcing every:

- function
- component
- class
- hook
- service
- controller
- utility

into the exact same structure.

Use patterns where they are useful, not mechanically.

---

## 4. Avoid unnecessary abstractions

Do not over-engineer simple problems.

Do not introduce unnecessary:

- classes
- interfaces
- services
- repositories
- factories
- managers
- wrappers
- helpers
- adapters
- configuration layers
- abstraction layers

Every abstraction should have a real purpose.

Prefer straightforward code when straightforward code is sufficient.

---

## 5. Avoid generic names

Avoid meaningless names such as:

```ts
data;
result;
response;
item;
value;
obj;
temp;
info;
thing;
helper;
processData;
handleData;
doSomething;
```

when more specific names are appropriate.

Use names that communicate the actual domain and purpose.

Prefer:

```ts
transferAmount;
senderWallet;
receiverWallet;
transactionFee;
authenticatedUser;
inventoryItem;
```

---

## 6. Avoid redundant comments

Do not write comments that simply describe the line immediately below them.

Avoid:

```ts
// Increment counter
counter++;
```

```ts
// Get user
const user = await getUser();
```

```ts
// Return result
return result;
```

Comments should provide useful context, reasoning, constraints, or non-obvious behavior.

---

## 7. Avoid excessive docstrings

Do not generate large documentation blocks for simple or self-explanatory functions.

Avoid adding documentation merely for appearance.

Documentation should exist where it provides meaningful information that cannot be understood easily
from the code itself.

---

## 8. Avoid boilerplate AI comments

Do not repeatedly use patterns such as:

```ts
// First, we...
// Next, we...
// Finally, we...
```

or:

```ts
// Validate input
// Process request
// Handle response
// Return result
```

when the comments add no meaningful information.

---

## 9. Avoid excessive try/catch

Do not wrap every operation in generic error handling.

Avoid patterns such as:

```ts
try {
  // everything
} catch (error) {
  console.error(error);
}
```

when there is no meaningful recovery or handling strategy.

Do not silently suppress errors.

---

## 10. Avoid duplicated blocks

Do not repeatedly generate nearly identical code blocks.

Before adding code, check whether the same behavior already exists.

Prefer an existing implementation when appropriate.

---

## 11. Avoid hardcoded values

Do not unnecessarily hardcode:

- secrets
- credentials
- tokens
- configuration
- URLs
- IDs
- limits
- fees
- environment-specific values

Use the project's established configuration or constants structure.

---

## 12. Avoid invented libraries or APIs

Never assume an API, library, method, package, component, hook, SDK function, or framework feature
exists.

Use only verified project dependencies and APIs.

Do not invent imports such as:

```ts
import { something } from "fake-library";
```

Do not invent methods because their names sound plausible.

---

## 13. Avoid invented files or architecture

Do not assume files, folders, services, utilities, or abstractions exist.

Inspect the existing project first.

Do not impose a generic architecture onto an existing repository.

---

## 14. Avoid unused code

Do not leave behind:

- unused imports
- unused variables
- unused functions
- dead branches
- temporary debugging code
- commented-out abandoned code
- unnecessary dependencies

Remove unnecessary code before completion.

---

## 15. Avoid hardcoded debugging

Do not leave behind:

```ts
console.log(...)
console.debug(...)
alert(...)
debugger;
```

or similar temporary debugging code unless it is intentionally part of the final implementation.

---

## 16. Avoid empty or fake implementations

Do not create functions merely to satisfy structure.

Avoid:

```ts
function processData() {}
```

or placeholder implementations pretending to be complete.

Do not generate fake logic merely to make the feature appear finished.

---

## 17. Avoid overly verbose code

Do not create 100 lines of code where 20 clear lines solve the same problem.

Remove unnecessary:

- conditions
- variables
- wrappers
- transformations
- intermediate objects
- abstractions
- repeated logic

while preserving readability.

---

## 18. Avoid artificially compressed code

Do not compress many unrelated operations into unreadable one-liners.

Avoid code that sacrifices readability merely to reduce line count.

Code should be easy to read and reason about.

---

## 19. Avoid AI-like "perfect" structure

Do not automatically make every part of the project look perfectly symmetrical.

Do not create identical structures simply because symmetry appears clean.

Real production code should reflect the actual requirements and constraints of the system.

---

## 20. Avoid unnatural naming

Do not use unnecessarily formal, verbose, or artificial names.

Use naming that would naturally be chosen by an experienced developer working on the specific
product.

Naming should reflect the domain.

---

## 21. Avoid unnecessary refactoring

When implementing a feature, do not rewrite unrelated code.

Do not rename unrelated variables, reorganize unrelated files, or replace working implementations
unless required for the task.

Keep changes focused.

---

## 22. Avoid context-free code

Do not generate generic code that could belong to any project.

Use the actual:

- domain
- naming conventions
- architecture
- data models
- APIs
- state management
- database structure
- existing utilities
- project conventions

Code should clearly belong to the project it is written for.

---

## 23. Avoid blind edge-case ignorance

Do not silently assume only the happy path exists.

Check relevant cases such as:

```text
null
undefined
empty input
invalid input
zero
negative values
maximum values
missing resources
duplicate requests
unexpected responses
failed network requests
database failures
```

Handle the cases that are relevant to the feature.

---

## 24. Avoid false confidence

Do not assume code works merely because it looks correct.

Do not generate code and immediately declare it complete.

Inspect the implementation for:

- logical bugs
- incorrect assumptions
- missing conditions
- incorrect imports
- incorrect API usage
- type issues
- race conditions
- duplicated behavior
- security problems
- integration problems

---

## 25. Avoid code that "looks like it came from a textbook"

Do not mechanically produce generic textbook implementations when the existing project already has
established patterns.

Prefer context-specific implementation.

The code should feel like it was written for this particular system.

---

## 26. Avoid pattern copying without purpose

Do not add common design patterns simply because they are considered "professional."

Do not use:

```text
Factory
Strategy
Adapter
Repository
Service
Manager
Singleton
Observer
Builder
```

unless the actual problem benefits from them.

---

## 27. Avoid unnecessary repetition in error messages

Do not generate large numbers of generic messages such as:

```text
Something went wrong.
An error occurred.
Unable to process request.
Unexpected error.
```

when a specific, useful message can be provided.

Use meaningful error information appropriate to the application's architecture.

---

## 28. Avoid generic AI-generated CRUD when custom logic is required

Do not automatically generate generic CRUD patterns for domain-specific behavior.

Understand the business logic first.

The code should model the actual operation rather than merely wrapping database operations.

---

## 29. Avoid maintenance debt

Before completion, inspect the code for anything that will make future modification unnecessarily
difficult.

Look for:

- duplication
- unclear names
- unnecessary coupling
- deeply nested logic
- oversized functions
- excessive abstraction
- hidden side effects
- unexplained behavior

Remove avoidable sources of future maintenance problems.

---

## Final rule

Do not write code that is:

```text
generic
repetitive
over-engineered
boilerplate-heavy
artificially uniform
poorly named
unnecessarily documented
duplicated
hallucinated
hardcoded
context-free
```

Write code that is:

```text
specific
intentional
context-aware
simple
maintainable
readable
consistent with the existing project
```

The implementation should look like it was written by an experienced engineer who understood the
system, rather than generated mechanically from a generic template.
