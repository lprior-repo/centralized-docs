---
doc_id: ops/general/docs-concept-how-cue-works-with-toml
chunk_id: ops/general/docs-concept-how-cue-works-with-toml#13-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 147
summary: import \"encoding/toml\". _cargo: \"\"\"
---


Copied!
config.cue

Copy code
Copied!

import "encoding/toml"

_cargo: """
	[package]
	name = "hello_world"
	version = "0.1.0"
	edition = "2021"

	[dependencies]
	serde = "1.0"
	"""
output: cargo: toml.Unmarshal(_cargo)

TERMINAL

Copy code
Copied!

$ cue export config.cue --out yaml
output:
  cargo:
    package:
      name: hello_world
      version: 0.1.0
      edition: "2021"
    dependencies:
      serde: "1.0"

CONVERTING TOML FILES TO CUE

The cue import [/docs/reference/command/cue-help-import/]
command can create a CUE file for each TOML file it’s given – and can even
