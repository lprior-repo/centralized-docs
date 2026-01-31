---
doc_id: ops/general/docs-concept-how-cue-works-with-json
chunk_id: ops/general/docs-concept-how-cue-works-with-json#2-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 130
summary: In other words: all valid JSON is CUE. The cue tool natively supports reading and writing JSON files
---

In other words: all valid JSON is CUE.

The cue tool natively supports reading and writing JSON files. In fact, JSON
is its default output format.

This allows JSON files to be processed by CUE’s wide range of data, schema, and
policy validation capabilities, and to convert input formats to JSON - as
demonstrated here by
cue export [/docs/reference/command/cue-help-export/]
unifying all its JSON, YAML, and CUE input files as JSON:

 * 
   
   Copied!
   data.json
 * 
   
   Copied!
   data.yml
 * 
   
   Copied!
