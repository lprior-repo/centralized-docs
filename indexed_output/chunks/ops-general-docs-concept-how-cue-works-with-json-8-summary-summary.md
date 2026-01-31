---
doc_id: ops/general/docs-concept-how-cue-works-with-json
chunk_id: ops/general/docs-concept-how-cue-works-with-json#8-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 129
summary: shape that’s required.  For example:
---

shape that’s required. For example:

Copied!
transform.cue

Copy code
Copied!

a: int
b: int
c: 1 + a*b

Copied!
data.json

Copy code
Copied!

{
    "a": 5,
    "b": 4
}

TERMINAL

Copy code
Copied!

$ cue export data.json transform.cue
{
    "a": 5,
    "b": 4,
    "c": 21
}

Learn more about transforming data with CUE in these How-to guides:

 * Transforming JSON with CUE [/docs/howto/transform-json-with-cue/]
 * Combining multiple JSON files into a list [/docs/howto/combine-multiple-json-files-into-a-list/]
