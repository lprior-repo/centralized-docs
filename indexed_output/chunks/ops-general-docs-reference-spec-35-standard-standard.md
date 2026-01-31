---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#35-standard
chunk_level: standard
chunk_type: table
heading: Introduction
token_count: 521
summary: and take v as the result of unification. Configuration    Evaluated
---

and take v as the result of unification.


Copy code
Copied!

Configuration    Evaluated
//    c           Cycles in nodes of type struct evaluate
//  ↙︎   ↖         to the fixed point of unifying their
// a  →  b        values ad infinitum.

a: b & { x: 1 }   // a: { x: 1, y: 2, z: 3 }
b: c & { y: 2 }   // b: { x: 1, y: 2, z: 3 }
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
a: b: a

// infinite evaluation
f: {
    n:   int
    out: n + (f & {n: 1}).out
}

CUE must allow or disallow structural cycles under certain circumstances.
