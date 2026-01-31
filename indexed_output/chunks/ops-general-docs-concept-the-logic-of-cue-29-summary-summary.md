---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#29-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 128
summary: CUE wants no part of these shenanigans. So the answer in this case is that there are no concrete values
---

CUE wants no part of these shenanigans.
So the answer in this case is that there are no concrete values
as the defaults cannot be used.

The model for this is actually quite simple.
Conceptually, CUE keeps two parallel values, one for all possible values
and one for the default, which must be an instance of the former.
Roughly speaking, for the example with the conflict,
it simultaneously evaluates:


Copy code
Copied!

// All allowed values
a: int
b: int
a: b
b: a


Copy code
Copied!

// Default
a: 1
b: 2
