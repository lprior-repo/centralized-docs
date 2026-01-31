---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#32-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 135
summary: containing the bolded versions: A 1.  Note that higher
---

containing the bolded versions: A 1.2, B 1.2, C 1.4, and D 1.2. Note that higher
versions of B and D are available but MVS does not select them, since nothing
requires them.

MODULE STORAGE FORMAT

Modules are stored in a registry using a standard manifest + blob
format. There is rarely any need to
interact directly with these artifacts, since the cue command creates, downloads,
and extracts them automatically from registries. However, it’s still useful to know about these
files to understand cross-platform compatibility constraints.
