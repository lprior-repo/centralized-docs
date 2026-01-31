---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#134-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 137
summary: evaluates to v. Unification is idempotent and unifying a value with itself ad infinitum,
---

evaluates to v.
Unification is idempotent and unifying a value with itself ad infinitum,
which is what the cycle represents, results in this value.
Implementations should detect cycles of this kind, ignore r,
and take v as the result of unification.


Copy code
Copied!

Configuration    Evaluated
//    c           Cycles in nodes of type struct evaluate
//  ↙︎   ↖         to the fixed point of unifying their
// a  →  b        values ad infinitum.

a: b & { x: 1 }   // a: { x: 1, y: 2, z: 3 }
b: c & { y: 2 }   // b: { x: 1, y: 2, z: 3 }
