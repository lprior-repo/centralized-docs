---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#21-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 519
summary: In front of a pattern constraint expression ([X=expr]: value):.  * binds the identifier to the concrete label that matches expr
---


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


Copy code
Copied!

ListLit       = "[" [ ElementList [ "," ] ] "]" .
ElementList   = Ellipsis | Embedding { "," Embedding } [ "," Ellipsis ] .

Lists can be thought of as structs:


Copy code
Copied!

List: *null | {
    Elem: _
    Tail: List
}

For closed lists, Tail is null for the last element, for open lists it is
