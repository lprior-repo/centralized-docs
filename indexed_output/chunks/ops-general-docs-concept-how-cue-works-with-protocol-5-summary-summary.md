---
doc_id: ops/general/docs-concept-how-cue-works-with-protocol
chunk_id: ops/general/docs-concept-how-cue-works-with-protocol#5-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 131
summary: unify each field’s right-hand-side. The behaviour of the cue import command can be affected by the flags outlined
---

unify each field’s right-hand-side.

The behaviour of the cue import command can be affected by the flags outlined
in the proto mode section of
the command’s help text [/docs/reference/command/cue-help-import/].

USING THE GO API

CUE’s Go API can achieve the same result as the cue import command,
converting Protobuf definitions to CUE, but with
more customization and flexibility [https://pkg.go.dev/cuelang.org/go/encoding/protobuf#Config].

This simple Go code takes the basic.proto file shown above, and prints the
