---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#146-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 133
summary: typically either the path of a builtin package or a fully qualifying location. of a package within a source code repository
---

typically either the path of a builtin package or a fully qualifying location
of a package within a source code repository.

An ImportLocation must be a non-empty string using only characters belonging to
Unicode’s L, M, N, P, and S general categories
(the Graphic characters without spaces)
and may not include the characters !"#$%&'()*,:;<=>?[\\]^`{|}
or the Unicode replacement character U+FFFD.

Assume we have package containing the package clause package math,
which exports function Sin at the path identified by lib/math.
