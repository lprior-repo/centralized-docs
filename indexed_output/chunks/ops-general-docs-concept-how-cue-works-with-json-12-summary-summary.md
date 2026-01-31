---
doc_id: ops/general/docs-concept-how-cue-works-with-json
chunk_id: ops/general/docs-concept-how-cue-works-with-json#12-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 128
summary: a built-in json package [https://pkg. dev/cuelang
---

a built-in json package [https://pkg.go.dev/cuelang.org/go/pkg/encoding/json]
containing functions that generate, parse, validate, and format JSON from
within CUE - some of which are shown here.

GENERATING ENCODED JSON

In this example a Kubernetes ConfigMap contains a JSON file encoded as a
single string field, in a YAML document. This is enabled by the
json.Marshal function:

Copied!
config.cue

Copy code
Copied!

import "encoding/json"

configMap: data: "point.json": json.Marshal({
	x: 1.2
	y: 3.45
})

