---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#14-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 141
summary: providing a distinct import path for each incompatible version. Unlike in Go [https://go
---

providing a distinct import path for each incompatible version.

Unlike in Go [https://go.dev/ref/mod#major-version-suffixes],
major version suffixes are always required in module paths . The burden
of changing import paths in packages is eased by allowing the
major version suffix to be omitted and inferred from the module.cue
file. See major version defaults [/docs/reference/modules/#major-version-defaults] for details.

Major version suffixes let multiple major versions of a module coexist in the
same build. This may be necessary due to a diamond dependency
