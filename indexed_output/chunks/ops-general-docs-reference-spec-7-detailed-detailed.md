---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#7-detailed
chunk_level: detailed
chunk_type: table
heading: Introduction
token_count: 1031
summary: The string type represents the set of UTF-8 strings,. not allowing surrogates
---


STRINGS

The string type represents the set of UTF-8 strings,
not allowing surrogates.
The predeclared string type is string; it is a defined type.

The length of a string s (its size in bytes) can be discovered using
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
or just a.f or b.f if f is in just a or b, respectively.
Any references [/docs/reference/spec/#references] to a or b
in their respective field values need to be replaced with references to c.
The result of a unification is bottom (_|_) if any of its defined
fields evaluates to bottom, recursively.

A struct literal may contain multiple fields with the same label,
the result of which is the unification of all those fields.


Copy code
Copied!

StructLit       = "{" { Declaration "," } "}" .
Declaration     = Field | Ellipsis | Embedding | LetClause | attribute .
Ellipsis        = "..." [ Expression ] .
Embedding       = Comprehension | AliasExpr .
Field           = Label ":" { Label ":" } AliasExpr { attribute } .
Label           = [ identifier "=" ] LabelExpr .
LabelExpr       = LabelName [ "?" | "!" ] | "[" AliasExpr "]" .
LabelName       = identifier | simple_string_lit | "(" AliasExpr ")" .

attribute       = "@" identifier "(" attr_tokens ")" .
attr_tokens     = { attr_token |
                    "(" attr_tokens ")" |
                    "[" attr_tokens "]" |
                    "{" attr_tokens "}" } .
attr_token      = /* any token except '(', ')', '[', ']', '{', or '}' */


Copy code
Copied!

Expression                             Result
{a: int, a: 1}                         {a: 1}
{a: int} & {a: 1}                      {a: 1}
{a: >=1 & <=7} & {a: >=5 & <=9}        {a: >=5 & <=7}
{a: >=1 & <=7, a: >=5 & <=9}           {a: >=5 & <=7}

{a: 1} & {b: 2}                        {a: 1, b: 2}
{a: 1, b: int} & {b: 2}                {a: 1, b: 2}

{a: 1} & {a: 2}                        _|_


FIELD CONSTRAINTS

A struct may declare field constraints which define values
that should be unified with a given field once it is defined.
The existence of a field constraint declares, but does not define, that field.
