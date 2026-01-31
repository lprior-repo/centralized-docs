---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#78-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 150
summary: // Package attribute. @protobuf(proto3)
---



Copy code
Copied!

// Package attribute
@protobuf(proto3)

myStruct1: {
    // Struct attribute:
    @jsonschema(id="https://example.org/mystruct1.json")

    // Field attributes
    field: string @go(Field)
    attr:  int    @xml(,attr) @go(Attr)
}

myStruct2: {
    field: string @go(Field)
    attr:  int    @xml(a1,attr) @go(Attr)
}

Combined: myStruct1 & myStruct2
// field: string @go(Field)
// attr:  int    @xml(,attr) @xml(a1,attr) @go(Attr)


ALIASES

Aliases name values that can be referred to
within the scope [/docs/reference/spec/#declarations-and-scopes] in which they are declared.
