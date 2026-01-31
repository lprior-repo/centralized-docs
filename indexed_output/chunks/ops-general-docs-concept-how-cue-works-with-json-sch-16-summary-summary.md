---
doc_id: ops/general/docs-concept-how-cue-works-with-json-sch
chunk_id: ops/general/docs-concept-how-cue-works-with-json-sch#16-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 145
summary: FUTURE PLANS. One of CUE’s goals is to act as an interlingua: a bidirectional bridge
---


FUTURE PLANS

One of CUE’s goals is to act as an interlingua: a bidirectional bridge
between all the formats that CUE speaks, linking constraints and data sources
of truth, no matter where they exist.

To meet this goal, CUE will gain the ability to export native CUE constraints
as JSON Schema, enabling their use by tools that aren’t aware of CUE. This is
tracked in issue #929 [/issue/929].

RELATED CONTENT

 * Reference: cue help import [/docs/reference/command/cue-help-import/]
 * The encoding/jsonschema [https://pkg.go.dev/cuelang.org/go/encoding/jsonschema] Go API
