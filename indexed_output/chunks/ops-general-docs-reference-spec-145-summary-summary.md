---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#145-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 143
summary: ImportSpec       = [ PackageName ] ImportPath . ImportLocation   = { unicode_value } 
---

ImportSpec       = [ PackageName ] ImportPath .
ImportLocation   = { unicode_value } .
ImportPath       = `"` ImportLocation [ ":" identifier ] `"` .

The PackageName is used in qualified identifiers to access
exported identifiers of the package within the importing source file.
It is declared in the file block.
It defaults to the identifier specified in the package clause of the imported
package, which must match either the last path component of ImportLocation
or the identifier following it.

The interpretation of the ImportPath is implementation-dependent but it is
