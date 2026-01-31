---
doc_id: ops/general/docs-concept-how-cue-works-with-json-sch
chunk_id: ops/general/docs-concept-how-cue-works-with-json-sch#15-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 130
summary: JSON schema in the first argument - printing the data if it’s valid and. displaying a validation error otherwise
---

JSON schema in the first argument - printing the data if it’s valid and
displaying a validation error otherwise. Here we use it to validate the same
good.json and bad.json files from above:

TERMINAL

Copy code
Copied!

$ go run . schema.json good.json
{
	name:    "Dorothy Cartwright"
	address: "Ripon, North Yorkshire"
}
$ go run . schema.json bad.json
name: conflicting values ["Charlie","Cartwright"] and strings.MinRunes(1) (mismatched types list and string):
    bad.json:2:13
    schema.json:13:13
exit status 1
