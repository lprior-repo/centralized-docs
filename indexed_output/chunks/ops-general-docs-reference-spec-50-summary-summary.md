---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#50-summary
chunk_level: summary
chunk_type: table
heading: Introduction
token_count: 141
summary: element in the lattice. bool_lit = \"true\" | \"false\" 
---

element in the lattice.


Copy code
Copied!

bool_lit = "true" | "false" .


Copy code
Copied!

bool & true          true
true & true          true
true & false         _|_
bool & (false|true)  false | true
bool & (true|false)  true | false

NUMERIC VALUES

The integer type represents the set of all integral numbers.
The decimal floating-point type represents the set of all decimal floating-point
numbers.
They are two distinct types.
Both are instances instances of a generic number type.

The predeclared number, integer, and decimal floating-point types are
