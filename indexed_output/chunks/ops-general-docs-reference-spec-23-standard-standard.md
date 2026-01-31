---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#23-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 523
summary:  The scope of an identifier denoting an alias.     declared at top level (outside any struct literal) is the file block
---

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
uint128   >=0 & <=340_282_366_920_938_463_463_374_607_431_768_211_455
int128    >=-170_141_183_460_469_231_731_687_303_715_884_105_728 &
           <=170_141_183_460_469_231_731_687_303_715_884_105_727
float32   >=-3.40282346638528859811704183484516925440e+38 &
          <=3.40282346638528859811704183484516925440e+38
float64   >=-1.797693134862315708145274237317043567981e+308 &
