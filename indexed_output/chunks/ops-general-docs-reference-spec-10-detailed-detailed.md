---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#10-detailed
chunk_level: detailed
chunk_type: prose
heading: Introduction
token_count: 1034
summary: z: d: 3  // not allowed, as referencing #B closes b. Attributes allow associating meta information with values
---

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

In front of a pattern constraint expression ([X=expr]: value):

 * binds the identifier to the concrete label that matches expr
   within the instances of the field value (value).

Before a value (foo: X=x)

 * binds the identifier to the value it precedes within the scope of that value.

Before a list element ([ X=value, X+1 ]) (Not yet implemented)

 * binds the identifier to the list element it precedes within the scope of the
   list expression.


Copy code
Copied!

// A field alias
foo: X  // 4
X="not an identifier": 4

// A value alias
foo: X={x: X.a}
bar: foo & {a: 1}  // {a: 1, x: 1}

// A label alias
[Y=string]: { name: Y }
foo: { value: 1 } // outputs: foo: { name: "foo", value: 1 }


LET DECLARATIONS

Let declarations bind an identifier to an expression.
The identifier is only visible within the scope [/docs/reference/spec/#declarations-and-scopes]
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
A list may be open or closed.
An open list is indicated with a ... at the end of an element list,
optionally followed by a value for the remaining elements.

The length of a closed list is the number of elements it contains.
The length of an open list is the number of elements as a lower bound
and an unlimited number of elements as its upper bound.
