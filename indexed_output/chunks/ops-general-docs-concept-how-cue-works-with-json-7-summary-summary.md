---
doc_id: ops/general/docs-concept-how-cue-works-with-json
chunk_id: ops/general/docs-concept-how-cue-works-with-json#7-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 140
summary: cluster: invalid value \"live03333333333333\" (does not satisfy strings. MaxRunes(16)):
---

    ./schema.cue:9:19
cluster: invalid value "live03333333333333" (does not satisfy strings.MaxRunes(16)):
    ./schema.cue:4:15
    ./config-b.json:2:16
    ./schema.cue:4:32
repository: invalid value "github.com/Alex_Personal_Account/alpha-fork" (out of bound =~"^source\\.company\\.example/"):
    ./schema.cue:6:15
    ./config-b.json:3:19

Learn more in the How-to guide Validating JSON using CUE [/docs/howto/validate-json-using-cue/].

PROCESSING AND TRANSFORMING JSON FILES

The cue tool can read and transform JSON files, producing output data in any
