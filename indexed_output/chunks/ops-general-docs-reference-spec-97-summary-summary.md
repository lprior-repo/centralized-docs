---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#97-summary
chunk_level: summary
chunk_type: table
heading: Introduction
token_count: 137
summary: BasicLit    = int_lit | float_lit | string_lit |.               null_lit | bool_lit | bottom_lit 
---

BasicLit    = int_lit | float_lit | string_lit |
              null_lit | bool_lit | bottom_lit .
OperandName = identifier | QualifiedIdent .

QUALIFIED IDENTIFIERS

A qualified identifier is an identifier qualified with a package name prefix.


Copy code
Copied!

QualifiedIdent = PackageName "." identifier .

A qualified identifier accesses an identifier in a different package,
which must be imported [/docs/reference/spec/#import-declarations].
The identifier must be declared in the package block [/docs/reference/spec/#blocks] of that package.
