---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#131-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 130
summary: value: \"hello\" & matchIf(string, len(value) > 3, value > 10)  // true. // If value matches {a: int}, it must have b field; otherwise a must be a string
---

value: "hello" & matchIf(string, len(value) > 3, value > 10)  // true

// If value matches {a: int}, it must have b field; otherwise a must be a string
x: {a: 1} & matchIf(x, {a: int}, {a: int, b: int}, {a: string})  // false: missing b

// If value is >5, it must be <10; otherwise it must be <3
y: 2 & matchIf(y, >5, <10, <3)  // true: 2 is <=5, so <3 is checked

CYCLES

Implementations are required to interpret or reject cycles encountered
during evaluation according to the rules in this section.

REFERENCE CYCLES
