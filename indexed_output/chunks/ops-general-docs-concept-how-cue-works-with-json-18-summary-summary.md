---
doc_id: ops/general/docs-concept-how-cue-works-with-json
chunk_id: ops/general/docs-concept-how-cue-works-with-json#18-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 145
summary: CONVERTING JSON FILES TO CUE. every valid JSON file is also a CUE file [/docs/tour/basics/json-superset/],
---


CONVERTING JSON FILES TO CUE

Because
every valid JSON file is also a CUE file [/docs/tour/basics/json-superset/],
one very easy way to convert JSON files to CUE is simply to rename them from
.json to .cue!

In more complex situations
cue import [/docs/reference/command/cue-help-import/]
can create a CUE file for each JSON file it’s given, and can even recognise
encoded YAML and JSON fields, and convert those structures recursively.

Examples of this command being used can be found in the
cue import CLI reference documentation [/docs/reference/command/cue-help-import/].
