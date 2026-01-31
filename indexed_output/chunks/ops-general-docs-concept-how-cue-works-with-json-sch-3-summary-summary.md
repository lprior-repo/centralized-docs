---
doc_id: ops/general/docs-concept-how-cue-works-with-json-sch
chunk_id: ops/general/docs-concept-how-cue-works-with-json-sch#3-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 135
summary: validate data, and to represent them natively in CUE’s more succinct and. expressive form
---

validate data, and to represent them natively in CUE’s more succinct and
expressive form.

In this guide we’ll see:

 * cue import [/docs/reference/command/cue-help-import/] converting a
   JSON Schema to CUE,
 * cue vet [/docs/reference/command/cue-help-vet/] using JSON Schema
   constraints directly,
 * and the
   encoding/jsonschema [https://pkg.go.dev/cuelang.org/go/encoding/jsonschema]
   Go API validating data against a JSON Schema.

The ability to export CUE constraints as JSON Schema is tracked in issue #929 [/issue/929].
