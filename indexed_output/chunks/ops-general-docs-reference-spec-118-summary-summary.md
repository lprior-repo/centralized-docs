---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#118-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 128
summary: at the position of the comprehension. Within structs, the values yielded by a comprehension are embedded within the
---

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
ForClause           = "for" identifier [ "," identifier ] "in" Expression .
GuardClause         = "if" Expression .
