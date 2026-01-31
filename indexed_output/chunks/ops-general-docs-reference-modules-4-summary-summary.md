---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#4-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 132
summary: A module is identified by a module path, which is declared in a. cue file, together with information about the module’s
---


A module is identified by a module path, which is declared in a
cue.mod/module.cue file, together with information about the module’s
dependencies. The module root directory is the directory that contains
the cue.mod directory. The main module is the module containing the
directory where the cue command is invoked.

Each package within a module is a collection of source files that are
unified together, usually all in the same directory. A package path
is the module path joined with the subdirectory containing the package,
