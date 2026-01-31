---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#40-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 137
summary:  * File and directory names within a module may consist of Unicode letters, ASCII.    digits, the ASCII space character (U+0020), and the ASCII punctuation
---

 * File and directory names within a module may consist of Unicode letters, ASCII
   digits, the ASCII space character (U+0020), and the ASCII punctuation
   characters !#$%&()+,-.=@[]^_{}~. Note that package paths may not contain all
   these characters. See
   module.CheckFilePath [https://pkg.go.dev/cuelang.org/go/internal/mod/module?tab=doc#CheckFilePath]
   and
   module.CheckImportPath [https://pkg.go.dev/golang.org/x/mod/module?tab=doc#CheckImportPath]
   for the differences.
 * A file or directory name up to the first dot must not be a
