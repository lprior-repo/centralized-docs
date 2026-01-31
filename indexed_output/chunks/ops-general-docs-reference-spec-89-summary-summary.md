---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#89-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 137
summary: An identifier declared in a block may be redeclared in an inner block. While the identifier of the inner declaration is in scope, it denotes the entity
---


An identifier declared in a block may be redeclared in an inner block.
While the identifier of the inner declaration is in scope, it denotes the entity
declared by the inner declaration.

The package clause is not a declaration;
the package name does not appear in any scope.
Its purpose is to identify the files belonging to the same package
and to specify the default name for import declarations.

PREDECLARED IDENTIFIERS

CUE predefines a set of types and builtin functions.
For each of these there is a corresponding keyword which is the name
