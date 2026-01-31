---
doc_id: ops/general/docs-tour-packages
chunk_id: ops/general/docs-tour-packages#1-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 135
summary:  Tour [https://cuelang. org/docs/tour/]
---


Menu

 1. Tour [https://cuelang.org/docs/tour/]
 2. Packages and Imports [https://cuelang.org/docs/tour/packages/]


 3. PACKAGES

By default, each CUE file is a standalone file.

A configuration can be split across multiple files by adding a package clause
to each file.

The configuration for each package is defined by the concatenation of all its files,
after stripping their package clauses and merging their import statements.
Multiple definitions of the same field across files and within the same file
are treated similarly, because
