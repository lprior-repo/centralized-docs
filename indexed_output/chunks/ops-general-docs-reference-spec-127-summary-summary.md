---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#127-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 132
summary: their first argument. The remainder of this section defines builtin validators
---

their first argument.

The remainder of this section defines builtin validators. These can only be
used as validators, so we will not refer to their function equivalents.

These builtins refer to finalized values, which means that the value being
validated is fully resolved, and defaults taken, before it is unified with the
schema.

MATCHN

The matchN builtin is a validator that checks if a specified number of schemas
from a given list unify successfully with the finalized value being validated.

matchN takes two arguments:
