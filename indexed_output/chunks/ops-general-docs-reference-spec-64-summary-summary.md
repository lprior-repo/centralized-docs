---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#64-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 138
summary: (a): \"baz\"                             foo: \"baz\". (a+b): \"qux\"                           foobar: \"qux\"
---

(a): "baz"                             foo: "baz"

(a+b): "qux"                           foobar: "qux"

(a)?: string                           foo?: string
(b)!: string                           bar!: string


PATTERN AND DEFAULT CONSTRAINTS

A struct may define constraints that apply to a collection of fields.

A pattern constraint, denoted [pattern]: value, defines a pattern, which
is a value of type string, and a value to unify with fields whose label
unifies with the pattern.
For a given struct a with pattern constraint [p]: v, v is unified
