---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#39-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 128
summary:    directories when extracting zip files.  * No two files within a zip file may have paths equal under Unicode case-folding
---

   directories when extracting zip files.
 * No two files within a zip file may have paths equal under Unicode case-folding
   (see strings.EqualFold [https://pkg.go.dev/strings?tab=doc#EqualFold]).
   This ensures that zip files can be extracted on case-insensitive file systems
   without collisions.
 * A cue.mod/module.cue file must appear in the top-level directory.
   If present, it must have the name cue.mod/module.cue (all
   lowercase). Directories named cue.mod are not allowed in any other directory.
