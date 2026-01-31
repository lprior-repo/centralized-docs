---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#144-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 137
summary: For example, an instance may be defined as the subset of package files. belonging to a directory and all its ancestors
---

For example, an instance may be defined as the subset of package files
belonging to a directory and all its ancestors.

IMPORT DECLARATIONS

An import declaration states that the source file containing the declaration
depends on definitions of the imported package
and enables access to exported identifiers of that package.
The import names an identifier (PackageName) to be used for access and an
ImportPath that specifies the package to be imported.


Copy code
Copied!

ImportDecl       = "import" ( ImportSpec | "(" { ImportSpec "," } ")" ) .
