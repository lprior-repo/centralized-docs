---
doc_id: ops/general/docs-concept-how-cue-works-with-go
chunk_id: ops/general/docs-concept-how-cue-works-with-go#10-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 129
summary: Finally, running the Go program displays the CUE data:. 	l: [1, 2, 3]
---

Finally, running the Go program displays the CUE data:

TERMINAL

Copy code
Copied!

$ go run .
{
	l: [1, 2, 3]
	v: "hello"
	message: {
		hello: "world!"
	}
}

CUE values have a default formatter that renders them sensibly.

LOADING NON-CUE DATA

The API also makes it easy to validate data held in YAML and JSON files.

This example loads a CUE schema that’s embedded in code,
then a YAML data file,
and then validates the data against the schema.

Copied!
main.go

Copy code
Copied!

package main

import (
	"fmt"
