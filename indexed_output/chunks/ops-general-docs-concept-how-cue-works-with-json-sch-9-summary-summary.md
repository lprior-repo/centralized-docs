---
doc_id: ops/general/docs-concept-how-cue-works-with-json-sch
chunk_id: ops/general/docs-concept-how-cue-works-with-json-sch#9-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 130
summary: $ cue vet -c -d '#Person' schema. name: conflicting values [\"Charlie\",\"Cartwright\"] and strings
---


TERMINAL

Copy code
Copied!

$ cue vet -c -d '#Person' schema.cue good.json bad.json
name: conflicting values ["Charlie","Cartwright"] and strings.MinRunes(1) (mismatched types list and string):
    ./bad.json:2:13
    ./schema.cue:11:9

The cue vet command can also validate the data using the JSON Schema directly:

TERMINAL

Copy code
Copied!

$ cue vet -c schema.json good.json bad.json
name: conflicting values ["Charlie","Cartwright"] and strings.MinRunes(1) (mismatched types list and string):
    ./bad.json:2:13
