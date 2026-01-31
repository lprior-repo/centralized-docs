---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#5-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 142
summary: relative to the module root. MODULE PATHS
---

relative to the module root.

MODULE PATHS

A module path is the canonical name for a module, declared with the
module field in the module’s cue.mod/module.cue file. A module’s
path is the prefix for package paths within the module.

A module path consists of a root path and a major version suffix,
for example in the module path myhost.example/foo@v0, the root path is
myhost.example/foo and the major version suffix is @v0.

Module paths are domain-name qualified: a module path always begins
with a host name, although that host is only a guide to the origin of
