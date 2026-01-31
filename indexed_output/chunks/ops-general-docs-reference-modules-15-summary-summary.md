---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#15-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 138
summary: problem [https://research. com/vgo-import#dependency_story]
---

problem [https://research.swtch.com/vgo-import#dependency_story]. Ordinarily, if
a module is required at two different versions by transitive dependencies, the
higher version will be used. However, if the two versions are incompatible,
neither version will satisfy all clients. Since incompatible versions must have
different major version numbers, they must also have different module paths due
to major version suffixes. This resolves the conflict: modules with distinct
suffixes are treated as separate modules, and their packages—even packages in
