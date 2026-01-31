---
doc_id: ops/general/docs-concept-how-cue-works-with-json-sch
chunk_id: ops/general/docs-concept-how-cue-works-with-json-sch#2-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 516
summary: 	children?: [. 	\"home phone\"?: string @deprecated()
---

	children?: [...string]
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
    ./schema.json:13:13

The cue command normally recognises JSON Schema’s signature fields and treats
the contents of JSON Schema as data constraints - not just additional data.
A qualifier can be used to change this behaviour, as outlined in
cue help filetypes [/docs/reference/command/cue-help-filetypes/]:

TERMINAL

Copy code
Copied!

$ cue def json: schema.json
$schema:     "https://json-schema.org/draft/2020-12/schema"
type:        "object"
title:       "Main Person schema."
description: "This schema defines a person."
...

USING JSON SCHEMA WITH THE GO API

The
encoding/jsonschema [https://pkg.go.dev/cuelang.org/go/encoding/jsonschema]
API
allows you to work with JSON Schema in Go code.

As with the cue command examples shown above, the API can be used to convert
JSON Schema to CUE. However, in this next example, we’ll use the API in a more
fully-formed context: controlling data validation at a lower level.

This Go program validates a JSON data file against a JSON Schema:
