---
doc_id: ops/general/docs-concept-how-cue-works-with-yaml
chunk_id: ops/general/docs-concept-how-cue-works-with-yaml#13-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 128
summary: import \"encoding/yaml\". output: yaml
---

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

