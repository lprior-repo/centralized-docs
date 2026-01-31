---
doc_id: ops/general/docs-concept-how-cue-works-with-openapi
chunk_id: ops/general/docs-concept-how-cue-works-with-openapi#5-summary
chunk_level: summary
chunk_type: table
heading: Introduction
token_count: 135
summary: 	kind!: #Kind. 	// Centenarian pets are not handled
---

	kind!: #Kind
	// Centenarian pets are not handled.
	age?: uint & <100 // TODO: increase limit if the tortoise proposal is accepted.
	...
}

// Kind encodes the different pets we handle.
#Kind: "cat" | "dog" | "goldfish"

The cue def [/docs/reference/command/cue-help-def/] command can
express CUE constraints as OpenAPI data schemas, in OpenAPI’s
components.schemas namespace - but only so long as all the top-level
constraints are specified using CUE definitions.

The OpenAPI info.title field is extracted from the top-level CUE comment
