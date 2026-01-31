---
doc_id: ops/general/docs-concept-how-cue-works-with-yaml
chunk_id: ops/general/docs-concept-how-cue-works-with-yaml#3-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 513
summary: GENERATING ENCODED YAML. In this example a Kubernetes ConfigMap contains a YAML file encoded as a
---


GENERATING ENCODED YAML

In this example a Kubernetes ConfigMap contains a YAML file encoded as a
single string field, in a JSON file. This is enabled by the
yaml.Marshal function:

Copied!
config.cue

Copy code
Copied!

import "encoding/yaml"

configMap: data: "point.yaml": yaml.Marshal({
	x: 1.2
	y: 3.45
})

TERMINAL

Copy code
Copied!

$ cue export config.cue --out json
{
    "configMap": {
        "data": {
            "point.yaml": "x: 1.2\n\"y\": 3.45\n"
        }
    }
}

PARSING ENCODED YAML

The yaml.Unmarshal function performs the reverse operation to yaml.Marshal:
it turns a string containing YAML into the structure represented by
the encoded data.

Here, some encoded YAML data is emitted as JSON:

Copied!
file.cue

Copy code
Copied!

import "encoding/yaml"

output: yaml.Unmarshal(data)
data: """
	  user: charlie
	  name: Charlie Cartwright
	  colour: orange
	  species: cat
	  address:
	    - 42 Lancashire Road
	    - Ripon
	    - North Yorkshire
	"""

TERMINAL

Copy code
Copied!

$ cue export file.cue -e output
{
    "user": "charlie",
    "name": "Charlie Cartwright",
    "colour": "orange",
    "species": "cat",
    "address": [
        "42 Lancashire Road",
        "Ripon",
        "North Yorkshire"
    ]
}

VALIDATING ENCODED YAML

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
item: bed: """
	width: 2
	height: 0.1
	depth: 2
	"""
// table's width is incorrectly specified as a string.
item: table: """
	width: "34"
	height: 23
	depth: 0.2
	"""
// painting's height field name is incorrectly upper-cased.
item: painting: """
