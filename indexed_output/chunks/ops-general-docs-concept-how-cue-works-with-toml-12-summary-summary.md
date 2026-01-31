---
doc_id: ops/general/docs-concept-how-cue-works-with-toml
chunk_id: ops/general/docs-concept-how-cue-works-with-toml#12-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 135
summary: Marshal function:. import \"encoding/toml\"
---

toml.Marshal function:

Copied!
config.cue

Copy code
Copied!

import "encoding/toml"

configMap: data: "point.toml": toml.Marshal({
	x: 1.2
	y: 3.45
})

TERMINAL

Copy code
Copied!

$ cue export config.cue --out yaml
configMap:
  data:
    point.toml: |
      x = 1.2
      y = 3.45

PARSING ENCODED TOML

The toml.Unmarshal function performs the reverse operation to toml.Marshal:
it turns a string containing TOML into the structure represented by
the encoded data.

Here, some encoded TOML data (a Rust crate manifest) is emitted as YAML:
