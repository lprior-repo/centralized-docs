---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#79-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 146
summary: The name of an alias must be unique within its scope. AliasExpr  = [ identifier \"=\" ] Expression 
---

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
