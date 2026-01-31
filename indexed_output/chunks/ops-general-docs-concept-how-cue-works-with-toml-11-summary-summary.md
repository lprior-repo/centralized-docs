---
doc_id: ops/general/docs-concept-how-cue-works-with-toml
chunk_id: ops/general/docs-concept-how-cue-works-with-toml#11-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 137
summary: CUE is frequently used to generate configuration files.  Some systems allow
---

CUE is frequently used to generate configuration files. Some systems allow
their configuration files to contain TOML encoded in string fields,
irrespective of the file’s main data format.

CUE’s standard library provides
a built-in toml package [https://pkg.go.dev/cuelang.org/go/pkg/encoding/toml]
containing functions that generate and parse TOML from within CUE, as shown here.

GENERATING ENCODED TOML

In this example a Kubernetes ConfigMap contains a TOML file encoded as a
single string field, in a YAML document. This is enabled by the
