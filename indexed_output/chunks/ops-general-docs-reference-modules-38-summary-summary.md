---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#38-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 134
summary:    empty directories in zip files it creates.  * Symbolic links and other irregular files are ignored when creating zip files,
---

   empty directories in zip files it creates.
 * Symbolic links and other irregular files are ignored when creating zip files,
   since they aren’t portable across operating systems and file systems, and
   there’s no portable way to represent them in the zip file format.
 * Files within directories containing cue.mod directories, other than the module
   root directory and the cue.mod directory itself, are ignored when creating zip files,
   since they are not part
   of the module. CUE ignores subdirectories containing cue.mod
