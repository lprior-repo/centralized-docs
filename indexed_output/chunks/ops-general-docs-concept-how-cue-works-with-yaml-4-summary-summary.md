---
doc_id: ops/general/docs-concept-how-cue-works-with-yaml
chunk_id: ops/general/docs-concept-how-cue-works-with-yaml#4-summary
chunk_level: summary
chunk_type: table
heading: Introduction
token_count: 131
summary: “pre-flight” checks to existing processes with CUE. In this example,
---

“pre-flight” checks to existing processes with CUE.

In this example,
cue vet [/docs/reference/command/cue-help-vet/]
is used to check that a hypothetical system’s YAML input files are valid - and
catches a problematic deployment early in the process:

Copied!
schema.cue

Copy code
Copied!

import "strings"

#Config: {
	cluster!:    strings.MaxRunes(16)
	region!:     #Region
	repository!: =~#"^source\.company\.example/"#
	tags?: [...#Tags]
}
#Region: "APAC" | "IMEA"
#Tags:   "prod" | "stage" | "qa" | "test" | "dev"
