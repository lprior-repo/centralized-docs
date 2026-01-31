---
doc_id: ops/general/docs-concept-how-cue-works-with-yaml
chunk_id: ops/general/docs-concept-how-cue-works-with-yaml#9-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 128
summary: This provides an alternative way to use CUE to validate data files against. schemas and constraints, and also gives CUE configurations access to data
---

This provides an alternative way to use CUE to validate data files against
schemas and constraints, and also gives CUE configurations access to data
stored in non-CUE files:

Copied!
example.cue

Copy code
Copied!

@extern(embed)

package p

_conf: _ @embed(file=config.yaml)
_data: _ @embed(glob=env/*.yml)

info: {
	version: _conf.version
	source:  _data["env/\(_conf.source).yml"].text
}

 * 
   
   Copied!
   config.yaml
 * 
   
   Copied!
   env/foo.yml
 * 
   
   Copied!
   env/bar.yml

Copy code
Copied!
