---
doc_id: ops/general/docs-concept-how-cue-works-with-openapi
chunk_id: ops/general/docs-concept-how-cue-works-with-openapi#4-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 129
summary:    encoding/openapi [https://pkg. dev/cuelang
---

 * and the
   encoding/openapi [https://pkg.go.dev/cuelang.org/go/encoding/openapi]
   API generating OpenAPI in Go code.

USING OPENAPI WITH THE CUE COMMAND

The cue command can use OpenAPI constraints in various ways, through its
different subcommands.

Let’s start with a simple CUE schema:

Copied!
schema.cue

Copy code
Copied!

// A schema for the pet API.
package api

$version: "v1.2.3"
// A Pet is a pet that we handle.
#Pet: {
	// A pet has a name.
	name!: string
	// We only handle certain kinds of pets.
