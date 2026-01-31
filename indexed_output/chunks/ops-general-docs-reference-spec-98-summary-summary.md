---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#98-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 131
summary: Sin    // denotes the Sin function in package math. An identifier operand refers to a field and is called a reference
---



Copy code
Copied!

math.Sin    // denotes the Sin function in package math

REFERENCES

An identifier operand refers to a field and is called a reference.
The value of a reference is a copy of the expression associated with the field
that it is bound to,
with any references within that expression bound to the respective copies of
the fields they were originally bound to.
Implementations may use a different mechanism to evaluate as long as
these semantics are maintained.


Copy code
Copied!

a: {
    place:    string
