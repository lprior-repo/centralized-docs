---
doc_id: ops/general/docs-concept-how-cue-works-with-openapi
chunk_id: ops/general/docs-concept-how-cue-works-with-openapi#9-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 134
summary: The cue import [/docs/reference/command/cue-help-import/] command. can take constraints found in OpenAPI data schemas and express them as CUE
---


The cue import [/docs/reference/command/cue-help-import/] command
can take constraints found in OpenAPI data schemas and express them as CUE.
Here we take the definition produced by cue def, above, and convert it back to CUE:

TERMINAL

Copy code
Copied!

$ cue import -p api api.pet.yaml

This produces the following CUE, which is as close to the original schema.cue
as OpenAPI currently permits:

Copied!
api.pet.cue

Copy code
Copied!

// A schema for the pet API.
package api

info: {
	title:   *"A schema for the pet API." | string
