---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#31-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 145
summary: it at the beginning of every module-aware command. Module version graph with visited versions highlighted [buildlist
---

it at the beginning of every module-aware command.

Module version graph with visited versions highlighted [buildlist.svg]
buildlist.svgModule version graph with visited versions highlightedConsider the example in this diagram. The main module requires module A
at version 1.2 or higher and module B at version 1.2 or higher. A 1.2 and B 1.2
require C 1.3 and C 1.4, respectively. C 1.3 and C 1.4 both require D 1.2.

MVS visits and loads the cue.mod/module.cue file for each of the module versions
highlighted in blue. At the end of the graph traversal, MVS returns a build list
