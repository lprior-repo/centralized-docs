---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#135-summary
chunk_level: summary
chunk_type: table
heading: Introduction
token_count: 131
summary: c: a & { z: 3 }   // c: { x: 1, y: 2, z: 3 }. // resolve a             b & {x:1}
---

c: a & { z: 3 }   // c: { x: 1, y: 2, z: 3 }

// resolve a             b & {x:1}
// substitute b          c & {y:2} & {x:1}
// substitute c          a & {z:3} & {y:2} & {x:1}
// eliminate a (cycle)   {z:3} & {y:2} & {x:1}
// simplify              {x:1,y:2,z:3}

This rule also applies to field values that are disjunctions of unification
operations of the above form.


Copy code
Copied!

a: b&{x:1} | {y:1}  // {x:1,y:3,z:2} | {y:1}
b: {x:2} | c&{z:2}  // {x:2} | {x:1,y:3,z:2}
c: a&{y:3} | {z:3}  // {x:1,y:3,z:2} | {z:3}
