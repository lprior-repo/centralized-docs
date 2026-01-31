---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#4-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 142
summary: (a lattice, to be precise). Even entire configurations and schemas are placed in this hierarchy
---

(a lattice, to be precise).
Even entire configurations and schemas are placed in this hierarchy.

WHAT IS A LATTICE?

This section is useful to understand what a lattice is,
but is not strictly needed to grasp the following sections,
nor the specifics of CUE itself. Skip at will.

A lattice is a partially ordered set, in which every two elements
have a unique least upper bound (join) and greatest lower bound (meet).
By definition this means there is always a single root (top) and a single
leaf (bottom).
Let’s consider what this means by looking at an example.
