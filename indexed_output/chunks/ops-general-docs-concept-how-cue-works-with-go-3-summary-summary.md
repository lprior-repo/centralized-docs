---
doc_id: ops/general/docs-concept-how-cue-works-with-go
chunk_id: ops/general/docs-concept-how-cue-works-with-go#3-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 143
summary: CUE schemas.  We’ll also use the API to convert both CUE and non-CUE data to
---

CUE schemas. We’ll also use the API to convert both CUE and non-CUE data to
native Go values, and validate some Go data natively with CUE.

CONVERTING GO TYPES TO CUE

If you’ve already invested time in developing Go types, you might need them to
be the source of truth in your system whilst also wanting to validate data that
matches those types against the more detailed constraints that CUE allows.

The cue command can help you achieve this as it can convert arbitrary Go types to CUE.
To demonstrate this, we’re going to fetch some Go source code published by the
