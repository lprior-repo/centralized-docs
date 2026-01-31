---
doc_id: ops/general/docs-concept-how-cue-works-with-yaml
chunk_id: ops/general/docs-concept-how-cue-works-with-yaml#14-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 134
summary: Validate and yaml. ValidatePartial functions allow encoded YAML to
---

The yaml.Validate and yaml.ValidatePartial functions allow encoded YAML to
be validated against native CUE schema constraints.

Here, each member of the item map is checked against the #Dimensions
schema. The cue tool correctly catches and flags up two problems with the
data:

Copied!
furniture.cue

Copy code
Copied!

import "encoding/yaml"

#Dimensions: {
	width:  number
	depth:  number
	height: number
}

// Validate each member of the map against a schema.
item: [string]: yaml.Validate(#Dimensions)

// bed is correctly specified.
