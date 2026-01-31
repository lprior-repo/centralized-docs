---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#11-detailed
chunk_level: detailed
chunk_type: prose
heading: Introduction
token_count: 1027
summary: ListLit       = \"[\" [ ElementList [ \",\" ] ] \"]\" . ElementList   = Ellipsis | Embedding { \",\" Embedding } [ \",\" Ellipsis ] 
---



Copy code
Copied!

ListLit       = "[" [ ElementList [ "," ] ] "]" .
ElementList   = Ellipsis | Embedding { "," Embedding } [ "," Ellipsis ] .

Lists can be thought of as structs:


Copy code
Copied!

List: *null | {
    Elem: _
    Tail: List
}

For closed lists, Tail is null for the last element, for open lists it is
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
 3. The scope of an identifier denoting an alias
    declared at top level (outside any struct literal) is the file block.
 4. The scope of a let identifier
    declared at top level (outside any struct literal) is the file block.
 5. The scope of the package name of an imported package is the file block of the
    file containing the import declaration.
 6. The scope of a field, alias or let identifier declared inside a struct
    literal is the innermost containing block.

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
of the predefined identifier, prefixed with __.


Copy code
Copied!

Functions
len close and or

Types
null      The null type and value
bool      All boolean values
int       All integral numbers
float     All decimal floating-point numbers
string    Any valid UTF-8 sequence
bytes     Any valid byte sequence

Derived   Value
number    int | float
uint      >=0
uint8     >=0 & <=255
int8      >=-128 & <=127
uint16    >=0 & <=65535
int16     >=-32_768 & <=32_767
rune      >=0 & <=0x10FFFF
uint32    >=0 & <=4_294_967_295
int32     >=-2_147_483_648 & <=2_147_483_647
uint64    >=0 & <=18_446_744_073_709_551_615
int64     >=-9_223_372_036_854_775_808 & <=9_223_372_036_854_775_807
