---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#30-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 515
summary: After they are evaluated, the parameters of the call are passed by value. to the function and the called function begins execution
---

order.
After they are evaluated, the parameters of the call are passed by value
to the function and the called function begins execution.
The return parameters
of the function are passed by value back to the calling function when the
function returns.

COMPREHENSIONS

Lists and fields can be constructed using comprehensions.

Comprehensions define a clause sequence that consists of a sequence of
for, if, and let clauses, nesting from left to right.
The sequence must start with a for or if clause.
The for and let clauses each define a new scope in which new values are
bound to be available for the next clause.

The for clause binds the defined identifiers, on each iteration, to the next
value of some iterable value in a new scope.
A for clause may bind one or two identifiers.
If there is one identifier, it binds it to the value of
a list element or struct field value.
If there are two identifiers, the first value will be the key or index,
if available, and the second will be the value.

For lists, for iterates over all elements in the list after closing it.
For structs, for iterates over all non-optional regular fields.

An if clause, or guard, specifies an expression that terminates the current
iteration if it evaluates to false.

The let clause binds the result of an expression to the defined identifier
in a new scope.

A current iteration is said to complete if the innermost block of the clause
sequence is reached.
Syntactically, the comprehension value is a struct.
A comprehension can generate non-struct values by embedding such values within
this struct.

Within lists, the values yielded by a comprehension are inserted in the list
at the position of the comprehension.
Within structs, the values yielded by a comprehension are embedded within the
struct.
Both structs and lists may contain multiple comprehensions.


Copy code
Copied!

Comprehension       = Clauses StructLit .

Clauses             = StartClause { [ "," ] Clause } .
StartClause         = ForClause | GuardClause .
Clause              = StartClause | LetClause .
