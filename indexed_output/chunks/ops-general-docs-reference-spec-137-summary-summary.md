---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#137-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 131
summary: evaluate to this value. STRUCTURAL CYCLES
---

evaluate to this value.

STRUCTURAL CYCLES

A structural cycle is when a node references one of its ancestor nodes.
It is possible to construct a structural cycle by unifying two acyclic values:


Copy code
Copied!

// acyclic
y: {
    f: h: g
    g: _
}
// acyclic
x: {
    f: _
    g: f
}
// introduces structural cycle
z: x & y

Implementations should be able to detect such structural cycles dynamically.

A structural cycle can result in infinite structure or evaluation loops.


Copy code
Copied!

// infinite structure
