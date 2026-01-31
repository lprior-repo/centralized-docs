---
doc_id: ops/general/docs-concept-how-cue-works-with-yaml
chunk_id: ops/general/docs-concept-how-cue-works-with-yaml#15-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 128
summary: item: bed: \"\"\". // table's width is incorrectly specified as a string
---

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
	width: 34
	HEIGHT: 12
	depth: 0.2
	"""

TERMINAL

Copy code
Copied!

$ cue vet -c furniture.cue
item.painting: invalid value "width: 34\nHEIGHT: 12\ndepth: 0.2" (does not satisfy encoding/yaml.Validate): error in call to encoding/yaml.Validate: field not allowed:
