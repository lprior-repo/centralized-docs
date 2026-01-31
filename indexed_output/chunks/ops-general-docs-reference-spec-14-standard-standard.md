---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#14-standard
chunk_level: standard
chunk_type: table
heading: Introduction
token_count: 517
summary: the builtin function len. The bytes type represents the set of byte sequences
---

the builtin function len.

BYTES

The bytes type represents the set of byte sequences.
A byte sequence value is a (possibly empty) sequence of bytes.
The number of bytes is called the length of the byte sequence
and is never negative.
The predeclared byte sequence type is bytes; it is a defined type.

BOUNDS

A bound, syntactically a unary expression [/docs/reference/spec/#operands], defines
a logically infinite disjunction of concrete values represented as a single comparison.
For example, >= 2 represents the infinite disjunction 2|3|4|5|6|7|….

For any comparison operator [/docs/reference/spec/#comparison-operators] op,
op a is the disjunction of every x such that x op a.


Copy code
Copied!

2 & >=2 & <=5           // 2, where 2 is either an int or float.
2.5 & >=1 & <=5         // 2.5
2 & >=1.0 & <3.0        // 2.0
2 & >1 & <3.0           // 2.0
2.5 & int & >1 & <5     // _|_
2.5 & float & >1 & <5   // 2.5
int & 2 & >1.0 & <3.0   // _|_
2.5 & >=(int & 1) & <5  // _|_
>=0 & <=7 & >=3 & <=10  // >=3 & <=7
!=null & 1              // 1
==[1, 2] & [1]          // _|_
!=[1, 2] & [1]          // [1]

STRUCTS

A struct is a set of elements called fields, each of
which has a name, called a label, and value.

We say a label is defined for a struct if the struct has a field with the
corresponding label.
The value for a label f of struct a is denoted a.f.
A struct a is an instance of b, or a ⊑ b, if for any label f
defined for b, label f is also defined for a and a.f ⊑ b.f.
Note that if a is an instance of b it may have fields with labels that
are not defined for b.

The (unique) struct with no fields, written {}, has every struct as an
instance. It can be considered the type of all structs.


Copy code
Copied!

{a: 1} ⊑ {}
{a: 1, b: 1} ⊑ {a: 1}
{a: 1} ⊑ {a: int}
{a: 1, b: 1.0} ⊑ {a: int, b: number}

{} ⋢ {a: 1}
{a: 2} ⋢ {a: 1}
{a: 1} ⋢ {b: 1}

The successful unification of structs a and b is a new struct c which
has all fields of both a and b, where
the value of a field f in c is a.f & b.f if f is defined in both a and b,
