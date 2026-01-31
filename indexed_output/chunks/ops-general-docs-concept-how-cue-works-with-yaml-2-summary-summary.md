---
doc_id: ops/general/docs-concept-how-cue-works-with-yaml
chunk_id: ops/general/docs-concept-how-cue-works-with-yaml#2-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 130
summary: those containing multiple documents. This allows YAML files to be processed by CUE’s wide range of data, schema, and
---

those containing multiple documents.

This allows YAML files to be processed by CUE’s wide range of data, schema, and
policy validation capabilities, and to convert input formats to YAML - as
demonstrated here by
cue export [/docs/reference/command/cue-help-export/]
unifying all its YAML, JSON, and CUE input files as YAML:

 * 
   
   Copied!
   data.yml
 * 
   
   Copied!
   data.json
 * 
   
   Copied!
   data.cue

Copy code
Copied!

a: 1
b: "2"
c: "three"
d: 4.4


Copy code
Copied!

{
    "e": 5,
    "f": "6"
