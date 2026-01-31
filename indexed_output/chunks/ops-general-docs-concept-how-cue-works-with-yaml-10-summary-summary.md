---
doc_id: ops/general/docs-concept-how-cue-works-with-yaml
chunk_id: ops/general/docs-concept-how-cue-works-with-yaml#10-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 140
summary: source: \"bar\". text: \"Some foo\"
---


version: 1.42.0
source: "bar"


Copy code
Copied!

text: "Some foo"


Copy code
Copied!

text: "A bar"

TERMINAL

Copy code
Copied!

$ cue export --out yaml
info:
  version: 1.42.0
  source: A bar

File embedding is available from CUE v0.12.0 onwards.
Find out more about this powerful validation feature in
Embedding files in a CUE evaluation [/docs/howto/embed-files-in-cue-evaluation/].

ENCODING YAML INSIDE CUE

CUE is frequently used to generate configuration files. Some systems allow
their configuration files to contain YAML encoded in string fields,
