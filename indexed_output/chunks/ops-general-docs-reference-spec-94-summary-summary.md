---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#94-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 140
summary: or if they appear in different packages and are not exported. Otherwise, they are the same
---

or if they appear in different packages and are not exported.
Otherwise, they are the same.

FIELD DECLARATIONS

A field associates the value of an expression to a label within a struct.
If this label is an identifier, it binds the field to that identifier,
so the field’s value can be referenced by writing the identifier.
String labels are not bound to fields.


Copy code
Copied!

a: {
    b: 2
    "s": 3

    c: b   // 2
    d: s   // _|_ unresolved identifier "s"
    e: a.s // 3
}

If an expression may result in a value associated with a default value
