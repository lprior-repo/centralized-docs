---
doc_id: ops/general/docs-concept-how-cue-works-with-json-sch
chunk_id: ops/general/docs-concept-how-cue-works-with-json-sch#11-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 129
summary: USING JSON SCHEMA WITH THE GO API. encoding/jsonschema [https://pkg
---

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

Copied!
main.go

Copy code
Copied!

package main
