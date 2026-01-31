---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#36-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 130
summary: the answer is 1 and verify that a-1 == 1 after resolving a. So CUE happily resolves this to
---

the answer is 1 and verify that a-1 == 1 after resolving a.

So CUE happily resolves this to


Copy code
Copied!

a: 2
b: 1

without resorting to any fancy algebraic constraint satisfaction solvers,
just plain ol’ logic.
Most cycles that do not result in infinite structures can be handled by CUE.
In fact, it could handle most infinite structures in bounded time
as well, but it puts limits on such cycles for
practical reasons.3

FILE ORGANIZATION

What applies at the language level also applies at the file level.
