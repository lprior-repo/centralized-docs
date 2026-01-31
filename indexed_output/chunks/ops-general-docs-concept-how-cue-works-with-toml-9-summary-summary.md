---
doc_id: ops/general/docs-concept-how-cue-works-with-toml
chunk_id: ops/general/docs-concept-how-cue-works-with-toml#9-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 128
summary: feature allows data files (including TOML) to be read when some CUE is evaluated. This provides an alternative way to use CUE to validate data files against
---

feature allows data files (including TOML) to be read when some CUE is evaluated.
This provides an alternative way to use CUE to validate data files against
schemas and constraints, and also gives CUE configurations access to data
stored in non-CUE files:

Copied!
example.cue

Copy code
Copied!

@extern(embed)

package p

_conf: _ @embed(file=config.toml)
_data: _ @embed(glob=env/*.toml)

info: {
	version: _conf.version
	source:  _data["env/\(_conf.source).toml"].text
}

 * 
   
   Copied!
   config.toml
 * 
