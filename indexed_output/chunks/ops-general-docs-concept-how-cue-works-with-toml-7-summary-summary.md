---
doc_id: ops/general/docs-concept-how-cue-works-with-toml
chunk_id: ops/general/docs-concept-how-cue-works-with-toml#7-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 139
summary: PROCESSING AND TRANSFORMING TOML FILES. The cue tool can read and transform TOML files, producing output data in any
---

    ./schema.cue:6:15
    ./config-b.toml:3:14

PROCESSING AND TRANSFORMING TOML FILES

The cue tool can read and transform TOML files, producing output data in any
shape that’s required. For example:

Copied!
transform.cue

Copy code
Copied!

a: int
b: int
c: 1 + a*b

Copied!
data.toml

Copy code
Copied!

a = 5
b = 4

TERMINAL

Copy code
Copied!

$ cue export data.toml transform.cue --out toml
a = 5
b = 4
c = 21

Learn more about transforming data with CUE in these How-to guides:

 * Transforming JSON with CUE [/docs/howto/transform-json-with-cue/]
