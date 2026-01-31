---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#30-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 134
summary: MVS starts at the main modules (special vertices in the graph that have no. version) and traverses the graph, tracking the highest required version of each
---


MVS starts at the main modules (special vertices in the graph that have no
version) and traverses the graph, tracking the highest required version of each
module. At the end of the traversal, the highest required versions comprise the
build list: they are the minimum versions that satisfy all requirements.

Unlike other dependency management systems, the build list is
not saved in a “lock” file. MVS is deterministic, and the build list doesn’t
change when new versions of dependencies are released, so MVS is used to compute
