---
doc_id: ops/general/docs-concept-how-cue-works-with-yaml
chunk_id: ops/general/docs-concept-how-cue-works-with-yaml#11-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 128
summary: irrespective of the file’s main data format. CUE’s standard library provides
---

irrespective of the file’s main data format.

CUE’s standard library provides
a built-in yaml package [https://pkg.go.dev/cuelang.org/go/pkg/encoding/yaml]
containing functions that generate, parse, and validate YAML from
within CUE - some of which are shown here.

GENERATING ENCODED YAML

In this example a Kubernetes ConfigMap contains a YAML file encoded as a
single string field, in a JSON file. This is enabled by the
yaml.Marshal function:

Copied!
config.cue

Copy code
Copied!

import "encoding/yaml"
