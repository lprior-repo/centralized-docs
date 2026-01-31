---
doc_id: ops/general/docs-concept-how-cue-works-with-json
chunk_id: ops/general/docs-concept-how-cue-works-with-json#10-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 128
summary: @extern(embed). _conf: _ @embed(file=config
---

Copied!
example.cue

Copy code
Copied!

@extern(embed)

package p

_conf: _ @embed(file=config.json)
_data: _ @embed(glob=env/*.json)

info: {
	version: _conf.version
	source:  _data["env/\(_conf.source).json"].text
}

 * 
   
   Copied!
   config.json
 * 
   
   Copied!
   env/foo.json
 * 
   
   Copied!
   env/bar.json

Copy code
Copied!

{
    "version": "1.42.0",
    "source": "bar"
}


Copy code
Copied!

{
    "text": "Some foo"
}


Copy code
Copied!

{
    "text": "A bar"
}

TERMINAL

Copy code
Copied!
