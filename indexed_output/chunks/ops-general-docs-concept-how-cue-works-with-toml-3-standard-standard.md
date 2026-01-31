---
doc_id: ops/general/docs-concept-how-cue-works-with-toml
chunk_id: ops/general/docs-concept-how-cue-works-with-toml#3-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 517
summary: containing functions that generate and parse TOML from within CUE, as shown here. GENERATING ENCODED TOML
---

containing functions that generate and parse TOML from within CUE, as shown here.

GENERATING ENCODED TOML

In this example a Kubernetes ConfigMap contains a TOML file encoded as a
single string field, in a YAML document. This is enabled by the
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
recognise encoded YAML and JSON fields, and convert those structures
recursively.

Examples of this command being used can be found in the
cue import [/docs/reference/command/cue-help-import/]
reference documentation.

RELATED CONTENT

 * Concept Guide: How CUE works with YAML [/docs/concept/how-cue-works-with-yaml/]
 * Concept Guide: How CUE works with JSON [/docs/concept/how-cue-works-with-json/]

Last modified November 6, 2025 [https://github.com/cue-lang/cuelang.org/commit/c4d2e727a59f5158a2be4946992d96c4612a9b88]

 * encodings [/search?q=tag:encodings]
 * cue command [/search?q=tag:%22cue%20command%22]


Open share options
 * 
   
   Copy link
   Copied!

 * Share on X (Twitter)
