---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#15-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 128
summary: software engineering: backwards compatibility. For a newer version of an API to be backwards compatible with the previous
---

software engineering: backwards compatibility.
For a newer version of an API to be backwards compatible with the previous
version it must subsume it.
In other words, the old version must be an instance of the new one.
Or yet another way to say it: a new version may not forbid what was allowed
in the older version.

With optional fields it gets a bit more subtle, but basically,
an instance may change an optional field to required, but not remove it.
The backwards compatibility metaphor applies here as well.
