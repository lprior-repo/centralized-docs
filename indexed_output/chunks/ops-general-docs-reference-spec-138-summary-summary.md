---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#138-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 132
summary: // infinite evaluation.     out: n + (f & {n: 1})
---

a: b: a

// infinite evaluation
f: {
    n:   int
    out: n + (f & {n: 1}).out
}

CUE must allow or disallow structural cycles under certain circumstances.

If a node a references an ancestor node, we call it and any of its
field values a.f cyclic.
So if a is cyclic, all of its descendants are also regarded as cyclic.
A given node x, whose value is composed of the conjuncts c1 & ... & cn,
is valid if any of its conjuncts is not cyclic.


Copy code
Copied!

// Disallowed: a list of infinite length with all elements being 1.
