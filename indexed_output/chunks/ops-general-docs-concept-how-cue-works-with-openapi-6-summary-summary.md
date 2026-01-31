---
doc_id: ops/general/docs-concept-how-cue-works-with-openapi
chunk_id: ops/general/docs-concept-how-cue-works-with-openapi#6-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 134
summary: unless the field is specified explicitly.  The same goes for OpenAPI’s
---

unless the field is specified explicitly. The same goes for OpenAPI’s
info.version field, which is extracted from CUE’s top-level $version field
if info.version isn’t present in the CUE.

Running cue def normalizes our CUE constraints, converting them into the
format specified by the --out parameter.
Be aware of just how long an equivalent OpenAPI definition can become - not
all formats possess CUE’s succinctness and economy of expression!

TERMINAL

Copy code
Copied!

$ cue def schema.cue -o api.pet.yaml --out openapi+yaml
