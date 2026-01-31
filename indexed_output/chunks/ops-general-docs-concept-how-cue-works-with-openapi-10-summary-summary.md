---
doc_id: ops/general/docs-concept-how-cue-works-with-openapi
chunk_id: ops/general/docs-concept-how-cue-works-with-openapi#10-summary
chunk_level: summary
chunk_type: table
heading: Introduction
token_count: 134
summary: 	version: *\"v1. 3\" | string
---

	version: *"v1.2.3" | string
}
// Kind encodes the different pets we handle.
#Kind: "cat" | "dog" | "goldfish"

// A Pet is a pet that we handle.
#Pet: {
	// A pet has a name.
	name!: string
	kind!: #Kind

	// Centenarian pets are not handled.
	age?: int & >=0 & <100
	...
}

The cue vet [/docs/reference/command/cue-help-vet/] command can
directly use constraints from OpenAPI data schemas to validate data.

Let’s check the details of a well-known animal,
Jonathan the tortoise [https://en.wikipedia.org/wiki/Jonathan_%28tortoise%29]:
