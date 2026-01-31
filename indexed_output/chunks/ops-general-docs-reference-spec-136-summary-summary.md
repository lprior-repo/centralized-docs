---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#136-summary
chunk_level: summary
chunk_type: table
heading: Introduction
token_count: 134
summary: // resolving a           b&{x:1} | {y:1}. // substitute b          ({x:2} | c&{z:2})&{x:1} | {y:1}
---



// resolving a           b&{x:1} | {y:1}
// substitute b          ({x:2} | c&{z:2})&{x:1} | {y:1}
// simplify              c&{z:2}&{x:1} | {y:1}
// substitute c          (a&{y:3} | {z:3})&{z:2}&{x:1} | {y:1}
// simplify              a&{y:3}&{z:2}&{x:1} | {y:1}
// eliminate a (cycle)   {y:3}&{z:2}&{x:1} | {y:1}
// expand                {x:1,y:3,z:2} | {y:1}

Note that all nodes that form a reference cycle to form a struct will evaluate
to the same value.
If a field value is a disjunction, any element that is part of a cycle will
