---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#20-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 523
summary: An attribute associates an identifier with a value, a balanced token sequence,. which is a sequence of CUE tokens with balanced brackets ((), [], and {})
---


An attribute associates an identifier with a value, a balanced token sequence,
which is a sequence of CUE tokens with balanced brackets ((), [], and {}).
The sequence may not contain interpolations.

Fields, structs and packages can be associated with a set of attributes.
Attributes accumulate during unification, but implementations may remove
duplicates that have the same source string representation.
The interpretation of an attribute, including the handling of multiple
attributes for a given identifier, is up to the consumer of the attribute.

Field attributes define additional information about a field,
such as a mapping to a protocol buffer tag or alternative
name of the field when mapping to a different language.


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
The name of an alias must be unique within its scope.


Copy code
Copied!

AliasExpr  = [ identifier "=" ] Expression .

Aliases can appear in several positions:

In front of a Label (X=label: value):

 * binds the identifier to the same value as label would be bound
   to if it were a valid identifier.

In front of a dynamic field (X=(label): value):

 * binds the identifier to the same value as label if it were a valid
   static identifier.

In front of a dynamic field expression ((X=expr): value):

 * binds the identifier to the concrete label resulting from evaluating expr.

In front of a pattern constraint (X=[expr]: value):

 * binds the identifier to the same field as the matched by the pattern
   within the instance of the field value (value).
