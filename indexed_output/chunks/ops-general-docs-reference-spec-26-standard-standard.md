---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#26-standard
chunk_level: standard
chunk_type: table
heading: Introduction
token_count: 512
summary: PrimaryExpr =. 	PrimaryExpr Selector |
---

Copied!

PrimaryExpr =
	Operand |
	PrimaryExpr Selector |
	PrimaryExpr Index |
	PrimaryExpr Arguments .

Selector       = "." (identifier | simple_string_lit) .
Index          = "[" Expression "]" .
Argument       = Expression .
Arguments      = "(" [ ( Argument { "," Argument } ) [ "," ] ] ")" .


Copy code
Copied!

x
2
(s + ".txt")
f(3.1415, true)
m["foo"]
obj.color
f.p[i].x

SELECTORS

For a primary expression [/docs/reference/spec/#primary-expressions] x that is not a package name [/docs/reference/spec/#package-clause],
the selector expression


Copy code
Copied!

x.f

denotes the element of a struct x identified by f.

f must be an identifier or a string literal identifying
any definition or regular non-optional field.
The identifier f is called the field selector.

If x is a package name, see the section on qualified identifiers [/docs/reference/spec/#qualified-identifiers].

Otherwise, if x is not a struct,
or if f does not exist in x,
the result of the expression is bottom (an error).
In the latter case the expression is incomplete.
The operand of a selector may be associated with a default.


Copy code
Copied!

T: {
    x:     int
    y:     3
    "x-y": 4
}

a: T.x     // int
b: T.y     // 3
c: T.z     // _|_ // field 'z' not found in T
d: T."x-y" // 4

e: {a: 1|*2} | *{a: 3|*4}
f: e.a  // 4 (default value)

INDEX EXPRESSIONS

A primary expression of the form


Copy code
Copied!

a[x]

denotes the element of a list or struct a indexed by x.
The value x is called the index or field name, respectively.
The following rules apply:

If a is not a struct:

 * a is a list (which need not be complete)
 * the index x unified with int must be concrete.
 * the index x is in range if 0 <= x < len(a), where only the
   explicitly defined values of an open-ended list are considered,
   otherwise it is out of range

The result of a[x] is

for a of list type:

 * the list element at index x, if x is within range
 * bottom (an error), otherwise

for a of struct type:

 * the index x unified with string must be concrete.
