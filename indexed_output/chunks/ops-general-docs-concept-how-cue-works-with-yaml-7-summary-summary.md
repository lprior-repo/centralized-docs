---
doc_id: ops/general/docs-concept-how-cue-works-with-yaml
chunk_id: ops/general/docs-concept-how-cue-works-with-yaml#7-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 128
summary: Learn more in the How-to guide Validating YAML using CUE [/docs/howto/validate-yaml-using-cue/]. PROCESSING AND TRANSFORMING YAML FILES
---

    ./schema.cue:6:15
    ./config-b.yaml:2:13

Learn more in the How-to guide Validating YAML using CUE [/docs/howto/validate-yaml-using-cue/].

PROCESSING AND TRANSFORMING YAML FILES

The cue tool can read and transform YAML files, producing output data in any
shape that’s required. For example:

Copied!
transform.cue

Copy code
Copied!

a: int
b: int
c: 1 + a*b

Copied!
data.yaml

Copy code
Copied!

a: 5
b: 4

TERMINAL

Copy code
Copied!

$ cue export --out yaml data.yaml transform.cue
a: 5
b: 4
c: 21
