---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#12-detailed
chunk_level: detailed
chunk_type: table
heading: Introduction
token_count: 1036
summary: uint128   >=0 & <=340_282_366_920_938_463_463_374_607_431_768_211_455. int128    >=-170_141_183_460_469_231_731_687_303_715_884_105_728 &
---

uint128   >=0 & <=340_282_366_920_938_463_463_374_607_431_768_211_455
int128    >=-170_141_183_460_469_231_731_687_303_715_884_105_728 &
           <=170_141_183_460_469_231_731_687_303_715_884_105_727
float32   >=-3.40282346638528859811704183484516925440e+38 &
          <=3.40282346638528859811704183484516925440e+38
float64   >=-1.797693134862315708145274237317043567981e+308 &
          <=1.797693134862315708145274237317043567981e+308

EXPORTED IDENTIFIERS

An identifier of a package may be exported to permit access to it
from another package.
All identifiers not starting with _ (so all regular fields and definitions
starting with #) are exported.
Any identifier starting with _ is not visible outside the package and resides
in a separate namespace than namesake identifiers of other packages.


Copy code
Copied!

package mypackage

foo:   string  // visible outside mypackage
"bar": string  // visible outside mypackage

#Foo: {      // visible outside mypackage
    a:  1    // visible outside mypackage
    _b: 2    // not visible outside mypackage

    #C: {    // visible outside mypackage
        d: 4 // visible outside mypackage
    }
    _#E: foo // not visible outside mypackage
}

UNIQUENESS OF IDENTIFIERS

Given a set of identifiers, an identifier is called unique if it is different
from every other in the set, after applying normalization following
Unicode Annex #31 [https://unicode.org/reports/tr31/].
Two identifiers are different if they are spelled differently
or if they appear in different packages and are not exported.
Otherwise, they are the same.

FIELD DECLARATIONS

A field associates the value of an expression to a label within a struct.
If this label is an identifier, it binds the field to that identifier,
so the field’s value can be referenced by writing the identifier.
String labels are not bound to fields.


Copy code
Copied!

a: {
    b: 2
    "s": 3

    c: b   // 2
    d: s   // _|_ unresolved identifier "s"
    e: a.s // 3
}

If an expression may result in a value associated with a default value
as described in default values [/docs/reference/spec/#default-values], the field binds to this
value-default pair.

LET DECLARATIONS

Within a struct, a let clause binds an identifier to the given expression.

Within the scope of the identifier, the identifier refers to the
locally declared expression.
The expression is evaluated in the scope it was declared.

EXPRESSIONS

An expression specifies the computation of a value by applying operators and
builtin functions to operands.

Expressions that require concrete values are called incomplete if any of
their operands are not concrete, but define a value that would be legal for
that expression.
Incomplete expressions may be left unevaluated until a concrete value is
requested at the application level.

OPERANDS

Operands denote the elementary values in an expression.
An operand may be a literal, a (possibly qualified) identifier denoting
a field, alias, or let declaration, or a parenthesized expression.


Copy code
Copied!

Operand     = Literal | OperandName | "(" Expression ")" .
Literal     = BasicLit | ListLit | StructLit .
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


Copy code
Copied!

math.Sin    // denotes the Sin function in package math

REFERENCES

An identifier operand refers to a field and is called a reference.
The value of a reference is a copy of the expression associated with the field
that it is bound to,
with any references within that expression bound to the respective copies of
the fields they were originally bound to.
Implementations may use a different mechanism to evaluate as long as
