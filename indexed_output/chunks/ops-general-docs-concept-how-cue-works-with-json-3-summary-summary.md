---
doc_id: ops/general/docs-concept-how-cue-works-with-json
chunk_id: ops/general/docs-concept-how-cue-works-with-json#3-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 142
summary:     \"c\": \"three\",. $ cue export data
---

   data.cue

Copy code
Copied!

{
    "a": 1,
    "b": "2",
    "c": "three",
    "d": 4.4
}


Copy code
Copied!

e: 5
f: "6"


Copy code
Copied!

g: "seven"
h: 4.4 * 2

TERMINAL

Copy code
Copied!

$ cue export data.json data.yml data.cue
{
    "a": 1,
    "b": "2",
    "c": "three",
    "d": 4.4,
    "e": 5,
    "f": "6",
    "g": "seven",
    "h": 8.8
}

In addition to JSON, cue can read and write
a range of other formats [/docs/integration/].

VALIDATING JSON FILES AGAINST A SCHEMA

CUE is often used to make systems safer without having to teach the underlying
