---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#34-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 525
summary: If the condition unifies successfully, the “then” schema is applied;. otherwise, the “else” schema is applied
---

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
