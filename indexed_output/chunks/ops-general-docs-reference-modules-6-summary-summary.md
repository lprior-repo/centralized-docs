---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#6-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 135
summary: the module and is not used directly to fetch the module’s contents (see. here [/docs/reference/modules/#cue-registry-env] for details about that))
---

the module and is not used directly to fetch the module’s contents (see
here [/docs/reference/modules/#cue-registry-env] for details about that)). The expectation is
that any modules you create should have names that are inside domains
or namespaces that you have control of, enabling modules from different
creators to live together without conflicts in the same registry.

 * The root path is the
   portion of the module path that identifies the OCI repository within
   a registry. All versions of a module are located in that same OCI
