---
doc_id: ops/general/docs-concept-how-cue-works-with-json
chunk_id: ops/general/docs-concept-how-cue-works-with-json#16-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 169
summary: // painting's height field name is incorrectly upper-cased. item: painting: #\"{ \"width\": 34, \"HEIGHT\": 12, \"depth\": 0
---

// painting's height field name is incorrectly upper-cased.
item: painting: #"{ "width": 34, "HEIGHT": 12, "depth": 0.2 }"#

TERMINAL

Copy code
Copied!

$ cue vet -c furniture.cue
item.painting: invalid value "{ \"width\": 34, \"HEIGHT\": 12, \"depth\": 0.2 }" (does not satisfy encoding/json.Validate): error in call to encoding/json.Validate: field not allowed:
    ./furniture.cue:10:17
    ./furniture.cue:17:17
    json.Validate:1:16
item.table: invalid value "{ \"width\": \"34\", \"height\": 23, \"depth\": 0.2 }" (does not satisfy encoding/json.Validate): error in call to encoding/json.Validate: conflicting values "34" and number (mismatched types string and number):
