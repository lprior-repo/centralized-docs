---
doc_id: ops/general/docs-concept-how-cue-works-with-json-sch
chunk_id: ops/general/docs-concept-how-cue-works-with-json-sch#10-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 133
summary: The cue command normally recognises JSON Schema’s signature fields and treats. the contents of JSON Schema as data constraints - not just additional data
---

    ./schema.json:13:13

The cue command normally recognises JSON Schema’s signature fields and treats
the contents of JSON Schema as data constraints - not just additional data.
A qualifier can be used to change this behaviour, as outlined in
cue help filetypes [/docs/reference/command/cue-help-filetypes/]:

TERMINAL

Copy code
Copied!

$ cue def json: schema.json
$schema:     "https://json-schema.org/draft/2020-12/schema"
type:        "object"
title:       "Main Person schema."
description: "This schema defines a person."
