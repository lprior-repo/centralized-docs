---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#17-detailed
chunk_level: detailed
chunk_type: table
heading: Introduction
token_count: 1024
summary: becomes available. The matchIf builtin is a conditional validator that applies different schema
---

becomes available.

MATCHIF

The matchIf builtin is a conditional validator that applies different schema
constraints based on whether an initial condition is satisfied.

matchIf takes three arguments:

 * a condition schema (the “if” clause),
 * the schema to apply if the condition matches (the “then” clause),
 * the schema to apply if the condition does not match (the “else” clause).

The validator first attempts to unify the finalized value with the condition
schema.
If the condition unifies successfully, the “then” schema is applied;
otherwise, the “else” schema is applied.
The validator succeeds if the chosen schema unifies successfully with the value.


Copy code
Copied!

// If value is a string, it must have length > 3; otherwise it must be > 10
value: "hello" & matchIf(string, len(value) > 3, value > 10)  // true

// If value matches {a: int}, it must have b field; otherwise a must be a string
x: {a: 1} & matchIf(x, {a: int}, {a: int, b: int}, {a: string})  // false: missing b

// If value is >5, it must be <10; otherwise it must be <3
y: 2 & matchIf(y, >5, <10, <3)  // true: 2 is <=5, so <3 is checked

CYCLES

Implementations are required to interpret or reject cycles encountered
during evaluation according to the rules in this section.

REFERENCE CYCLES

A reference cycle occurs if a field references itself, either directly or
indirectly.


Copy code
Copied!

// x references itself
x: x

// indirect cycles
b: c
c: d
d: b

Implementations should treat these as _.
Two particular cases are discussed below.


EXPRESSIONS THAT UNIFY AN ATOM WITH AN EXPRESSION

An expression of the form a & e, where a is an atom
and e is an expression, always evaluates to a or bottom.
As it does not matter how we fail, we can assume the result to be a
and postpone validating a == e until after all references
in e have been resolved.


Copy code
Copied!

// Config            Evaluates to (requiring concrete values)
x: {                  x: {
    a: b + 100            a: _|_ // cycle detected
    b: a - 100            b: _|_ // cycle detected
}                     }

y: x & {              y: {
    a: 200                a: 200 // asserted that 200 == b + 100
                          b: 100
}                     }


FIELD VALUES

A field value of the form r & v,
where r evaluates to a reference cycle and v is a concrete value,
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
