---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#22-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 518
summary: *null | List, defaulting to the shortest variant. For instance, the open list [ 1, 2, … ] can be represented as:
---

*null | List, defaulting to the shortest variant.
For instance, the open list [ 1, 2, … ] can be represented as:


Copy code
Copied!

open: List & { Elem: 1, Tail: { Elem: 2 } }

and the closed version of this list, [ 1, 2 ], as


Copy code
Copied!

closed: List & { Elem: 1, Tail: { Elem: 2, Tail: null } }

Using this representation, the subsumption rule for lists can
be derived from those of structs.
Implementations are not required to implement lists as structs.
The Elem and Tail fields are not special and len will not work as
expected in these cases.

DECLARATIONS AND SCOPES

BLOCKS

A block is a possibly empty sequence of declarations.
The braces of a struct literal { ... } form a block, but there are
others as well:

 * The universe block encompasses all CUE source text.
 * Each package [/docs/reference/spec/#modules-instances-and-packages] has a package block
   containing all CUE source text in that package.
 * Each file has a file block containing all CUE source text in that file.
 * Each for and let clause in a comprehension [/docs/reference/spec/#comprehensions]
   is considered to be its own implicit block.

Blocks nest and influence scoping.

DECLARATIONS AND SCOPE

A declaration may bind an identifier to a field, alias, or package.
Every identifier in a program must be declared.
Other than for fields,
no identifier may be declared twice within the same block.
For fields, an identifier may be declared more than once within the same block,
resulting in a field with a value that is the result of unifying the values
of all fields with the same identifier.
String labels do not bind an identifier to the respective field.

The scope of a declared identifier is the extent of source text in which the
identifier denotes the specified field, alias, or package.

CUE is lexically scoped using blocks:

 1. The scope of a predeclared identifier [/docs/reference/spec/#predeclared-identifiers] is the universe block.
 2. The scope of an identifier denoting a field
    declared at top level (outside any struct literal) is the package block.
