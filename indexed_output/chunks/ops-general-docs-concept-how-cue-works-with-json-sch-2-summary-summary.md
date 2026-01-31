---
doc_id: ops/general/docs-concept-how-cue-works-with-json-sch
chunk_id: ops/general/docs-concept-how-cue-works-with-json-sch#2-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 134
summary: [https://github. com/myitcv]
---


[https://github.com/myitcv]

Search all content by this author

[/search/?q=author:myitcv]
 * encodings [/search?q=tag:encodings]
 * cue command [/search?q=tag:%22cue%20command%22]
 * go api [/search?q=tag:%22go%20api%22]

CUE has first class support for JSON Schema [https://json-schema.org/]:
both the cue command and the Go API understand the format.

Constraints stored as JSON Schema are available for cue commands to use as if
they were expressed in CUE.
This allows you to work with JSON Schema constraints directly, using them to
