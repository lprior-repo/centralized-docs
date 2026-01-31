---
doc_id: ops/general/docs-concept-how-cue-works-with-go
chunk_id: ops/general/docs-concept-how-cue-works-with-go#13-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 128
summary: or from some remote service over the network. This time we place our schema in a separate CUE file:
---

or from some remote service over the network.

This time we place our schema in a separate CUE file:

Copied!
schema.cue

Copy code
Copied!

package example

#Person: {
	name?: string
	age?:  int & <=150
}

We embed the schema file using Go embedding [https://pkg.go.dev/embed],
and load it via a string.
Then we use the #Person schema to validate a Go Person,
either logging a fatal error or reporting a successful validation.

Copied!
main.go

Copy code
Copied!

package main

import (
	_ "embed"
	"fmt"
	"log"
