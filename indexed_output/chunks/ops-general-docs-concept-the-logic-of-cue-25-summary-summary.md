---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#25-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 141
summary: So there is a clear benefit to having fully expanded configurations. over such override methods
---


So there is a clear benefit to having fully expanded configurations
over such override methods.
CUE simulates that benefit by guaranteeing that any observed field value
holds for the final result.

If the user makes the false assumption that no concrete value is specified to discard the default value,
CUE will catch an erroneous change to that value and report the conflicting
locations.

But there is more.
In CUE one can apply a constraint to a group of values at once,
even across files.
Once set, there is no need to look at the individual values and files to
