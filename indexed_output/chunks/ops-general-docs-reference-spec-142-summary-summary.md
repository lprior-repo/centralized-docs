---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#142-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 142
summary: \"Hello \(#place)!\". #place: \"world\"
---



Copy code
Copied!

"Hello \(#place)!"

#place: "world"

// Outputs "Hello world!"

PACKAGE CLAUSE

A package clause is an optional clause that defines the package to which
a source file the file belongs.


Copy code
Copied!

PackageClause  = "package" PackageName .
PackageName    = identifier .

The PackageName must not be a definition identifier.

If the PackageName is the blank identifier (_), it is treated the same
as if there were no package clause. This can be useful to allow adding
package level attributes or doc comments to a CUE file without a package
