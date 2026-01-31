---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#69-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 136
summary: This is makes it easy to add fields, but can lead to bugs:.     field1: string
---

This is makes it easy to add fields, but can lead to bugs:


Copy code
Copied!

S: {
    field1: string
}

S1: S & { field2: "foo" }

// S1 is { field1: string, field2: "foo" }


A: {
    field1: string
    field2: string
}

A1: A & {
    feild1: "foo"  // "field1" was accidentally misspelled
}

// A1 is
//    { field1: string, field2: string, feild1: "foo" }
// not the intended
//    { field1: "foo", field2: string }

A closed struct c is a struct whose instances may not declare any field
with a name that does not match the name of a field
