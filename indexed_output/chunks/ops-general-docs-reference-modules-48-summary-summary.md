---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#48-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 130
summary: minor version: The second number in a semantic version (2 in v1. a release with new, backwards compatible functionality, the minor version must
---


minor version: The second number in a semantic version (2 in v1.2.3). In
a release with new, backwards compatible functionality, the minor version must
be incremented, and the patch version must be set to 0.

module: A collection of packages that are released, versioned, and
distributed together.

module cache: A local directory storing downloaded modules, located in
$CUE_CACHE_DIR. See Module cache [/docs/reference/modules/#module-cache].

module graph: The directed graph of module requirements, rooted at the main
