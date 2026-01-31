---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#80-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 132
summary: In front of a pattern constraint (X=[expr]: value):.  * binds the identifier to the same field as the matched by the pattern
---


In front of a pattern constraint (X=[expr]: value):

 * binds the identifier to the same field as the matched by the pattern
   within the instance of the field value (value).

In front of a pattern constraint expression ([X=expr]: value):

 * binds the identifier to the concrete label that matches expr
   within the instances of the field value (value).

Before a value (foo: X=x)

 * binds the identifier to the value it precedes within the scope of that value.

Before a list element ([ X=value, X+1 ]) (Not yet implemented)
