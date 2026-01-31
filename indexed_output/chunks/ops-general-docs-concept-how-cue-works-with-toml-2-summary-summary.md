---
doc_id: ops/general/docs-concept-how-cue-works-with-toml
chunk_id: ops/general/docs-concept-how-cue-works-with-toml#2-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 129
summary: TOML can be processed by CUE’s wide range of data, schema, and policy. validation capabilities
---

TOML can be processed by CUE’s wide range of data, schema, and policy
validation capabilities.
Data in any supported encoding can be read and exported as TOML
– as demonstrated here by
cue export [/docs/reference/command/cue-export/]
unifying its TOML, JSON, and CUE input files and producing TOML:

 * 
   
   Copied!
   a.toml
 * 
   
   Copied!
   b.json
 * 
   
   Copied!
   c.cue

Copy code
Copied!

a = "1"

[b]
c = 2.2

[b.d]
e = 3


Copy code
Copied!

{
    "f": "4",
    "g": 5.5
}


Copy code
Copied!
