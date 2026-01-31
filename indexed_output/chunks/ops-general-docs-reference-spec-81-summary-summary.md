---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#81-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 129
summary:  * binds the identifier to the list element it precedes within the scope of the.    list expression
---


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
