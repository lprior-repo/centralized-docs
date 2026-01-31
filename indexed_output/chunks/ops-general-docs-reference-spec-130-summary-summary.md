---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#130-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 145
summary:  * a condition schema (the “if” clause),.  * the schema to apply if the condition matches (the “then” clause),
---


 * a condition schema (the “if” clause),
 * the schema to apply if the condition matches (the “then” clause),
 * the schema to apply if the condition does not match (the “else” clause).

The validator first attempts to unify the finalized value with the condition
schema.
If the condition unifies successfully, the “then” schema is applied;
otherwise, the “else” schema is applied.
The validator succeeds if the chosen schema unifies successfully with the value.


Copy code
Copied!

// If value is a string, it must have length > 3; otherwise it must be > 10
