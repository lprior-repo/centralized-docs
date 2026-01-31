---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#22-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 128
summary: all modules, such as github. com/foo/bar will be fetched from
---


all modules, such as github.com/foo/bar will be fetched from
public-registry.example with the exception of modules with the
prefix github.com/acmecorp/, such as github.com/acmecorp/somemodule
which will be fetched from the modules/github.com/acmecorp/somemodule repository
in the host registry.acme.example at port 6000.

CUE.MOD/MODULE.CUE FILES

A module is defined by a cue.mod directory in its root containing
a module.cue CUE file.


Copy code
Copied!

// module indicates the module's path.
module!: #Module
