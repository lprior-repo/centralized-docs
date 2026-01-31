---
doc_id: ops/general/docs-concept-how-cue-works-with-yaml
chunk_id: ops/general/docs-concept-how-cue-works-with-yaml#16-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 142
summary: Validate:2:1. table: invalid value \"width: \\"34\\"\nheight: 23\ndepth: 0
---

    ./furniture.cue:10:17
    ./furniture.cue:25:17
    yaml.Validate:2:1
item.table: invalid value "width: \"34\"\nheight: 23\ndepth: 0.2" (does not satisfy encoding/yaml.Validate): error in call to encoding/yaml.Validate: conflicting values "34" and number (mismatched types string and number):
    ./furniture.cue:10:17
    ./furniture.cue:4:10
    ./furniture.cue:19:14
    yaml.Validate:1:8

OTHER YAML FUNCTIONS

The
yaml package [https://pkg.go.dev/cuelang.org/go/pkg/encoding/yaml]
contains other useful functions which are demonstrated in guides that you can
