---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#19-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 517
summary: The result of { A } is A for any A (including definitions). Syntactically, embeddings may be any expression
---


The result of { A } is A for any A (including definitions).

Syntactically, embeddings may be any expression.


Copy code
Copied!

S1: {
    a: 1
    b: 2
    {
        c: 3
    }
}
// S1 is { a: 1, b: 2, c: 3 }

S2: close({
    a: 1
    b: 2
    {
        c: 3
    }
})
// same as close(S1)

S3: {
    a: 1
    b: 2
    close({
        c: 3
    })
}
// same as S2


DEFINITIONS AND HIDDEN FIELDS

A field is a definition if its identifier starts with # or _#.
A field is hidden if its identifier starts with a _.
All other fields are regular.

Definitions and hidden fields are not emitted when converting a CUE program
to data and are never required to be concrete.

Referencing a definition will recursively close [/docs/reference/spec/#closed-structs] it.
That is, a referenced definition will not unify with a struct
that would add a field anywhere within the definition that it does not
already define or explicitly allow with a pattern constraint or ....
Embedding [/docs/reference/spec/#embedding] allows bypassing this check.

If referencing a definition would always result in an error, implementations
may report this inconsistency at the point of its declaration.


Copy code
Copied!

#MyStruct: {
    sub: field:    string
}

#MyStruct: {
    sub: enabled?: bool
}

myValue: #MyStruct & {
    sub: feild:   2     // error, feild not defined in #MyStruct
    sub: enabled: true  // okay
}

#D: {
    #OneOf

    c: int // adds this field.
}

#OneOf: { a: int } | { b: int }


D1: #D & { a: 12, c: 22 }  // { a: 12, c: 22 }
D2: #D & { a: 12, b: 33 }  // _|_ // cannot define both `a` and `b`


Copy code
Copied!

#A: {a: int}

B: {
    #A
    b: c: int
}

x: B
x: d: 3  // not allowed, as closed by embedded #A

y: B.b
y: d: 3  // allowed as nothing closes b

#B: {
    #A
    b: c: int
}

z: #B.b
z: d: 3  // not allowed, as referencing #B closes b


ATTRIBUTES

Attributes allow associating meta information with values.
Their primary purpose is to define mappings between CUE and
other representations.
Attributes do not influence the evaluation of CUE.
