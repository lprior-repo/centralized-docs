---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#22-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 142
summary: Default values are CUE’s equivalent of inheritance,. specifically the kind that allows instances to override any value of its parent
---


Default values are CUE’s equivalent of inheritance,
specifically the kind that allows instances to override any value of its parent.
Without it, very little boilerplate removal would be possible.
That is fine if CUE is used just for validation,
but as it aims to be useful across the entire configuration continuum,
it seemed too restrictive to not have such a construct.


RELATION TO INHERITANCE

In CUE, if one sees a concrete value for a field,
it is guaranteed that this will be the final result.
If a value is not concrete (like string), it is clear the search
