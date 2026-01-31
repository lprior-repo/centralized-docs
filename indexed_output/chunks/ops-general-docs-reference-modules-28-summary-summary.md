---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#28-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 138
summary: cue file is designed to be human readable and machine writable. cue command will provide several subcommands that manipulate cue
---

}

The module.cue file is designed to be human readable and machine writable. The
cue command will provide several subcommands that manipulate cue.mod/module.cue files.
For now, the only one is cue mod tidy which will fetch dependencies
and canonicalize the module.cue file to reflect all the most recent versions.

A cue.mod/module.cue file is required for all modules.

MINIMAL VERSION SELECTION (MVS)

CUE uses an algorithm called Minimal version selection (MVS) to select
a set of module versions to use when building packages. MVS is described in
