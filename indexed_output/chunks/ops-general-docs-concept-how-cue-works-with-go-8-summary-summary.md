---
doc_id: ops/general/docs-concept-how-cue-works-with-go
chunk_id: ops/general/docs-concept-how-cue-works-with-go#8-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 131
summary: The Go API injects the power and expressiveness of CUE into your Go programs,. allowing them to
---


The Go API injects the power and expressiveness of CUE into your Go programs,
allowing them to
load and validate both CUE and non-CUE data (such as JSON or YAML),
and to
check data marshalled by Go, wherever it comes from.

LOADING CUE DATA

In this example, we load some data from the following CUE file and display it:

Copied!
file.cue

Copy code
Copied!

package example

l: [1, 2, 3]
v: "hello"
message: (v): "world!"

The cuelang.org/go/cue/load package provides a similar interface to the cue
command for loading CUE.
