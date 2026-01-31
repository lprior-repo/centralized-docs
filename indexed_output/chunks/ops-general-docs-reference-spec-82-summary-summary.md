---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#82-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 132
summary: in which it is declared. The identifier must be unique within its scope
---

in which it is declared.
The identifier must be unique within its scope.


Copy code
Copied!

let x = expr

a: x + 1
b: x + 2


SHORTHAND NOTATION FOR NESTED STRUCTS

A field whose value is a struct with a single field may be written as
a colon-separated sequence of the two field names,
followed by a colon and the value of that single field.


Copy code
Copied!

job: myTask: replicas: 2

expands to


Copy code
Copied!

job: {
    myTask: {
        replicas: 2
    }
}

LISTS

A list literal defines a new value of type list.
