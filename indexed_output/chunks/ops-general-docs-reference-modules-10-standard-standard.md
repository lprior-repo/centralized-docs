---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#10-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 518
summary:    in module zip files but are not extracted.  The cue command does not include
---

   in module zip files but are not extracted. The cue command does not include
   empty directories in zip files it creates.
 * Symbolic links and other irregular files are ignored when creating zip files,
   since they aren’t portable across operating systems and file systems, and
   there’s no portable way to represent them in the zip file format.
 * Files within directories containing cue.mod directories, other than the module
   root directory and the cue.mod directory itself, are ignored when creating zip files,
   since they are not part
   of the module. CUE ignores subdirectories containing cue.mod
   directories when extracting zip files.
 * No two files within a zip file may have paths equal under Unicode case-folding
   (see strings.EqualFold [https://pkg.go.dev/strings?tab=doc#EqualFold]).
   This ensures that zip files can be extracted on case-insensitive file systems
   without collisions.
 * A cue.mod/module.cue file must appear in the top-level directory.
   If present, it must have the name cue.mod/module.cue (all
   lowercase). Directories named cue.mod are not allowed in any other directory.
 * File and directory names within a module may consist of Unicode letters, ASCII
   digits, the ASCII space character (U+0020), and the ASCII punctuation
   characters !#$%&()+,-.=@[]^_{}~. Note that package paths may not contain all
   these characters. See
   module.CheckFilePath [https://pkg.go.dev/cuelang.org/go/internal/mod/module?tab=doc#CheckFilePath]
   and
   module.CheckImportPath [https://pkg.go.dev/golang.org/x/mod/module?tab=doc#CheckImportPath]
   for the differences.
 * A file or directory name up to the first dot must not be a
   reserved file name on Windows [https://learn.microsoft.com/en-us/windows/win32/fileio/naming-a-file#naming-conventions],
   regardless of case (CON, com1, NuL, and so on).

MODULE CACHING

By default, the cue command caches downloaded modules in the local
filesystem. It uses the local user configuration directory by default, but
that can be changed by setting $CUE_CACHE_DIR, which is
