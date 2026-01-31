---
doc_id: ops/general/docs-concept-how-cue-works-with-go
chunk_id: ops/general/docs-concept-how-cue-works-with-go#4-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 134
summary: Kubernetes project, import some types it defines, and use some of the CUE that. gets produced
---

Kubernetes project, import some types it defines, and use some of the CUE that
gets produced.

Let’s start by downloading a specific version of the k8s.io/api module:

TERMINAL

Copy code
Copied!

$ go get k8s.io/api/apps/v1@v0.29.3
...

We use cue get go to generate CUE definitions from the Go types in the k8s.io/api/apps/v1 package:

TERMINAL

Copy code
Copied!

$ cue get go k8s.io/api/apps/v1

This generates some CUE packages, placing them alongside our main CUE module:

TERMINAL

Copy code
Copied!

$ tree -d cue.mod/gen/k8s.io
