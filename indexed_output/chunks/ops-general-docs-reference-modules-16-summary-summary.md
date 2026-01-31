---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#16-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 139
summary: same subdirectory relative to their module roots—are distinct. MAJOR VERSION DEFAULTS
---

same subdirectory relative to their module roots—are distinct.

MAJOR VERSION DEFAULTS

When a package import path does not contain a major version,
the module.cue file is consulted to determine which major
version of the module to use. In a canonical module.cue file,
all imports without major versions will have an explicit default: true
present in the corresponding dependency entry, but cue mod tidy
will add those if not present and there is no ambiguity in the build list.

That is, given only a single major version of a module in the build list,
