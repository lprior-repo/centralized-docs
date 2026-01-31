---
doc_id: ops/general/docs-tour-expressions
chunk_id: ops/general/docs-tour-expressions#3-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 132
summary: 0 > 3  // type bool. // String types are compared lexically byte-wise
---

i: 3.0 > 3  // type bool
// String types are compared lexically byte-wise
j: "aaa" >= "ZZZ" // type bool

k: div(10, 3) // type int
l: mod(10, 3) // type int

TERMINAL

Copy code
Copied!

$ cue eval operators.cue
a: 2
b: "xxxOOO"
c: 4
d: 4.0
e: 3.00000
f: 3.0
g: 3.0
h: true
i: false
j: true
k: 3
l: 1

The CUE languge specification [/docs/reference/spec/#operators]
details the operator precedence levels.

Last modified September 23, 2025 [https://github.com/cue-lang/cuelang.org/commit/af004fcc0845b84296228c157951aba972957888]
