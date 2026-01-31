---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#121-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 140
summary:  * list: illegal.  * struct: illegal
---

 * list: illegal
 * struct: illegal


Copy code
Copied!

a: "World"
b: "Hello \( a )!" // Hello World!

BUILTIN FUNCTIONS

Builtin functions are predeclared. They are called like any other function.

ERROR

The error builtin allows users to create custom error values with a specified
message.
User-generated errors can be included in disjunctions; if at least one disjunct
is valid, any user errors are ignored.
However, if all disjuncts fail, all user error messages are reported together.

error takes a single string argument. If this argument is a literal
