---
doc_id: ops/general/docs-concept-how-cue-works-with-json-sch
chunk_id: ops/general/docs-concept-how-cue-works-with-json-sch#8-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 130
summary: 	\"home phone\"?: string @deprecated(). We use the imported schema to validate some known-good data (good
---

	"home phone"?: string @deprecated()
	...
}

We use the imported schema to validate some known-good data (good.json)
and known-bad data (bad.json):

Copied!
good.json

Copy code
Copied!

{
    "name": "Dorothy Cartwright",
    "address": "Ripon, North Yorkshire"
}

Copied!
bad.json

Copy code
Copied!

{
    "name": [
        "Charlie",
        "Cartwright"
    ],
    "address": "Ripon, North Yorkshire"
}

The
cue vet [/docs/reference/command/cue-help-vet/]
command validates our data against the #Person constraint:
