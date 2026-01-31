---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#29-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 130
summary: detail in Minimal Version Selection [https://research. com/vgo-mvs] by
---

detail in Minimal Version Selection [https://research.swtch.com/vgo-mvs] by
Russ Cox.

Conceptually, MVS operates on a directed graph of modules, specified with
module.cue files [/docs/reference/modules/#glos-cue-mod-file]. Each vertex in the graph represents a
module version. Each edge represents a minimum required version of a dependency,
specified with an entry in the deps field.

MVS produces the build list [/docs/reference/modules/#glos-build-list] as output, the list of module
versions used for an evaluation.
